use crate::contract::{CELESTIA_VALIDATOR_PREFIX, IBC_TIMEOUT};
use crate::error::{ContractError, ContractResult};
use crate::helpers::{
    compute_mint_amount, compute_unbond_amount, derive_intermediate_sender, get_redemption_rate,
    paginate_map, validate_address, validate_addresses,
};
use crate::oracle::{Oracle, RedemptionRateAttributes, ORACLE_REDEMPTION_RATE_KEY, PurchaseRateAttributes, ORACLE_PURCHASE_RATE_KEY};
use crate::state::{
    ibc::{IBCTransfer, PacketLifecycleStatus},
    Config, IbcWaitingForReply, MultisigAddressConfig, ProtocolFeeConfig, State, ADMIN, BATCHES,
    CONFIG, IBC_WAITING_FOR_REPLY, INFLIGHT_PACKETS, PENDING_BATCH_ID, STATE,
};
use crate::state::{new_unstake_request, remove_unstake_request, unstake_requests, UnstakeRequest};
use cosmwasm_std::{
    ensure, Addr, Binary, CosmosMsg, Deps, DepsMut, Env, IbcTimeout, MessageInfo, Order, ReplyOn,
    Response, SubMsg, SubMsgResponse, SubMsgResult, Timestamp, Uint128,
};
use cw_utils::PaymentError;
use milky_way::staking::{Batch, BatchStatus};
use osmosis_std::types::cosmos::bank::v1beta1::MsgSend;
use osmosis_std::types::cosmos::base::v1beta1::Coin;
use osmosis_std::types::cosmwasm::wasm::v1::MsgExecuteContract;
use osmosis_std::types::ibc::applications::transfer::v1::MsgTransfer;
use osmosis_std::types::ibc::applications::transfer::v1::MsgTransferResponse;
use osmosis_std::types::osmosis::tokenfactory::v1beta1::{MsgBurn, MsgMint};
use prost::Message;
use crate::msg::MigrateMsg;

pub fn transfer_stake_msg(
    deps: &Deps,
    env: &Env,
    amount: Uint128,
) -> Result<MsgTransfer, ContractError> {
    let config: Config = CONFIG.load(deps.storage)?;

    if config.ibc_channel_id.is_empty() {
        return Err(ContractError::IbcChannelNotFound {});
    }

    let ibc_coin = Coin {
        denom: config.native_token_denom,
        amount: amount.to_string(),
    };

    let timeout = IbcTimeout::with_timestamp(Timestamp::from_nanos(
        env.block.time.nanos() + IBC_TIMEOUT.nanos(),
    ));

    let to_address = config.multisig_address_config.staker_address.to_string();
    let ibc_msg = MsgTransfer {
        source_channel: config.ibc_channel_id,
        source_port: "transfer".to_string(),
        token: Some(ibc_coin),
        receiver: to_address.clone(),
        sender: env.contract.address.to_string(),
        timeout_height: None,
        timeout_timestamp: timeout.timestamp().unwrap().nanos(),
        memo: format!(
            "{{\"ibc_callback\":\"{}\"}}",
            env.contract.address.to_string()
        ),
    };

    Ok(ibc_msg)
}

fn transfer_stake_sub_msg(
    deps: &mut DepsMut,
    env: &Env,
    amount: Uint128,
    sub_msg_id: Option<u64>,
) -> Result<SubMsg, ContractError> {
    let ibc_msg = transfer_stake_msg(&deps.as_ref(), env, amount)?;
    let sub_msg_id = sub_msg_id.unwrap_or({
        match env.transaction {
            Some(ref tx) => tx.index as u64 + env.block.time.nanos(),
            None => env.block.time.nanos(),
        }
    });

    let ibc_waiting_for_reply = IbcWaitingForReply {
        amount: amount.into(),
    };

    save_ibc_waiting_for_reply(deps, sub_msg_id, ibc_waiting_for_reply)?;

    Ok(SubMsg {
        id: sub_msg_id,
        msg: ibc_msg.into(),
        gas_limit: None,
        reply_on: ReplyOn::Always,
    })
}

