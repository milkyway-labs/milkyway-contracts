use std::collections::BTreeMap;

use crate::contract::IBC_TIMEOUT;
use crate::error::{ContractError, ContractResult};
use crate::helpers::{
    compute_mint_amount, compute_unbond_amount, dedup_vec, get_rates, paginate_map,
    validate_ibc_denom,
};
use crate::oracle::Oracle;
use crate::state::{
    ibc::{IBCTransfer, PacketLifecycleStatus},
    Config, IbcWaitingForReply, State, ADMIN, BATCHES, CONFIG, IBC_WAITING_FOR_REPLY,
    INFLIGHT_PACKETS, PENDING_BATCH_ID, STATE,
};
use crate::state::{new_unstake_request, remove_unstake_request, unstake_requests, UnstakeRequest};
use crate::tokenfactory;
use crate::types::{
    BatchExpectedAmount, UnsafeNativeChainConfig, UnsafeProtocolChainConfig,
    UnsafeProtocolFeeConfig,
};
use cosmwasm_std::{
    ensure, Coin, CosmosMsg, Deps, DepsMut, Env, IbcTimeout, MessageInfo, Order, ReplyOn, Response,
    SubMsg, SubMsgResponse, SubMsgResult, Timestamp, Uint128,
};
use cw_utils::PaymentError;
use milky_way::staking::{Batch, BatchStatus};
use milky_way::utils::{validate_address, validate_addresses};
use osmosis_std::types::cosmos::bank::v1beta1::MsgSend;
use osmosis_std::types::cosmos::base::v1beta1::Coin as OsmosisCoin;
use osmosis_std::types::cosmwasm::wasm::v1::MsgExecuteContract;
use osmosis_std::types::ibc::applications::transfer::v1::MsgTransfer;
use osmosis_std::types::ibc::applications::transfer::v1::MsgTransferResponse;
use prost::Message;

const FEE_RATE_DENOMINATOR: u64 = 100_000;

pub fn ibc_transfer_msg(
    deps: &Deps,
    env: &Env,
    receiver: impl Into<String>,
    token: Coin,
) -> Result<MsgTransfer, ContractError> {
    let config: Config = CONFIG.load(deps.storage)?;

    if config.protocol_chain_config.ibc_channel_id.is_empty() {
        return Err(ContractError::IbcChannelNotFound {});
    }

    let timeout = IbcTimeout::with_timestamp(Timestamp::from_nanos(
        env.block.time.nanos() + IBC_TIMEOUT.nanos(),
    ));

    let ibc_msg = MsgTransfer {
        source_channel: config.protocol_chain_config.ibc_channel_id,
        source_port: "transfer".to_string(),
        token: Some(OsmosisCoin::from(token)),
        receiver: receiver.into(),
        sender: env.contract.address.to_string(),
        timeout_height: None,
        timeout_timestamp: timeout.timestamp().unwrap().nanos(),
        memo: format!("{{\"ibc_callback\":\"{}\"}}", env.contract.address),
    };

    Ok(ibc_msg)
}

fn ibc_transfer_sub_msg(
    deps: &mut DepsMut,
    env: &Env,
    receiver: impl Into<String>,
    amount: Coin,
    sub_msg_id: Option<u64>,
) -> Result<SubMsg, ContractError> {
    let receiver = receiver.into();
    let ibc_msg = ibc_transfer_msg(&deps.as_ref(), env, &receiver, amount.clone())?;
    let sub_msg_id = sub_msg_id.unwrap_or({
        match env.transaction {
            Some(ref tx) => tx.index as u64 + env.block.time.nanos(),
            None => env.block.time.nanos(),
        }
    });

    let ibc_waiting_for_reply = IbcWaitingForReply { amount, receiver };

    save_ibc_waiting_for_reply(deps, sub_msg_id, ibc_waiting_for_reply)?;

    Ok(SubMsg {
        id: sub_msg_id,
        msg: ibc_msg.into(),
        gas_limit: None,
        reply_on: ReplyOn::Always,
    })
}

