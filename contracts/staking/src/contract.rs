use crate::execute::{
    circuit_breaker, execute_submit_batch, fee_withdraw, handle_ibc_reply, receive_rewards,
    receive_unstaked_tokens, recover, resume_contract, slash_batches, update_config,
};
use crate::helpers::validate_denom;
use crate::ibc::{receive_ack, receive_timeout};
use crate::migrations;
use crate::query::{
    query_all_unstake_requests, query_all_unstake_requests_v2, query_batch, query_batches,
    query_batches_by_ids, query_config, query_ibc_queue, query_pending_batch, query_reply_queue,
    query_state, query_unstake_requests,
};
use crate::state::{
    assert_not_migrating, Config, State, ADMIN, BATCHES, CONFIG, IBC_WAITING_FOR_REPLY, MIGRATING,
    PENDING_BATCH_ID, STATE,
};
use crate::{
    error::ContractError,
    execute::{
        execute_accept_ownership, execute_add_validator, execute_liquid_stake,
        execute_liquid_unstake, execute_remove_validator, execute_revoke_ownership_transfer,
        execute_transfer_ownership, execute_withdraw,
    },
    msg::{ExecuteMsg, IBCLifecycleComplete, InstantiateMsg, MigrateMsg, QueryMsg, SudoMsg},
    tokenfactory,
};
use cosmwasm_std::Timestamp;
use cosmwasm_std::{
    entry_point, to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response,
    StdError, StdResult, Uint128,
};
use cw2::set_contract_version;
use cw_utils::must_pay;
use milky_way::staking::Batch;
use milky_way::utils::validate_addresses;
use semver::Version;

// Version information for migration
pub const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const IBC_TIMEOUT: Timestamp = Timestamp::from_nanos(1000000000000); // TODO: Placeholder value for IBC timeout

///////////////////
/// INSTANTIATE ///
///////////////////

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    mut deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let admin = info.sender.clone();
    ADMIN.set(deps.branch(), Some(admin.clone()))?;

    // Init Config
    let native_chain_config = msg.native_chain_config.validate()?;
    let protocol_chain_config = msg.protocol_chain_config.validate()?;
    let protocol_fee_config = msg.protocol_fee_config.validate(&protocol_chain_config)?;

    // Ensure the batch period is lower then unbonding period.
    if msg.batch_period > native_chain_config.unbonding_period {
        return Err(ContractError::ValueTooBig {
            field_name: "batch_period".to_string(),
            value: Uint128::from(msg.batch_period),
            max: Uint128::from(native_chain_config.unbonding_period),
        });
    }

    let config = Config {
        native_chain_config,
        protocol_chain_config,
        protocol_fee_config,
        liquid_stake_token_denom: format!(
            "factory/{0}/{1}",
            env.contract.address,
            validate_denom(&msg.liquid_stake_token_denom)?
        ),
        monitors: validate_addresses(
            &msg.monitors,
            &msg.protocol_chain_config.account_address_prefix,
        )?,
        batch_period: msg.batch_period,
        stopped: true, // we start stopped
    };
    CONFIG.save(deps.storage, &config)?;

    // Init State
    let state = State {
        total_native_token: Uint128::zero(),
        total_liquid_stake_token: Uint128::zero(),
        pending_owner: None,
        total_reward_amount: Uint128::zero(),
        total_fees: Uint128::zero(),
        ibc_id_counter: 0,
        rate: 1u128.into(),
        owner_transfer_min_time: None,
    };

    STATE.save(deps.storage, &state)?;

    // Create liquid stake token denom
    let cosmos_tokenfactory_msg = tokenfactory::create_denom(
        env.contract.address.to_string(),
        msg.liquid_stake_token_denom,
    )?;

    let pending_batch = Batch::new(
        1,
        Uint128::zero(),
        env.block.time.seconds() + config.batch_period,
    );

    // Set pending batch and batches
    BATCHES.save(deps.storage, 1, &pending_batch)?;
    PENDING_BATCH_ID.save(deps.storage, &1)?;

    // TODO: Update attributes
    Ok(Response::new()
        .add_attribute("action", "instantiate")
        .add_attribute("owner", admin)
        .add_message(cosmos_tokenfactory_msg))
}