fn update_oracle_msgs(deps: Deps, env: Env, config: &Config) -> Result<Vec<CosmosMsg>, ContractError> {
    let redemption_rate = get_redemption_rate(&deps);
    let mut messages: Vec<CosmosMsg> = Vec::new();
    let update_redemption_rate_execute_msg = Oracle::PostMetric {
        key: ORACLE_REDEMPTION_RATE_KEY.to_string(),
        value: redemption_rate.to_string(),
        metric_type: crate::oracle::MetricType::RedemptionRate,
        update_time: env.block.time.seconds(),
        block_height: env.block.height,
        attributes: Some(Binary::from(
            serde_json::to_string(&RedemptionRateAttributes {
                sttoken_denom: config.liquid_stake_token_denom.clone(),
            })
            .unwrap()
            .into_bytes(),
        )),
    };

    let update_purchase_rate_execute_msg = Oracle::PostMetric {
        key: ORACLE_PURCHASE_RATE_KEY.to_string(),
        value: redemption_rate.to_string(),
        metric_type: crate::oracle::MetricType::PurchaseRate,
        update_time: env.block.time.seconds(),
        block_height: env.block.height,
        attributes: Some(Binary::from(
            serde_json::to_string(&PurchaseRateAttributes {
                sttoken_denom: config.liquid_stake_token_denom.clone(),
            })
            .unwrap()
            .into_bytes(),
        )),
    };
    
    let update_redemption_rate_msg_json = serde_json::to_string(&update_redemption_rate_execute_msg).unwrap();
    messages.push(MsgExecuteContract {
        sender: env.contract.address.to_string(),
        contract: config.oracle_contract_address.clone().unwrap().to_string(),
        msg: update_redemption_rate_msg_json.as_bytes().to_vec(),
        funds: vec![]
    }.into());
    messages.push(MsgExecuteContract {
        sender: env.contract.address.to_string(),
        contract: config.oracle_contract_address_v2.clone().unwrap().to_string(),
        msg: update_redemption_rate_msg_json.as_bytes().to_vec(),
        funds: vec![]
    }.into());

    // purchase rate for the new contract only
    let update_purchase_rate_msg_json = serde_json::to_string(&update_purchase_rate_execute_msg).unwrap();
    messages.push(MsgExecuteContract {
        sender: env.contract.address.to_string(),
        contract: config.oracle_contract_address_v2.clone().unwrap().to_string(),
        msg: update_purchase_rate_msg_json.as_bytes().to_vec(),
        funds: vec![]
    }.into());

    Ok(messages)
}

pub fn check_stopped(config: &Config) -> Result<(), ContractError> {
    if config.stopped {
        return Err(ContractError::Halted {});
    }
    Ok(())
}

// PENDING
// Payment validation handled by caller (not sure what this means)
// Denom validation handled by caller (done in contract.rs)
pub fn execute_liquid_stake(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    amount: Uint128,
    mint_to: Option<String>,
    expected_mint_amount: Option<Uint128>,
) -> ContractResult<Response> {
    let config = CONFIG.load(deps.storage)?;

    check_stopped(&config)?;

    // a native user address is 43 chars long
    if mint_to.is_none() && info.sender.as_str().len() != 43 {
        return Err(ContractError::MissingMintAddress {});
    }

    // if sent via IBC or the sender is a contract the user needs to provide an osmosis address to mint to
    let mint_to_address = if mint_to.is_some() && info.sender.as_str().len() != 43 {
        let mint_to_addr = mint_to.unwrap();
        validate_address(&mint_to_addr, "osmo")?;

        mint_to_addr
    } else {
        info.sender.to_string()
    };

    let mut state: State = STATE.load(deps.storage)?;
    ensure!(
        amount >= config.minimum_liquid_stake_amount,
        ContractError::MinimumLiquidStakeAmount {
            minimum_stake_amount: (config.minimum_liquid_stake_amount),
            sent_amount: (amount)
        }
    );

    // this handles a special case that through slashing and redeeming chaining we get into a state
    // where the total liquid stake is zero but the total native stake is not
    // nobody can claim the native stake, so we need to claim it to the DAO
    if state.total_liquid_stake_token.is_zero() && !state.total_native_token.is_zero() {
        state.total_fees += state.total_native_token;
        state.total_native_token = Uint128::zero();
    }

    // Compute mint amount
    let mint_amount = compute_mint_amount(
        state.total_native_token,
        state.total_liquid_stake_token,
        amount,
    );
    // If mint amount is zero it is likely there was a an issue with rounding, return error and do not mint
    if mint_amount.is_zero() {
        return Err(ContractError::MintError {});
    }
    if let Some(expected_mint_amount) = expected_mint_amount {
        ensure!(
            mint_amount >= expected_mint_amount,
            ContractError::MintAmountMismatch {
                expected: expected_mint_amount,
                actual: mint_amount
            }
        );
    }

    // TODO: Confirm Uint128 to String conversion is ok (proto requires this)
    //       Needs testing and validation - also need to check mint_to_address
    //
    // Mint liquid staking token
    let mint_msg = MsgMint {
        sender: env.contract.address.to_string(),
        amount: Some(Coin {
            denom: config.liquid_stake_token_denom.clone(),
            amount: mint_amount.to_string(),
        }),
        mint_to_address,
    };

    // Transfer native token to multisig address
    let sub_msg = transfer_stake_sub_msg(&mut deps, &env, amount, None)?;
    let update_oracle_msgs = update_oracle_msgs(deps.as_ref(), env, &config)?;

    state.total_native_token += amount;
    state.total_liquid_stake_token += mint_amount;

    STATE.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_message(mint_msg)
        .add_messages(update_oracle_msgs)
        .add_submessage(sub_msg)
        .add_attribute("action", "liquid_stake")
        .add_attribute("sender", info.sender.to_string())
        .add_attribute("in_amount", amount)
        .add_attribute("mint_amount", mint_amount))
}

