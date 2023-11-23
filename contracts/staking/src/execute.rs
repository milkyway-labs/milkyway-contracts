use crate::ack::{MsgTransferResponse};
use crate::contract::CELESTIA_VALIDATOR_PREFIX;
use crate::error::{ContractError, ContractResult};
use crate::helpers::{
    compute_mint_amount, compute_unbond_amount, derive_intermediate_sender, validate_address,
};
use crate::state::{
    Config, ForwardMsgReplyState, MultisigAddressConfig, ProtocolFeeConfig, ADMIN, BATCHES, CONFIG,
    FORWARD_REPLY_STATE, IBC_CONFIG, INFLIGHT_PACKETS, PENDING_BATCH_ID, STATE, RECOVERY_STATES, 
    ibc::{IBCTransfer, PacketLifecycleStatus},
};
use cosmwasm_std::{
    ensure, Deps, DepsMut, Env, IbcMsg, IbcTimeout, MessageInfo, Order, Response, SubMsgResponse,
    SubMsgResult, Timestamp, Uint128, Addr,
};
use cw_utils::PaymentError;
use milky_way::staking::{Batch, BatchStatus, LiquidUnstakeRequest};
use osmosis_std::types::cosmos::bank::v1beta1::MsgSend;
use osmosis_std::types::cosmos::base::v1beta1::Coin;
use osmosis_std::types::osmosis::tokenfactory::v1beta1::{MsgBurn, MsgMint};
use prost::Message;


pub fn transfer_stake_msg(deps: Deps, env: Env, amount: Uint128) -> Result<IbcMsg, ContractError> {
    let config = CONFIG.load(deps.storage)?;
    let ibc_config = IBC_CONFIG.load(deps.storage)?;

    if ibc_config.channel_id.is_empty() {
        return Err(ContractError::IbcChannelNotFound {});
    }

    let ibc_coin = cosmwasm_std::Coin {
        denom: config.native_token_denom,
        amount,
    };

    let timeout = IbcTimeout::with_timestamp(Timestamp::from_nanos(
        env.block.time.nanos() + ibc_config.default_timeout.nanos(),
    ));

    let to_address = config.multisig_address_config.staker_address.to_string();
    let ibc_msg = IbcMsg::Transfer {
        channel_id: ibc_config.channel_id,
        to_address: to_address.clone(),
        amount: ibc_coin.clone(),
        timeout,
    };

    Ok(ibc_msg)
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
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    amount: Uint128,
) -> ContractResult<Response> {
    let config = CONFIG.load(deps.storage)?;

    check_stopped(&config)?;

    let mut state = STATE.load(deps.storage)?;
    ensure!(
        amount >= config.minimum_liquid_stake_amount,
        ContractError::MinimumLiquidStakeAmount {
            minimum_stake_amount: (config.minimum_liquid_stake_amount),
            sent_amount: (amount)
        }
    );

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

    // TODO: Confirm Uint128 to String conversion is ok (proto requires this)
    //       Needs testing and validation - also need to check mint_to_address
    //
    // Mint liquid staking token
    let mint_msg = MsgMint {
        sender: env.contract.address.to_string(),
        amount: Some(Coin {
            denom: config.liquid_stake_token_denom,
            amount: mint_amount.to_string(),
        }),
        mint_to_address: info.sender.to_string(),
    };

    // Transfer native token to multisig address
    let ibc_msg = transfer_stake_msg(deps.as_ref(), env, amount)?;

    state.total_native_token += amount;
    state.total_liquid_stake_token += mint_amount;
    STATE.save(deps.storage, &state)?;

    // Ensure the state is properly setup to handle a reply from the ibc_message
    let ibc_config = IBC_CONFIG.load(deps.storage)?;
    save_forward_reply_state(
        deps,
        ForwardMsgReplyState {
            channel_id: ibc_config.channel_id,
            to_address: config.multisig_address_config.staker_address.to_string(),
            amount: Uint128::from(amount).into(),
            denom: config.native_token_denom,
        },
    )?;

    Ok(Response::new()
        .add_message(mint_msg)
        .add_message(ibc_msg)
        .add_attribute("action", "liquid_stake")
        .add_attribute("sender", info.sender.to_string())
        .add_attribute("amount", amount))
}