fn update_oracle_msgs(
    env: &Env,
    config: &Config,
    state: &State,
) -> Result<Vec<CosmosMsg>, ContractError> {
    let mut messages: Vec<CosmosMsg> = Vec::new();

    if let Some(oracle_address) = &config.protocol_chain_config.oracle_address {
        let (redemption_rate, purchase_rate) = get_rates(state);
        // Post rates to Milkyway Oracle contract
        let post_rates_msg = Oracle::PostRates {
            purchase_rate: purchase_rate.to_string(),
            redemption_rate: redemption_rate.to_string(),
            denom: config.liquid_stake_token_denom.clone(),
        };

        let post_rate_msg_json = serde_json::to_string(&post_rates_msg).unwrap();
        messages.push(
            MsgExecuteContract {
                sender: env.contract.address.to_string(),
                contract: oracle_address.to_string(),
                msg: post_rate_msg_json.as_bytes().to_vec(),
                funds: vec![],
            }
            .into(),
        );
    }

    Ok(messages)
}

pub fn check_stopped(config: &Config) -> Result<(), ContractError> {
    if config.stopped {
        return Err(ContractError::Stopped {});
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
    transfer_to_native_chain: Option<bool>,
    expected_mint_amount: Option<Uint128>,
) -> ContractResult<Response> {
    let config = CONFIG.load(deps.storage)?;

    check_stopped(&config)?;

    let mint_to_address = mint_to.unwrap_or_else(|| info.sender.to_string());
    let mint_to_is_native = validate_address(
        &mint_to_address,
        &config.native_chain_config.account_address_prefix,
    )
    .is_ok();
    let mut mint_to_is_protocol = validate_address(
        &mint_to_address,
        &config.protocol_chain_config.account_address_prefix,
    )
    .is_ok();

    // Ensure the mint to is either a protocol chain account or
    // native chain account.
    if !mint_to_is_protocol && !mint_to_is_native {
        return Err(ContractError::InvalidAddress {});
    }

    // There may be cases where the address prefixes of the native chain and the protocol chain are the same.
    // In such cases, we determine the target chain based on the `transfer_to_native_chain` flag.
    if mint_to_is_native && mint_to_is_protocol && transfer_to_native_chain.unwrap_or(false) {
        mint_to_is_protocol = false;
    }

    let mut state: State = STATE.load(deps.storage)?;
    ensure!(
        amount >= config.protocol_chain_config.minimum_liquid_stake_amount,
        ContractError::MinimumLiquidStakeAmount {
            minimum_stake_amount: config.protocol_chain_config.minimum_liquid_stake_amount,
            sent_amount: amount,
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

    // Mint liquid staking token
    let mint_msg = tokenfactory::mint(
        env.contract.address.to_string(),
        cosmwasm_std::Coin {
            denom: config.liquid_stake_token_denom.clone(),
            amount: mint_amount,
        },
        env.contract.address.to_string(),
    )?;

    // Transfer native token to multisig address
    let stake_sub_message = ibc_transfer_sub_msg(
        &mut deps,
        &env,
        &config.native_chain_config.staker_address,
        Coin::new(amount.u128(), &config.protocol_chain_config.ibc_token_denom),
        None,
    )?;
    state.total_native_token += amount;
    state.total_liquid_stake_token += mint_amount;
    STATE.save(deps.storage, &state)?;

    // Get the stake sub message id so if we need to ibc transfer the minted
    // liquid staked tokens we use this id plus one.
    let stake_sub_message_id = stake_sub_message.id;
    let update_oracle_msgs = update_oracle_msgs(&env, &config, &state)?;

    let response = Response::new()
        .add_message(mint_msg)
        .add_messages(update_oracle_msgs)
        .add_submessage(stake_sub_message)
        .add_attribute("action", "liquid_stake")
        .add_attribute("sender", info.sender.to_string())
        .add_attribute("in_amount", amount)
        .add_attribute("mint_amount", mint_amount);

    let response = if mint_to_is_protocol {
        // Send the minted tokens to the user on the protocol network trough a MsgSend
        response.add_message(osmosis_std::types::cosmos::bank::v1beta1::MsgSend {
            from_address: env.contract.address.to_string(),
            to_address: mint_to_address,
            amount: vec![OsmosisCoin {
                denom: config.liquid_stake_token_denom.clone(),
                amount: mint_amount.to_string(),
            }],
        })
    } else {
        // IBC transfer the minted liquid staked representation
        // back to the native chain account
        response.add_submessage(ibc_transfer_sub_msg(
            &mut deps,
            &env,
            mint_to_address,
            Coin::new(mint_amount.u128(), &config.liquid_stake_token_denom),
            Some(stake_sub_message_id + 1),
        )?)
    };

    Ok(response)
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

/// Submit batch and transition pending batch to submitted.
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

    if batch.unstake_requests_count.unwrap_or(0) == 0 {
        return Err(ContractError::BatchEmpty {});
    }

    let mut state = STATE.load(deps.storage)?;

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
    let tokenfactory_burn_msg = tokenfactory::burn(
        env.contract.address.to_string(),
        cosmwasm_std::Coin {
            denom: config.liquid_stake_token_denom.clone(),
            amount: batch.batch_total_liquid_stake,
        },
        env.contract.address.to_string(),
    )?;

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
        Some(env.block.time.seconds() + config.native_chain_config.unbonding_period),
    );

    BATCHES.save(deps.storage, batch.id, &batch)?;

    let update_oracle_msgs = update_oracle_msgs(&env, &config, &state)?;

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

    let liquid_unstake_request = unstake_requests()
        .may_load(deps.storage, (batch.id, info.sender.to_string()))?
        .ok_or(ContractError::NoRequestInBatch {})?;

    let amount = received_native_unstaked.multiply_ratio(
        liquid_unstake_request.amount,
        batch.batch_total_liquid_stake,
    );

    remove_unstake_request(&mut deps, info.sender.to_string(), batch.id)?;

    let mut messages: Vec<CosmosMsg> = vec![];
    let send_msg = MsgSend {
        from_address: env.contract.address.to_string(),
        to_address: info.sender.to_string(),
        amount: vec![OsmosisCoin {
            denom: config.protocol_chain_config.ibc_token_denom.clone(),
            amount: amount.to_string(),
        }],
    };
    messages.push(send_msg.into());

    let state = STATE.load(deps.storage)?;
    let update_oracle_msgs = update_oracle_msgs(&env, &config, &state)?;

    Ok(Response::new()
        .add_attribute("action", "execute_withdraw")
        .add_attribute("batch", batch.id.to_string())
        .add_attribute("amount", amount.to_string())
        .add_messages([messages, update_oracle_msgs].concat()))
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
    let new_validator_addr = validate_address(
        &new_validator,
        &config.native_chain_config.validator_address_prefix,
    )?;

    // Check if the new_validator is already in the list.
    if config
        .native_chain_config
        .validators
        .iter()
        .any(|validator| *validator == new_validator_addr)
    {
        return Err(ContractError::DuplicateValidator {
            validator: new_validator.clone(),
        });
    }

    // Add the new validator to the list.
    config
        .native_chain_config
        .validators
        .push(new_validator_addr.clone());

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
    let validator_addr_to_remove = validate_address(
        &validator_to_remove,
        &config.native_chain_config.validator_address_prefix,
    )?;

    // Find the position of the validator to be removed.
    if let Some(pos) = config
        .native_chain_config
        .validators
        .iter()
        .position(|validator| *validator == validator_addr_to_remove)
    {
        // Remove the validator if found.
        config.native_chain_config.validators.remove(pos);
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
    _info: MessageInfo,
    selected_packets: Option<Vec<u64>>,
    receiver: Option<String>,
    page: bool,
) -> Result<Response, ContractError> {
    let page_size = 10;
    let config = CONFIG.load(deps.storage)?;
    let receiver = receiver
        // Validate the address
        .map(|s| validate_address(&s, &config.native_chain_config.account_address_prefix))
        .transpose()?
        // Fallback to staker address in case the receiver was None
        .unwrap_or(config.native_chain_config.staker_address);

    // timed out and failed packets
    let packets: Vec<IBCTransfer> = if selected_packets.is_some() {
        let selected_packets = dedup_vec(selected_packets.unwrap());
        let mut packets: Vec<IBCTransfer> = vec![];
        for packet_id in selected_packets {
            let packet = INFLIGHT_PACKETS.load(deps.storage, packet_id)?;
            // Ensure the selected packet are all for the same user
            if packet.receiver != receiver.as_str() {
                return Err(ContractError::InvalidReceiver {});
            }
            if packet.status != PacketLifecycleStatus::AckFailure
                && packet.status != PacketLifecycleStatus::TimedOut
            {
                return Err(ContractError::InvalidPacketStatus { id: packet_id });
            }
            packets.push(packet);
        }
        packets
    } else {
        // Clone the receiver so that can be captured by the filter function.
        let receiver = receiver.clone();
        let packets: Vec<IBCTransfer> = paginate_map(
            deps.as_ref(),
            &INFLIGHT_PACKETS,
            None,
            if page { Some(page_size) } else { None },
            Order::Ascending,
            Some(Box::new(move |r: &IBCTransfer| {
                r.receiver == receiver
                    && (r.status == PacketLifecycleStatus::AckFailure
                        || r.status == PacketLifecycleStatus::TimedOut)
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

    // Compute the total amount and remove the packets from the
    // INFLIGHT_PACKETS state.
    let mut total_amounts = BTreeMap::<String, Uint128>::new();
    let mut handled_packets_count = 0usize;
    for packet in packets.iter() {
        let coin_amount = total_amounts.get(&packet.amount.denom);

        // Compute the new amount
        let new_amount = if let Some(amount) = coin_amount {
            // We found the value in the map, update it.
            (*amount).checked_add(packet.amount.amount)
        } else {
            Ok(packet.amount.amount)
        };

        if let Ok(amount) = new_amount {
            // If we have correctly computed the new amount
            // remove the packet from the inflight packets
            INFLIGHT_PACKETS.remove(deps.storage, packet.sequence);
            // Update the amount for the denom
            total_amounts.insert(packet.amount.denom.clone(), amount);
            // Update the number of handled packets
            handled_packets_count += 1;
        }
    }

    let sub_msgs = total_amounts
        .iter()
        .enumerate()
        .map(|(index, (key, value))| {
            ibc_transfer_sub_msg(
                &mut deps,
                &env,
                receiver.as_str(),
                Coin {
                    denom: key.to_string(),
                    amount: *value,
                },
                Some(max_submessage_id + (index as u64) + 1),
            )
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(Response::new()
        .add_attribute("action", "recover")
        .add_attribute("packets", handled_packets_count.to_string())
        .add_submessages(sub_msgs))
}

// Update the config; callable by the owner
#[allow(clippy::too_many_arguments)]
pub fn update_config(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    native_chain_config: Option<UnsafeNativeChainConfig>,
    protocol_chain_config: Option<UnsafeProtocolChainConfig>,
    protocol_fee_config: Option<UnsafeProtocolFeeConfig>,
    monitors: Option<Vec<String>>,
    batch_period: Option<u64>,
) -> ContractResult<Response> {
    ADMIN.assert_admin(deps.as_ref(), &info.sender)?;

    let mut config: Config = CONFIG.load(deps.storage)?;

    if let Some(native_chain_config) = &native_chain_config {
        config.native_chain_config = native_chain_config.validate()?;
    }

    if let Some(protocol_chain_config) = protocol_chain_config {
        config.protocol_chain_config =
            protocol_chain_config.validate(&config.native_chain_config.token_denom)?;
    }

    // The native chain config contains the native token denom,
    // which influences protocol_chain_config.ibc_token_denom.
    // Ensure that if the native token denom has changed,
    // the configured IBC denom remains valid after updating protocol_chain_config.
    if native_chain_config.is_some() {
        validate_ibc_denom(
            &config.protocol_chain_config.ibc_token_denom,
            &config.protocol_chain_config.ibc_channel_id,
            &config.native_chain_config.token_denom,
        )?;
    }

    if let Some(protocol_fee_config) = protocol_fee_config {
        config.protocol_fee_config = protocol_fee_config.validate(&config.protocol_chain_config)?
    }

    if let Some(monitors) = monitors {
        config.monitors = validate_addresses(
            &monitors,
            &config.protocol_chain_config.account_address_prefix,
        )?;
    }

    if let Some(batch_period) = batch_period {
        // Ensure the batch period is lower then unbonding period.
        if batch_period > config.native_chain_config.unbonding_period {
            return Err(ContractError::ValueTooBig {
                field_name: "batch_period".to_string(),
                value: Uint128::from(config.native_chain_config.unbonding_period),
                max: Uint128::from(batch_period),
            });
        }
        config.batch_period = batch_period;
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

    let coin = info
        .funds
        .iter()
        .find(|c| c.denom == config.protocol_chain_config.ibc_token_denom);
    if coin.is_none() {
        return Err(ContractError::Payment(PaymentError::NoFunds {}));
    }

    let amount = coin.unwrap().amount;
    let fee = config
        .protocol_fee_config
        .dao_treasury_fee
        .multiply_ratio(amount, FEE_RATE_DENOMINATOR);
    if fee.is_zero() {
        return Err(ContractError::ComputedFeesAreZero {
            received_rewards: amount,
        });
    }
    let amount_after_fees = amount.checked_sub(fee);
    if amount_after_fees.is_err() {
        return Err(ContractError::ReceiveRewardsTooSmall {
            amount,
            minimum: fee,
        });
    }
    let amount_after_fees = amount_after_fees.unwrap();

    // update the accounting of tokens
    state.total_native_token += amount_after_fees;
    state.total_reward_amount += amount;
    if config.protocol_fee_config.treasury_address.is_none() {
        state.total_fees += fee;
    }

    STATE.save(deps.storage, &state)?;

    // transfer the funds to Celestia to be staked
    let ibc_transfer_msg = ibc_transfer_sub_msg(
        &mut deps,
        &env,
        &config.native_chain_config.staker_address,
        Coin::new(
            amount_after_fees.u128(),
            &config.protocol_chain_config.ibc_token_denom,
        ),
        None,
    )?;
    let update_oracle_msgs = update_oracle_msgs(&env, &config, &state)?;

    let mut response = Response::new()
        .add_attribute("action", "receive_rewards")
        .add_attribute("action", "transfer_stake")
        .add_attribute("amount", amount)
        .add_attribute("amount_after_fees", amount_after_fees)
        .add_messages(update_oracle_msgs)
        .add_submessage(ibc_transfer_msg);

    if let Some(treasury_address) = config.protocol_fee_config.treasury_address {
        response = response.add_message(cosmwasm_std::BankMsg::Send {
            to_address: treasury_address.to_string(),
            amount: vec![cosmwasm_std::Coin::new(
                fee.u128(),
                config.protocol_chain_config.ibc_token_denom,
            )],
        });
    }

    Ok(response)
}

pub fn receive_unstaked_tokens(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    batch_id: u64,
) -> ContractResult<Response> {
    let config: Config = CONFIG.load(deps.storage)?;

    check_stopped(&config)?;

    let coin = info
        .funds
        .iter()
        .find(|c| c.denom == config.protocol_chain_config.ibc_token_denom);
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

    let expected_native_amount = batch
        .expected_native_unstaked
        .ok_or(ContractError::BatchWithoutExpectedNativeAmount { batch_id })?;
    if expected_native_amount != amount {
        return Err(ContractError::ReceivedWrongBatchAmount {
            batch_id,
            expected: expected_native_amount,
            received: amount,
        });
    }

    batch.received_native_unstaked = Some(amount);
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

    if ADMIN.assert_admin(deps.as_ref(), &info.sender).is_err()
        && !config.monitors.iter().any(|v| *v == sender)
    {
        return Err(ContractError::Unauthorized { sender });
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
    if !config.stopped {
        return Err(ContractError::NotStopped {});
    }

    config.stopped = false;
    CONFIG.save(deps.storage, &config)?;

    let state = STATE.update(
        deps.storage,
        |mut state| -> Result<State, cosmwasm_std::StdError> {
            state.total_native_token = total_native_token;
            state.total_liquid_stake_token = total_liquid_stake_token;
            state.total_reward_amount = total_reward_amount;
            Ok(state)
        },
    )?;

    let update_oracle_msgs = update_oracle_msgs(&env, &config, &state)?;

    Ok(Response::new()
        .add_attribute("action", "resume_contract")
        .add_attribute("total_native_token", total_native_token)
        .add_attribute("total_liquid_stake_token", total_liquid_stake_token)
        .add_attribute("total_reward_amount", total_reward_amount)
        .add_messages(update_oracle_msgs))
}

pub fn slash_batches(
    deps: DepsMut,
    info: MessageInfo,
    expected_amounts: Vec<BatchExpectedAmount>,
) -> ContractResult<Response> {
    ADMIN.assert_admin(deps.as_ref(), &info.sender)?;

    // Ensure the contract is stopped before slashing the batches
    if !CONFIG.load(deps.storage)?.stopped {
        return Err(ContractError::NotStopped {});
    }

    for batch_expected_amount in expected_amounts.iter() {
        let mut batch = BATCHES.load(deps.storage, batch_expected_amount.batch_id)?;
        if batch.status != BatchStatus::Pending && batch.status != BatchStatus::Submitted {
            return Err(ContractError::UnexpecedBatchStatus {
                actual: batch.status,
            });
        }

        if batch.expected_native_unstaked.is_none() {
            return Err(ContractError::BatchWithoutExpectedNativeAmount {
                batch_id: batch_expected_amount.batch_id,
            });
        };

        batch.expected_native_unstaked = Some(batch_expected_amount.amount);

        BATCHES.save(deps.storage, batch_expected_amount.batch_id, &batch)?;
    }

    Ok(Response::new()
        .add_attribute("action", "slash_batches")
        .add_attribute("updated_batches", serde_json::to_string(&expected_amounts)?))
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

    let IbcWaitingForReply { amount, receiver } =
        IBC_WAITING_FOR_REPLY.load(deps.storage, msg.id)?;
    IBC_WAITING_FOR_REPLY.remove(deps.storage, msg.id);

    let recovery = IBCTransfer {
        sequence: transfer_response.sequence,
        amount,
        receiver,
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

    if config.protocol_fee_config.treasury_address.is_none() {
        return Err(ContractError::TreasuryNotConfigured {});
    }
    let treasury_address = config
        .protocol_fee_config
        .treasury_address
        .unwrap()
        .to_string();

    state.total_fees = state.total_fees.checked_sub(amount).unwrap();
    STATE.save(deps.storage, &state)?;

    let send_msg = MsgSend {
        from_address: env.contract.address.to_string(),
        to_address: treasury_address.clone(),
        amount: vec![OsmosisCoin {
            denom: config.protocol_chain_config.ibc_token_denom,
            amount: amount.to_string(),
        }],
    };

    Ok(Response::new()
        .add_attribute("action", "fee_withdraw")
        .add_attribute("receiver", treasury_address)
        .add_attribute("amount", amount)
        .add_message(send_msg))
}