pub fn execute_liquid_unstake(
    mut deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    amount: Uint128,
) -> ContractResult<Response> {
    let config = CONFIG.load(deps.storage)?;

    check_stopped(&config)?;

    STATE.load(deps.storage)?;

    // Load current pending batch
    let pending_batch_id = PENDING_BATCH_ID.load(deps.storage)?;

    // Add unstake request to pending batch
    let pending_unstake_request =
        unstake_requests().may_load(deps.storage, (pending_batch_id, info.sender.to_string()))?;
    let is_new_request = pending_unstake_request.is_none();
    match pending_unstake_request {
        Some(_) => {
            unstake_requests().update(
                deps.storage,
                (pending_batch_id, info.sender.to_string()),
                |or| -> Result<UnstakeRequest, ContractError> {
                    match or {
                        Some(r) => Ok(UnstakeRequest {
                            batch_id: r.batch_id,
                            user: r.user.clone(),
                            amount: r.amount + amount,
                        }),
                        None => Err(ContractError::NoRequestInBatch {}),
                    }
                },
            )?;
        }
        None => {
            new_unstake_request(&mut deps, info.sender.to_string(), pending_batch_id, amount)?;
        }
    }

    // Add amount to batch total (stTIA)
    BATCHES.update(
        deps.storage,
        pending_batch_id,
        |_batch| -> Result<Batch, ContractError> {
            let mut batch = _batch.unwrap();
            batch.batch_total_liquid_stake += amount;
            if is_new_request {
                batch.unstake_requests_count = Some(batch.unstake_requests_count.unwrap_or(0) + 1);
            }
            Ok(batch)
        },
    )?;

    Ok(Response::new()
        .add_attribute("action", "liquid_unstake")
        .add_attribute("sender", info.sender.to_string())
        .add_attribute("batch", pending_batch_id.to_string())
        .add_attribute("amount", amount))
}

// Submit batch and transition pending batch to submitted
// Called automatically during liquidUnstake, but also can be called by anyone
pub fn execute_submit_batch(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
) -> ContractResult<Response> {
    let config = CONFIG.load(deps.storage)?;

    check_stopped(&config)?;

    let pending_batch_id = PENDING_BATCH_ID.load(deps.storage)?;
    let mut batch = BATCHES.load(deps.storage, pending_batch_id)?;

    if let Some(est_next_batch_time) = batch.next_batch_action_time {
        // Check if the batch has been submitted
        if env.block.time.seconds() < est_next_batch_time {
            return Err(ContractError::BatchNotReady {
                actual: env.block.time.seconds(),
                expected: est_next_batch_time,
            });
        }
    } else {
        // Should not enter as pending batch should have a next batch action time
        return Err(ContractError::BatchNotReady {
            actual: env.block.time.seconds(),
            expected: 0u64,
        });
    }

    let unstake_requests = unstake_requests()
        .prefix(pending_batch_id)
        .range(deps.storage, None, None, Order::Ascending)
        .take(1)
        .count();

    if unstake_requests == 0 {
        return Err(ContractError::BatchEmpty {});
    }

    let mut state = STATE.load(deps.storage)?;

    // TODO: Circuit break?
    // Need to add a test for this
    ensure!(
        state.total_liquid_stake_token >= batch.batch_total_liquid_stake,
        ContractError::InvalidUnstakeAmount {
            total_liquid_stake_token: (state.total_liquid_stake_token),
            amount_to_unstake: (batch.batch_total_liquid_stake)
        }
    );

    // Create new pending batch
    let new_pending_batch = Batch::new(
        batch.id + 1,
        Uint128::zero(),
        env.block.time.seconds() + config.batch_period,
    );

    // Save new pending batch
    BATCHES.save(deps.storage, new_pending_batch.id, &new_pending_batch)?;
    PENDING_BATCH_ID.save(deps.storage, &new_pending_batch.id)?;

    // Issue tokenfactory burn message
    // Waiting until batch submission to burn tokens
    let tokenfactory_burn_msg = MsgBurn {
        sender: env.contract.address.to_string(),
        amount: Some(Coin {
            denom: config.liquid_stake_token_denom.clone(),
            amount: batch.batch_total_liquid_stake.to_string(),
        }),
        burn_from_address: env.contract.address.to_string(),
    };

    let unbond_amount = compute_unbond_amount(
        state.total_native_token,
        state.total_liquid_stake_token,
        batch.batch_total_liquid_stake,
    );

    // Reduce underlying TIA balance by unbonded amount
    state.total_native_token = state
        .total_native_token
        .checked_sub(unbond_amount)
        .unwrap_or_else(|_| Uint128::zero());

    // Reduce underlying stTIA balance by batch total
    state.total_liquid_stake_token = state
        .total_liquid_stake_token
        .checked_sub(batch.batch_total_liquid_stake)
        .unwrap_or_else(|_| Uint128::zero());

    STATE.save(deps.storage, &state)?;

    // Update batch status
    batch.expected_native_unstaked = Some(unbond_amount);
    batch.update_status(
        BatchStatus::Submitted,
        Some(env.block.time.seconds() + config.unbonding_period),
    );

    BATCHES.save(deps.storage, batch.id, &batch)?;

    let update_oracle_msgs = update_oracle_msgs(deps.as_ref(), env, &config)?;

    Ok(Response::new()
        .add_message(tokenfactory_burn_msg)
        .add_messages(update_oracle_msgs)
        .add_attribute("action", "submit_batch")
        .add_attribute("batch_id", batch.id.to_string())
        .add_attribute("batch_total", batch.batch_total_liquid_stake)
        .add_attribute("expected_native_unstaked", unbond_amount))
}