pub fn execute_liquid_unstake(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    amount: Uint128,
) -> ContractResult<Response> {
    let config = CONFIG.load(deps.storage)?;

    check_stopped(&config)?;

    STATE.load(deps.storage)?;

    // TODO: lets discuss, added minimum_liquid_stake_amount as a placeholder
    // Do we want to add a minimum unstake amount? As time goes on the stake and unstake amounts will diverge
    ensure!(
        amount >= config.minimum_liquid_stake_amount,
        ContractError::MinimumLiquidStakeAmount {
            minimum_stake_amount: (config.minimum_liquid_stake_amount),
            sent_amount: (amount)
        }
    );
    // Load current pending batch
    let mut pending_batch: Batch = BATCHES
        .range(deps.storage, None, None, Order::Descending)
        .find(|r| r.is_ok() && r.as_ref().unwrap().1.status == BatchStatus::Pending)
        .unwrap()
        .unwrap()
        .1;

    // Add unstake request to pending batch
    match pending_batch
        .liquid_unstake_requests
        .get_mut(&info.sender.to_string())
    {
        Some(request) => {
            request.shares += amount;
        }
        None => {
            pending_batch.liquid_unstake_requests.insert(
                info.sender.to_string(),
                LiquidUnstakeRequest::new(info.sender.clone(), amount),
            );
        }
    }

    // Add amount to batch total (stTIA)
    pending_batch.batch_total_liquid_stake += amount;

    BATCHES.save(deps.storage, pending_batch.id, &pending_batch)?;

    // let mut msgs: Vec<CosmosMsg> = vec![];
    // if batch period has elapsed, submit batch
    // for simplicity not doing this for now
    // if let Some(est_next_batch_action) = pending_batch.next_batch_action_time {
    //     if est_next_batch_action >= env.block.time.seconds() {
    //         msgs.push(CosmosMsg::Wasm(WasmMsg::Execute {
    //             contract_addr: env.contract.address.to_string(),
    //             msg: to_binary(&ExecuteMsg::SubmitBatch {
    //                 batch_id: pending_batch.id,
    //             })?,
    //             funds: vec![],
    //         }))
    //     }

    //     // Save updated pending batch
    //     PENDING_BATCH.save(deps.storage, &pending_batch)?;
    // }

    Ok(Response::new()
        .add_attribute("action", "liquid_unstake")
        .add_attribute("sender", info.sender.to_string())
        .add_attribute("batch", pending_batch.id.to_string())
        .add_attribute("amount", amount))
    // .add_messages(msgs))
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
    if batch.liquid_unstake_requests.is_empty() {
        return Err(ContractError::BatchEmpty {});
    }

    let config = CONFIG.load(deps.storage)?;
    let mut state = STATE.load(deps.storage)?;

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
            denom: config.liquid_stake_token_denom,
            amount: batch.batch_total_liquid_stake.to_string(),
        }),
        burn_from_address: env.contract.address.to_string(),
    };

    // TODO: Circuit break?
    // Need to add a test for this
    ensure!(
        state.total_liquid_stake_token >= batch.batch_total_liquid_stake,
        ContractError::InvalidUnstakeAmount {
            total_liquid_stake_token: (state.total_liquid_stake_token),
            amount_to_unstake: (batch.batch_total_liquid_stake)
        }
    );

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

    Ok(Response::new()
        .add_message(tokenfactory_burn_msg)
        .add_attribute("action", "submit_batch")
        .add_attribute("batch_id", batch.id.to_string())
        .add_attribute("batch_total", batch.batch_total_liquid_stake)
        .add_attribute("expected_native_unstaked", unbond_amount))
}