///////////////
/// EXECUTE ///
///////////////

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    assert_not_migrating(deps.as_ref())?;

    let config = CONFIG.load(deps.storage)?;
    match msg {
        ExecuteMsg::LiquidStake {
            mint_to,
            transfer_to_native_chain,
            expected_mint_amount,
        } => {
            let payment = must_pay(&info, &config.protocol_chain_config.ibc_token_denom)?;
            execute_liquid_stake(
                deps,
                env,
                info,
                payment,
                mint_to,
                transfer_to_native_chain,
                expected_mint_amount,
            )
        }
        ExecuteMsg::LiquidUnstake {} => {
            let payment = must_pay(&info, &config.liquid_stake_token_denom)?;
            execute_liquid_unstake(deps, env, info, payment)
        }
        ExecuteMsg::SubmitBatch {} => execute_submit_batch(deps, env, info),
        ExecuteMsg::Withdraw { batch_id } => execute_withdraw(deps, env, info, batch_id),
        ExecuteMsg::AddValidator { new_validator } => {
            execute_add_validator(deps, env, info, new_validator)
        }
        ExecuteMsg::RemoveValidator { validator } => {
            execute_remove_validator(deps, env, info, validator)
        }
        ExecuteMsg::TransferOwnership { new_owner } => {
            execute_transfer_ownership(deps, env, info, new_owner)
        }
        ExecuteMsg::AcceptOwnership {} => execute_accept_ownership(deps, env, info),
        ExecuteMsg::RevokeOwnershipTransfer {} => {
            execute_revoke_ownership_transfer(deps, env, info)
        }
        ExecuteMsg::UpdateConfig {
            native_chain_config,
            protocol_chain_config,
            protocol_fee_config,
            monitors,
            batch_period,
        } => update_config(
            deps,
            env,
            info,
            native_chain_config,
            protocol_chain_config,
            protocol_fee_config,
            monitors,
            batch_period,
        ),
        ExecuteMsg::ReceiveRewards {} => receive_rewards(deps, env, info),
        ExecuteMsg::ReceiveUnstakedTokens { batch_id } => {
            receive_unstaked_tokens(deps, env, info, batch_id)
        }
        ExecuteMsg::CircuitBreaker {} => circuit_breaker(deps, env, info),
        ExecuteMsg::ResumeContract {
            total_native_token,
            total_liquid_stake_token,
            total_reward_amount,
        } => resume_contract(
            deps,
            env,
            info,
            total_native_token,
            total_liquid_stake_token,
            total_reward_amount,
        ),
        ExecuteMsg::SlashBatches { new_amounts } => slash_batches(deps, info, new_amounts),
        ExecuteMsg::RecoverPendingIbcTransfers {
            paginated,
            selected_packets,
            receiver,
        } => recover(
            deps,
            env,
            info,
            selected_packets,
            receiver,
            paginated.unwrap_or(false),
        ),
        ExecuteMsg::FeeWithdraw { amount } => fee_withdraw(deps, env, info, amount),
    }
}