// doing a "push over pool" pattern for now
// eventually we can move this to auto-withdraw all funds upon batch completion
// Reasoning - any one issue in the batch will cause the entire batch to fail
pub fn execute_withdraw(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    batch_id: u64,
) -> ContractResult<Response> {
    let config: Config = CONFIG.load(deps.storage)?;

    check_stopped(&config)?;

    let _batch = BATCHES.load(deps.storage, batch_id);
    if _batch.is_err() {
        return Err(ContractError::BatchEmpty {});
    }
    let batch = _batch.unwrap();

    if batch.status != BatchStatus::Received {
        return Err(ContractError::TokensAlreadyClaimed { batch_id: batch.id });
    }
    let received_native_unstaked = batch.received_native_unstaked.as_ref().unwrap();

    let _liquid_unstake_request =
        unstake_requests().may_load(deps.storage, (batch.id, info.sender.to_string()))?;
    if _liquid_unstake_request.is_none() {
        return Err(ContractError::NoRequestInBatch {});
    }

    let unstake_request_amount = _liquid_unstake_request.unwrap().amount;

    let amount = received_native_unstaked
        .multiply_ratio(unstake_request_amount, batch.batch_total_liquid_stake);

    // TODO: if all liquid unstake requests have been withdrawn, delete the batch?
    remove_unstake_request(&mut deps, info.sender.to_string(), batch.id)?;

    let mut messages: Vec<CosmosMsg> = vec![];
    let send_msg = MsgSend {
        from_address: env.contract.address.to_string(),
        to_address: info.sender.to_string(),
        amount: vec![Coin {
            denom: config.native_token_denom.clone(),
            amount: amount.to_string(),
        }],
    };
    messages.push(send_msg.into());

    let update_oracle_msgs = update_oracle_msgs(deps.as_ref(), env, &config)?;

    Ok(Response::new()
        .add_attribute("action", "execute_withdraw")
        .add_attribute("batch", batch.id.to_string())
        .add_attribute("amount", amount.to_string())
        .add_messages([messages, update_oracle_msgs].concat())
    )
}

// Add a validator to the list of validators; callable by the owner
pub fn execute_add_validator(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    new_validator: String,
) -> ContractResult<Response> {
    ADMIN.assert_admin(deps.as_ref(), &info.sender)?;

    let mut config = CONFIG.load(deps.storage)?;
    let new_validator_addr = validate_address(&new_validator, CELESTIA_VALIDATOR_PREFIX)?;

    // Check if the new_validator is already in the list.
    if config
        .validators
        .iter()
        .any(|validator| *validator == new_validator_addr)
    {
        return Err(ContractError::DuplicateValidator {
            validator: new_validator.clone(),
        });
    }

    // Add the new validator to the list.
    config.validators.push(new_validator_addr.clone());

    // Save the updated config.
    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new()
        .add_attribute("action", "add_validator")
        .add_attribute("new_validator", new_validator_addr)
        .add_attribute("sender", info.sender))
}