// doing a "push over pool" pattern for now
// eventually we can move this to auto-withdraw all funds upon batch completion
// Reasoning - any one issue in the batch will cause the entire batch to fail
pub fn execute_withdraw(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    batch_id: u64,
) -> ContractResult<Response> {
    let config: Config = CONFIG.load(deps.storage)?;

    check_stopped(&config)?;

    println!("{}", batch_id);

    let _batch = BATCHES.load(deps.storage, batch_id);
    if _batch.is_err() {
        return Err(ContractError::BatchEmpty {});
    }
    let batch = _batch.unwrap();

    println!("x");

    if batch.status != BatchStatus::Received {
        return Err(ContractError::BatchNotReady {
            actual: batch.status as u64,
            expected: BatchStatus::Received as u64,
        });
    }

    let _liquid_unstake_request: Option<LiquidUnstakeRequest> = batch
        .liquid_unstake_requests
        .get(&info.sender.to_string())
        .cloned();

    if _liquid_unstake_request.is_none() {
        return Err(ContractError::NoRequestInBatch {});
    }

    let mut liquid_unstake_request = _liquid_unstake_request.unwrap();

    if liquid_unstake_request.redeemed {
        return Err(ContractError::AlreadyRedeemed {});
    }

    liquid_unstake_request.redeemed = true;
    BATCHES.save(deps.storage, batch.id, &batch)?;

    // TODO make this a share of total liquid stake? in case of slashes?
    // let total_shares = batch.batch_total_liquid_stake;
    let amount = liquid_unstake_request.shares;

    let send_msg = MsgSend {
        from_address: env.contract.address.to_string(),
        to_address: info.sender.to_string(),
        amount: vec![Coin {
            denom: config.native_token_denom,
            amount: amount.to_string(),
        }],
    };

    Ok(Response::new()
        .add_attribute("action", "execute_withdraw")
        .add_attribute("batch", batch.id.to_string())
        .add_attribute("amount", amount.to_string())
        .add_message(send_msg))
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
    let new_validator_addr = validate_address(new_validator.clone(), CELESTIA_VALIDATOR_PREFIX)?;

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
        validate_address(validator_to_remove.clone(), CELESTIA_VALIDATOR_PREFIX)?;

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
pub fn execute_transfer_ownership(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    new_owner: String,
) -> ContractResult<Response> {
    ADMIN.assert_admin(deps.as_ref(), &info.sender)?;

    let mut state = STATE.load(deps.storage)?;
    state.pending_owner = Some(deps.api.addr_validate(&new_owner)?);

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

    STATE.save(deps.storage, &state)?;

    Ok(Response::new().add_attribute("action", "revoke_ownership_transfer"))
}

pub fn execute_accept_ownership(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
) -> ContractResult<Response> {
    let new_owner = {
        let mut state = STATE.load(deps.storage)?;
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

pub fn recover(deps: DepsMut, env: Env, info: MessageInfo) -> ContractResult<Response> {
    // TODO: retry pending transfers
    let recoveries = RECOVERY_STATES.load(deps.storage, &info.sender)?;
    // Remove the recoveries from the store. If the sends fail, the whole tx should be reverted.
    RECOVERY_STATES.remove(deps.storage, &info.sender);
    let ibc_config = IBC_CONFIG.load(deps.storage)?;
    let timeout = IbcTimeout::with_timestamp(Timestamp::from_nanos(
        env.block.time.nanos() + ibc_config.default_timeout.nanos(),
    ));

    let msgs = recoveries.into_iter().map(|r| IbcMsg::Transfer {
        channel_id: r.channel_id,
        to_address: r.recovery_addr.into_string(),
        amount: cosmwasm_std::Coin {
            denom: r.denom,
            amount: Uint128::from(r.amount),
        },
        timeout: timeout.clone(),
    });

    Ok(Response::new()
        .add_attribute("action", "recover") // TODO: is that ok setting the action name 'recover' ?
        .add_messages(msgs))
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
    reserve_token: Option<String>,
    channel_id: Option<String>,
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
    if channel_id.is_some() && reserve_token.is_none() {
        return Err(ContractError::IbcChannelNotFound {});
    }

    let channel_regexp = regex::Regex::new(r"^channel-[0-9]+$").unwrap();
    if channel_id.is_some() && !channel_regexp.is_match(&channel_id.clone().unwrap()) {
        return Err(ContractError::IbcChannelNotFound {});
    }
    let ibc_token_regexp = regex::Regex::new(r"^ibc/[A-Z0-9]{64}$").unwrap();
    if reserve_token.is_some() && !ibc_token_regexp.is_match(&reserve_token.clone().unwrap()) {
        return Err(ContractError::IbcChannelNotFound {});
    }
    if reserve_token.is_some() && channel_id.is_none()
        || channel_id.is_some() && reserve_token.is_none()
    {
        return Err(ContractError::IbcChannelNotFound {});
    }
    if reserve_token.is_some() && channel_id.is_some() {
        config.ibc_channel_id = channel_id.unwrap();
        config.native_token_denom = reserve_token.unwrap();
    }

    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new().add_attribute("action", "update_config"))
}

pub fn receive_rewards(deps: DepsMut, env: Env, info: MessageInfo) -> ContractResult<Response> {
    let config: Config = CONFIG.load(deps.storage)?;

    check_stopped(&config)?;

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
        return Err(ContractError::FormatError {});
    }
    let amount_after_fees = amount_after_fees.unwrap();

    // update the accounting of tokens
    let mut state = STATE.load(deps.storage)?;

    state.total_native_token += amount_after_fees.clone();
    state.total_reward_amount += amount.clone();
    state.total_fees += fee;

    STATE.save(deps.storage, &state)?;

    // transfer the funds to Celestia to be staked
    let ibc_transfer_msg = transfer_stake_msg(deps.as_ref(), env, amount_after_fees.clone())?;

    Ok(Response::new()
        .add_attribute("method", "receive_rewards")
        .add_attribute("method", "transfer_stake")
        .add_attribute("amount", amount)
        .add_message(ibc_transfer_msg))
}

pub fn receive_unstaked_tokens(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
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

    // get the oldest submitted batch
    let _batch: Option<Batch> = BATCHES
        .range(deps.storage, None, None, Order::Ascending)
        .find(|r| r.is_ok() && r.as_ref().unwrap().1.status == BatchStatus::Submitted)
        .map(|r| r.unwrap().1);

    if _batch.is_none() {
        return Err(ContractError::BatchEmpty {});
    }

    let mut batch = _batch.unwrap();

    if batch.next_batch_action_time.is_none() {
        return Err(ContractError::BatchNotReady {
            actual: env.block.time.seconds(),
            expected: 0,
        });
    }
    let next_batch_action_time = batch.next_batch_action_time.unwrap();
    if next_batch_action_time > env.block.time.seconds() {
        return Err(ContractError::BatchNotReady {
            actual: env.block.time.seconds(),
            expected: next_batch_action_time,
        });
    }

    batch.update_status(BatchStatus::Received, None);

    BATCHES.save(deps.storage, batch.id, &batch)?;

    Ok(Response::new()
        .add_attribute("method", "receive_unstaked_tokens")
        .add_attribute("amount", amount))
}

pub fn circuit_breaker(deps: DepsMut, _env: Env, info: MessageInfo) -> ContractResult<Response> {
    let sender = info.sender.to_string();

    let mut config: Config = CONFIG.load(deps.storage)?;

    if !config.operators.iter().any(|v| *v == sender) {
        return Err(ContractError::Unauthorized { sender });
    }

    config.stopped = true;
    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new().add_attribute("method", "circuit_breaker"))
}