/////////////
/// QUERY ///
/////////////

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_json_binary(&query_config(deps)?),
        QueryMsg::State {} => to_json_binary(&query_state(deps)?),
        QueryMsg::Batch { id } => to_json_binary(&query_batch(deps, id)?),
        QueryMsg::Batches {
            start_after,
            limit,
            status,
        } => to_json_binary(&query_batches(deps, start_after, limit, status)?),
        QueryMsg::BatchesByIds { ids } => to_json_binary(&query_batches_by_ids(deps, ids)?),
        QueryMsg::PendingBatch {} => to_json_binary(&query_pending_batch(deps)?),
        QueryMsg::UnstakeRequests { user } => {
            to_json_binary(&query_unstake_requests(deps, user.into_string())?)
        }
        // DEPR
        QueryMsg::AllUnstakeRequests { start_after, limit } => {
            to_json_binary(&query_all_unstake_requests(deps, start_after, limit)?)
        }
        QueryMsg::AllUnstakeRequestsV2 { start_after, limit } => {
            to_json_binary(&query_all_unstake_requests_v2(deps, start_after, limit)?)
        }

        // dev only, depr
        QueryMsg::IbcQueue { start_after, limit } => {
            to_json_binary(&query_ibc_queue(deps, start_after, limit)?)
        }
        QueryMsg::IbcReplyQueue { start_after, limit } => {
            to_json_binary(&query_reply_queue(deps, start_after, limit)?)
        }
    }
}

///////////////
/// MIGRATE ///
///////////////

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(mut deps: DepsMut, env: Env, msg: MigrateMsg) -> Result<Response, ContractError> {
    let current_version = cw2::get_contract_version(deps.storage)?;
    if CONTRACT_NAME != current_version.contract.as_str() {
        return Err(StdError::generic_err("Cannot upgrade to a different contract").into());
    }

    let version: Version = current_version
        .version
        .parse()
        .map_err(|_| StdError::generic_err("Invalid contract version"))?;
    let new_version: Version = CONTRACT_VERSION
        .parse()
        .map_err(|_| StdError::generic_err("Invalid contract version"))?;

    // Prevent downgrade
    if version > new_version {
        return Err(StdError::generic_err("Cannot upgrade to a previous contract version").into());
    }
    // if same version return
    if version == new_version {
        let is_migrating = MIGRATING.may_load(deps.storage)?.unwrap_or(false);
        if !is_migrating {
            return Err(StdError::generic_err("Cannot migrate to the same version.").into());
        }
    }

    let migration_response = match msg {
        MigrateMsg::V0_4_18ToV0_4_20 {
            send_fees_to_treasury,
        } => migrations::v0_4_20::migrate(deps.branch(), env, send_fees_to_treasury)?,
        MigrateMsg::V0_4_20ToV1_0_0 {
            native_account_address_prefix,
            native_validator_address_prefix,
            native_token_denom,
            protocol_account_address_prefix,
        } => migrations::v1_0_0::migrate(
            deps.branch(),
            env,
            native_account_address_prefix,
            native_validator_address_prefix,
            native_token_denom,
            protocol_account_address_prefix,
        )?,
        MigrateMsg::V1_0_0ToV1_1_0 { limit } => {
            migrations::v1_1_0::migrate(deps.branch(), env, limit)?
        }
    };

    Ok(migration_response)
}

/////////////
/// SUDO  ///
/////////////

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn sudo(deps: DepsMut, _env: Env, msg: SudoMsg) -> Result<Response, ContractError> {
    assert_not_migrating(deps.as_ref())?;

    match msg {
        SudoMsg::IBCLifecycleComplete(IBCLifecycleComplete::IBCAck {
            channel,
            sequence,
            ack,
            success,
        }) => receive_ack(deps, channel, sequence, ack, success),
        SudoMsg::IBCLifecycleComplete(IBCLifecycleComplete::IBCTimeout { channel, sequence }) => {
            receive_timeout(deps, channel, sequence)
        }
    }
}

/////////////
/// REPLY ///
/////////////

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, reply: Reply) -> Result<Response, ContractError> {
    assert_not_migrating(deps.as_ref())?;

    let ibc_waiting_result = IBC_WAITING_FOR_REPLY.load(deps.storage, reply.id);
    match ibc_waiting_result {
        Ok(_ibc_waiting_for_reply) => handle_ibc_reply(deps, reply),
        Err(_) => Err(ContractError::InvalidReplyID { id: reply.id }),
    }
}