pub fn execute_remove_validator(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    validator_to_remove: String,
) -> ContractResult<Response> {
    ADMIN.assert_admin(deps.as_ref(), &info.sender)?;

    let mut config = CONFIG.load(deps.storage)?;
    let validator_addr_to_remove =
        validate_address(&validator_to_remove, CELESTIA_VALIDATOR_PREFIX)?;

    // Find the position of the validator to be removed.
    if let Some(pos) = config
        .validators
        .iter()
        .position(|validator| *validator == validator_addr_to_remove)
    {
        // Remove the validator if found.
        config.validators.remove(pos);
    } else {
        // If the validator is not found, return an error.
        return Err(ContractError::ValidatorNotFound {
            validator: validator_to_remove.clone(),
        });
    }

    // Save the updated config.
    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new()
        .add_attribute("action", "remove_validator")
        .add_attribute("removed_validator", validator_addr_to_remove)
        .add_attribute("sender", info.sender))
}

// Transfer ownership to another account; callable by the owner
// This will require the new owner to accept to take effect.
// No need to handle case of overwriting the pending owner
// Ownership can only be claimed after 7 days to mitigate fat finger errors
pub fn execute_transfer_ownership(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    new_owner: String,
) -> ContractResult<Response> {
    ADMIN.assert_admin(deps.as_ref(), &info.sender)?;

    let mut state: State = STATE.load(deps.storage)?;
    state.pending_owner = Some(deps.api.addr_validate(&new_owner)?);
    state.owner_transfer_min_time = Some(Timestamp::from_seconds(
        _env.block.time.seconds() + 60 * 60 * 24 * 7,
    )); // 7 days

    STATE.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("action", "transfer_ownership")
        .add_attribute("new_owner", new_owner)
        .add_attribute("previous_owner", info.sender))
}

// Revoke transfer ownership, callable by the owner
pub fn execute_revoke_ownership_transfer(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
) -> ContractResult<Response> {
    ADMIN.assert_admin(deps.as_ref(), &info.sender)?;

    let mut state = STATE.load(deps.storage)?;
    state.pending_owner = None;
    state.owner_transfer_min_time = None;

    STATE.save(deps.storage, &state)?;

    Ok(Response::new().add_attribute("action", "revoke_ownership_transfer"))
}

pub fn execute_accept_ownership(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
) -> ContractResult<Response> {
    let mut state: State = STATE.load(deps.storage)?;
    if state.owner_transfer_min_time.is_some()
        && state.owner_transfer_min_time.unwrap().seconds() > _env.block.time.seconds()
    {
        return Err(ContractError::OwnershipTransferNotReady {
            time_to_claim: Timestamp::from_seconds(
                state.owner_transfer_min_time.unwrap().seconds(),
            ),
        });
    }

    let new_owner = {
        match state.pending_owner {
            Some(pending_owner) if pending_owner == info.sender => {
                state.pending_owner = None;
                STATE.save(deps.storage, &state)?;
                Some(pending_owner)
            }
            _ => None,
        }
    };

    match new_owner {
        Some(pending_owner) => {
            ADMIN.set(deps, Some(pending_owner))?;
            Ok(Response::new()
                .add_attribute("action", "accept_ownership")
                .add_attribute("new_owner", info.sender))
        }
        None => Err(ContractError::NoPendingOwner {}),
    }
}

pub fn recover(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    selected_packets: Option<Vec<u64>>,
    page: bool,
) -> Result<Response, ContractError> {
    let page_size = 10;

    // forced recovery is dangerous and should only be done by the admin
    if selected_packets.is_some() {
        ADMIN.assert_admin(deps.as_ref(), &info.sender)?;
    }

    // timed out and failed packets
    let packets: Vec<IBCTransfer> = if selected_packets.is_some() {
        let selected_packets = selected_packets.unwrap();
        let mut packets: Vec<IBCTransfer> = vec![];
        for packet_id in selected_packets {
            let packet = INFLIGHT_PACKETS.load(deps.storage, packet_id)?;
            packets.push(packet);
        }
        packets
    } else {
        let packets: Vec<IBCTransfer> = paginate_map(
            deps.as_ref(),
            &INFLIGHT_PACKETS,
            None,
            if page { Some(page_size) } else { None },
            Order::Ascending,
            Some(Box::new(|r: &IBCTransfer| {
                r.status == PacketLifecycleStatus::AckFailure
                    || r.status == PacketLifecycleStatus::TimedOut
            })),
        )?;
        packets
    };

    if packets.is_empty() {
        return Err(ContractError::NoInflightPackets {});
    }

    let max_submessage_id = INFLIGHT_PACKETS
        .range(deps.storage, None, None, Order::Descending)
        .take(1)
        .next()
        .unwrap()
        .unwrap()
        .0;

    let total_amount = packets
        .iter()
        .map(|r| {
            INFLIGHT_PACKETS.remove(deps.storage, r.sequence);
            r.amount
        })
        .reduce(|a, b| a + b)
        .unwrap();

    // this shouldn't collide. any committed submessage package should have enough upper room in the indexes
    // they are based on block times in nano seconds
    // we are fusing all pending transfers into one
    let sub_msg = transfer_stake_sub_msg(
        &mut deps,
        &env,
        Uint128::from(total_amount),
        Some(max_submessage_id + 1),
    )?;

    Ok(Response::new()
        .add_attribute("action", "recover")
        .add_attribute("packets", packets.len().to_string())
        .add_submessage(sub_msg))
}