pub fn resume_contract(deps: DepsMut, _env: Env, info: MessageInfo) -> ContractResult<Response> {
    ADMIN.assert_admin(deps.as_ref(), &info.sender)?;

    let mut config: Config = CONFIG.load(deps.storage)?;

    config.stopped = false;
    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new().add_attribute("method", "resume_contract"))
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

    let ForwardMsgReplyState {
        channel_id,
        to_address,
        amount,
        denom,
    } = FORWARD_REPLY_STATE.load(deps.storage)?;
    FORWARD_REPLY_STATE.remove(deps.storage);

    // If a recovery address was provided, store sent IBC transfer so that it
    // can later be recovered by that addr.
    let recovery = IBCTransfer {
        recovery_addr: Addr::unchecked(to_address.clone()),
        channel_id: channel_id.clone(),
        sequence: transfer_response.sequence,
        amount,
        denom: denom.clone(),
        status: PacketLifecycleStatus::Sent,
    };

    // Save as in-flight to be able to manipulate when the ack/timeout is received
    INFLIGHT_PACKETS.save(
        deps.storage,
        (&channel_id, transfer_response.sequence),
        &recovery,
    )?;

    let response = Response::new()
        .add_attribute("action", "handle_ibc_reply")
        .add_attribute("status", "ibc_message_successfully_submitted")
        .add_attribute("channel", &channel_id)
        .add_attribute("receiver", &to_address)
        .add_attribute(
            "packet_sequence",
            format!("{:?}", transfer_response.sequence),
        );

    Ok(response)
}

fn save_forward_reply_state(
    deps: DepsMut,
    forward_reply_state: ForwardMsgReplyState,
) -> Result<(), ContractError> {
    // Check that there isn't anything stored in FORWARD_REPLY_STATES. If there
    // is, it means that the contract is already waiting for a reply and should
    // not override the stored state. This should never happen here, but adding
    // the check for safety. If this happens there is likely a malicious attempt
    // modify the contract's state before it has replied.
    if FORWARD_REPLY_STATE.may_load(deps.storage)?.is_some() {
        return Err(ContractError::ContractLocked {
            msg: "Already waiting for a reply".to_string(),
        });
    }
    // Store the ibc send information and the user's failed delivery preference
    // so that it can be handled by the response
    FORWARD_REPLY_STATE.save(deps.storage, &forward_reply_state)?;
    Ok(())
}