pub fn update_config_from_migrate(
    deps: DepsMut,
    msg: MigrateMsg,
) -> ContractResult<Response> {
    let mut config: Config = CONFIG.load(deps.storage)?;
   
    // update oracle contract address v2
    if msg.oracle_contract_address_v2.is_some() {
        let oracle_contract_address_v2 = msg.oracle_contract_address_v2.unwrap();
        validate_address(&oracle_contract_address_v2, "osmo")?;
        config.oracle_contract_address_v2 = Some(Addr::unchecked(oracle_contract_address_v2));
    }
    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new().add_attribute("action", "update_oracle_contract_address_v2"))
}

// Update the config; callable by the owner
pub fn update_config(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    batch_period: Option<u64>,
    unbonding_period: Option<u64>,
    minimum_liquid_stake_amount: Option<Uint128>,
    multisig_address_config: Option<MultisigAddressConfig>,
    protocol_fee_config: Option<ProtocolFeeConfig>,
    native_token_denom: Option<String>,
    channel_id: Option<String>,
    monitors: Option<Vec<String>>,
    treasury_address: Option<String>,
    oracle_contract_address: Option<String>,
    oracle_contract_address_v2: Option<String>,
) -> ContractResult<Response> {
    ADMIN.assert_admin(deps.as_ref(), &info.sender)?;

    let mut config: Config = CONFIG.load(deps.storage)?;

    if let Some(batch_period) = batch_period {
        config.batch_period = batch_period;
    }
    if let Some(unbonding_period) = unbonding_period {
        config.unbonding_period = unbonding_period;
    }
    if let Some(minimum_liquid_stake_amount) = minimum_liquid_stake_amount {
        config.minimum_liquid_stake_amount = minimum_liquid_stake_amount;
    }
    if let Some(multisig_address_config) = multisig_address_config {
        config.multisig_address_config = multisig_address_config;
    }
    if let Some(protocol_fee_config) = protocol_fee_config {
        config.protocol_fee_config = protocol_fee_config;
    }
    if let Some(monitors) = monitors {
        validate_addresses(&monitors, "osmo")?;
        config.monitors = Some(monitors.into_iter().map(|o| Addr::unchecked(o)).collect());
    }
    if let Some(treasury_address) = treasury_address {
        validate_address(&treasury_address, "osmo")?;
        config.treasury_address = Addr::unchecked(treasury_address);
    }

    // TODO get reserve token from channel? Maybe leave as safeguard?
    if channel_id.is_some() || native_token_denom.is_some() {
        if channel_id.is_none() || native_token_denom.is_none() {
            return Err(ContractError::IbcChannelConfigWrong {});
        }

        let channel_id = channel_id.unwrap();
        let native_token_denom = native_token_denom.unwrap();
        let channel_id_correct = channel_id.starts_with("channel-")
            && channel_id
                .strip_prefix("channel-")
                .unwrap()
                .parse::<u64>()
                .is_ok();
        let native_token_denom_correct = native_token_denom.starts_with("ibc/")
            && native_token_denom.strip_prefix("ibc/").unwrap().len() == 64;

        if !channel_id_correct || !native_token_denom_correct {
            return Err(ContractError::IbcChannelConfigWrong {});
        }

        config.ibc_channel_id = channel_id;
        config.native_token_denom = native_token_denom;
    }

    if oracle_contract_address.is_some() {
        let oracle_contract_address = oracle_contract_address.unwrap();
        validate_address(&oracle_contract_address, "osmo")?;
        config.oracle_contract_address = Some(Addr::unchecked(oracle_contract_address));
    }

    if oracle_contract_address_v2.is_some() {
        let oracle_contract_address_v2 = oracle_contract_address_v2.unwrap();
        validate_address(&oracle_contract_address_v2, "osmo")?;
        config.oracle_contract_address_v2 = Some(Addr::unchecked(oracle_contract_address_v2));
    }

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new().add_attribute("action", "update_config"))
}

pub fn receive_rewards(mut deps: DepsMut, env: Env, info: MessageInfo) -> ContractResult<Response> {
    let config: Config = CONFIG.load(deps.storage)?;
    let mut state: State = STATE.load(deps.storage)?;

    check_stopped(&config)?;

    if state.total_liquid_stake_token.is_zero() {
        return Err(ContractError::NoLiquidStake {});
    }

    let expected_sender = derive_intermediate_sender(
        &config.ibc_channel_id,
        config
            .multisig_address_config
            .reward_collector_address
            .as_ref(),
        "osmo",
    );
    if expected_sender.is_err() {
        return Err(ContractError::Unauthorized {
            sender: info.sender.to_string(),
        });
    }
    if info.sender != expected_sender.unwrap() {
        return Err(ContractError::Unauthorized {
            sender: info.sender.to_string(),
        });
    }

    let coin = info
        .funds
        .iter()
        .find(|c| c.denom == config.native_token_denom);
    if coin.is_none() {
        return Err(ContractError::Payment(PaymentError::NoFunds {}));
    }

    let amount = coin.unwrap().amount;
    let fee = config
        .protocol_fee_config
        .dao_treasury_fee
        .multiply_ratio(amount, 100_000u128);
    let amount_after_fees = amount.checked_sub(fee);
    if amount_after_fees.is_err() {
        return Err(ContractError::ReceiveRewardsTooSmall {
            amount,
            minimum: fee,
        });
    }
    let amount_after_fees = amount_after_fees.unwrap();

    // update the accounting of tokens
    state.total_native_token += amount_after_fees.clone();
    state.total_reward_amount += amount.clone();
    state.total_fees += fee;

    STATE.save(deps.storage, &state)?;

    // transfer the funds to Celestia to be staked
    let ibc_transfer_msg =
        transfer_stake_sub_msg(&mut deps, &env, amount_after_fees.clone(), None)?;
    let update_oracle_msgs = update_oracle_msgs(deps.as_ref(), env, &config)?;

    Ok(Response::new()
        .add_attribute("action", "receive_rewards")
        .add_attribute("action", "transfer_stake")
        .add_attribute("amount", amount)
        .add_attribute("amount_after_fees", amount_after_fees)
        .add_messages(update_oracle_msgs)
        .add_submessage(ibc_transfer_msg))
}

pub fn receive_unstaked_tokens(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    batch_id: u64,
) -> ContractResult<Response> {
    let config: Config = CONFIG.load(deps.storage)?;

    check_stopped(&config)?;

    let expected_sender = derive_intermediate_sender(
        &config.ibc_channel_id,
        config.multisig_address_config.staker_address.as_ref(),
        "osmo",
    );
    if expected_sender.is_err() {
        return Err(ContractError::Unauthorized {
            sender: info.sender.to_string(),
        });
    }
    if info.sender != expected_sender.unwrap() {
        return Err(ContractError::Unauthorized {
            sender: info.sender.to_string(),
        });
    }

    let coin = info
        .funds
        .iter()
        .find(|c| c.denom == config.native_token_denom);
    if coin.is_none() {
        return Err(ContractError::Payment(PaymentError::NoFunds {}));
    }

    let amount = coin.unwrap().amount;

    let mut batch: Batch = BATCHES.load(deps.storage, batch_id)?;

    if batch.status != BatchStatus::Submitted {
        return Err(ContractError::BatchNotClaimable {
            batch_id: batch.id,
            status: batch.status,
        });
    }

    if batch.next_batch_action_time.is_none() {
        return Err(ContractError::BatchNotClaimable {
            batch_id: batch.id,
            status: batch.status,
        });
    }
    let next_batch_action_time = batch.next_batch_action_time.unwrap();
    if next_batch_action_time > env.block.time.seconds() {
        return Err(ContractError::BatchNotReady {
            actual: env.block.time.seconds(),
            expected: next_batch_action_time,
        });
    }

    batch.received_native_unstaked = Some(amount.clone());
    batch.update_status(BatchStatus::Received, None);

    BATCHES.save(deps.storage, batch.id, &batch)?;

    Ok(Response::new()
        .add_attribute("action", "receive_unstaked_tokens")
        .add_attribute("batch", batch_id.to_string())
        .add_attribute("amount", amount))
}

pub fn circuit_breaker(deps: DepsMut, _env: Env, info: MessageInfo) -> ContractResult<Response> {
    let sender = info.sender.to_string();

    let mut config: Config = CONFIG.load(deps.storage)?;

    if ADMIN.assert_admin(deps.as_ref(), &info.sender).is_err() {
        if !config
            .clone()
            .monitors
            .unwrap_or(vec![])
            .iter()
            .any(|v| *v == sender)
        {
            return Err(ContractError::Unauthorized { sender });
        }
    }

    config.stopped = true;
    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new().add_attribute("action", "circuit_breaker"))
}

pub fn resume_contract(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    total_native_token: Uint128,
    total_liquid_stake_token: Uint128,
    total_reward_amount: Uint128,
) -> ContractResult<Response> {
    ADMIN.assert_admin(deps.as_ref(), &info.sender)?;

    let mut config: Config = CONFIG.load(deps.storage)?;

    config.stopped = false;
    CONFIG.save(deps.storage, &config)?;

    let mut state: State = STATE.load(deps.storage)?;

    state.total_native_token = total_native_token;
    state.total_liquid_stake_token = total_liquid_stake_token;
    state.total_reward_amount = total_reward_amount;

    let update_oracle_msgs = update_oracle_msgs(deps.as_ref(), env, &config)?;

    STATE.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("action", "resume_contract")
        .add_attribute("total_native_token", total_native_token)
        .add_attribute("total_liquid_stake_token", total_liquid_stake_token)
        .add_attribute("total_reward_amount", total_reward_amount)
        .add_messages(update_oracle_msgs))
}

pub fn handle_ibc_reply(deps: DepsMut, msg: cosmwasm_std::Reply) -> ContractResult<Response> {
    // Parse the result from the underlying chain call (IBC send)
    let SubMsgResult::Ok(SubMsgResponse { data: Some(b), .. }) = msg.result else {
        return Err(ContractError::FailedIBCTransfer {
            msg: format!("failed reply: {:?}", msg.result),
        });
    };

    // The response contains the packet sequence. This is needed to be able to
    // ensure that, if there is a delivery failure, the packet that failed is
    // the same one that we stored recovery information for
    let transfer_response =
        MsgTransferResponse::decode(&b[..]).map_err(|_e| ContractError::FailedIBCTransfer {
            msg: format!("could not decode response: {b}"),
        })?;

    let IbcWaitingForReply { amount } = IBC_WAITING_FOR_REPLY.load(deps.storage, msg.id)?;
    IBC_WAITING_FOR_REPLY.remove(deps.storage, msg.id);

    let recovery = IBCTransfer {
        sequence: transfer_response.sequence,
        amount,
        status: PacketLifecycleStatus::Sent,
    };

    // Save as in-flight to be able to manipulate when the ack/timeout is received
    INFLIGHT_PACKETS.save(deps.storage, transfer_response.sequence, &recovery)?;

    let response = Response::new()
        .add_attribute("action", "handle_ibc_reply")
        .add_attribute("status", "ibc_message_successfully_submitted")
        .add_attribute(
            "packet_sequence",
            format!("{:?}", transfer_response.sequence),
        );

    Ok(response)
}

fn save_ibc_waiting_for_reply(
    deps: &mut DepsMut,
    id: u64,
    ibc_msg: IbcWaitingForReply,
) -> Result<(), ContractError> {
    // Check that there isn't anything stored in IBC_WAITING_FOR_REPLY. If there
    // is, it means that the contract is already waiting for a reply and should
    // not override the stored state. This should never happen here, but adding
    // the check for safety. If this happens there is likely a malicious attempt
    // modify the contract's state before it has replied.
    if IBC_WAITING_FOR_REPLY.may_load(deps.storage, id)?.is_some() {
        return Err(ContractError::ContractLocked {
            msg: "Already waiting for a reply".to_string(),
        });
    }
    // Store the ibc send information and the user's failed delivery preference
    // so that it can be handled by the response
    IBC_WAITING_FOR_REPLY.save(deps.storage, id, &ibc_msg)?;
    Ok(())
}

pub fn fee_withdraw(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    amount: Uint128,
) -> ContractResult<Response> {
    ADMIN.assert_admin(deps.as_ref(), &info.sender)?;

    let config: Config = CONFIG.load(deps.storage)?;
    let mut state: State = STATE.load(deps.storage)?;

    if state.total_fees < amount {
        return Err(ContractError::InsufficientFunds {});
    }

    state.total_fees = state.total_fees.checked_sub(amount).unwrap();
    STATE.save(deps.storage, &state)?;

    let send_msg = MsgSend {
        from_address: env.contract.address.to_string(),
        to_address: config.treasury_address.to_string(),
        amount: vec![Coin {
            denom: config.native_token_denom,
            amount: amount.to_string(),
        }],
    };

    Ok(Response::new()
        .add_attribute("action", "fee_withdraw")
        .add_attribute("receiver", config.treasury_address.to_string())
        .add_attribute("amount", amount)
        .add_message(send_msg))
}
