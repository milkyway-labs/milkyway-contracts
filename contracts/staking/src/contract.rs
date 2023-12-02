use crate::execute::{
    circuit_breaker, execute_submit_batch, handle_ibc_reply, receive_rewards,
    receive_unstaked_tokens, recover, resume_contract, update_config,
};
use crate::helpers::{validate_address, validate_addresses};
use crate::ibc::{receive_ack, receive_timeout};
use crate::query::{
    query_batch, query_batches, query_claimable, query_config, query_ibc_queue,
    query_pending_batch, query_reply_queue, query_state,
};
use crate::state::{
    Config, IbcConfig, State, ADMIN, BATCHES, CONFIG, IBC_CONFIG, IBC_WAITING_FOR_REPLY,
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
};
use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdResult,
    Uint128,
};
use cosmwasm_std::{CosmosMsg, Timestamp};
use cw2::set_contract_version;
use cw_utils::must_pay;
use milky_way::staking::Batch;
use osmosis_std::types::osmosis::tokenfactory::v1beta1::MsgCreateDenom;

// Version information for migration
const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const IBC_TIMEOUT: Timestamp = Timestamp::from_nanos(1000000000000); // TODO: Placeholder value for IBC timeout

pub const CELESTIA_ACCOUNT_PREFIX: &str = &"celestia";
pub const OSMOSIS_ACCOUNT_PREFIX: &str = &"osmo";
pub const CELESTIA_VALIDATOR_PREFIX: &str = &"celestiavaloper";

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
    let operators = validate_addresses(&msg.operators, OSMOSIS_ACCOUNT_PREFIX)?;
    let validators = validate_addresses(&msg.validators, CELESTIA_VALIDATOR_PREFIX)?;

    // TODO: determine if info.sender is the admin or if we want to pass in with msg
    ADMIN.set(deps.branch(), Some(info.sender.clone()))?;

    if msg.ibc_channel_id == "" {
        return Err(ContractError::ConfigWrong {});
    }

    if msg.native_token_denom == "" {
        return Err(ContractError::ConfigWrong {});
    }

    validate_address(
        &msg.multisig_address_config
            .reward_collector_address
            .to_string(),
        &CELESTIA_ACCOUNT_PREFIX,
    )?;
    validate_address(
        &msg.multisig_address_config.staker_address.to_string(),
        &CELESTIA_ACCOUNT_PREFIX,
    )?;

    // Init Config
    let config = Config {
        native_token_denom: msg.native_token_denom,
        liquid_stake_token_denom: format!(
            "factory/{0}/{1}",
            env.contract.address, msg.liquid_stake_token_denom
        ), //TODO determine the format to save in
        treasury_address: deps.api.addr_validate(&msg.treasury_address)?,
        operators,
        validators,
        batch_period: msg.batch_period,
        unbonding_period: msg.unbonding_period,
        protocol_fee_config: msg.protocol_fee_config,
        multisig_address_config: msg.multisig_address_config,
        minimum_liquid_stake_amount: msg.minimum_liquid_stake_amount,
        ibc_channel_id: msg.ibc_channel_id.clone(),
        stopped: false,
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
    };

    STATE.save(deps.storage, &state)?;

    // Create liquid stake token denom
    let tokenfactory_msg = MsgCreateDenom {
        sender: env.contract.address.to_string(),
        subdenom: msg.liquid_stake_token_denom,
    };

    let cosmos_tokenfactory_msg: CosmosMsg = tokenfactory_msg.into();

    let pending_batch = Batch::new(
        1,
        Uint128::zero(),
        env.block.time.seconds() + config.batch_period,
    );

    // Set pending batch and batches
    BATCHES.save(deps.storage, 1, &pending_batch)?;
    PENDING_BATCH_ID.save(deps.storage, &1)?;

    let ibc_config = IbcConfig {
        channel_id: msg.ibc_channel_id.clone(),
        default_timeout: IBC_TIMEOUT,
    };
    IBC_CONFIG.save(deps.storage, &ibc_config)?;

    // TODO: Update attributes
    Ok(Response::new()
        .add_attribute("action", "instantiate")
        .add_attribute("owner", info.sender)
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
    let config = CONFIG.load(deps.storage)?;
    match msg {
        ExecuteMsg::LiquidStake {} => {
            let payment = must_pay(&info, &config.native_token_denom)?;
            execute_liquid_stake(deps, env, info, payment)
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
            batch_period,
            unbonding_period,
            minimum_liquid_stake_amount,
            multisig_address_config,
            protocol_fee_config,
            reserve_token,
            channel_id,
            operators,
        } => update_config(
            deps,
            env,
            info,
            batch_period,
            unbonding_period,
            minimum_liquid_stake_amount,
            multisig_address_config,
            protocol_fee_config,
            reserve_token,
            channel_id,
            operators,
        ),
        ExecuteMsg::ReceiveRewards {} => receive_rewards(deps, env, info),
        ExecuteMsg::ReceiveUnstakedTokens { batch_id } => {
            receive_unstaked_tokens(deps, env, info, batch_id)
        }
        ExecuteMsg::CircuitBreaker {} => circuit_breaker(deps, env, info),
        ExecuteMsg::ResumeContract {} => resume_contract(deps, env, info),
        ExecuteMsg::RecoverPendingIbcTransfers {} => recover(deps, env, info),
    }
}

/////////////
/// QUERY ///
/////////////

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_binary(&query_config(deps)?),
        QueryMsg::State {} => to_binary(&query_state(deps)?),
        QueryMsg::Batch { id } => to_binary(&query_batch(deps, id)?),
        QueryMsg::Batches {} => to_binary(&query_batches(deps)?),
        QueryMsg::PendingBatch {} => to_binary(&query_pending_batch(deps)?),
        QueryMsg::ClaimableBatches { user } => to_binary(&query_claimable(deps, user)?),

        // dev only, depr
        QueryMsg::IbcQueue {} => to_binary(&query_ibc_queue(deps)?),
        QueryMsg::IbcReplyQueue {} => to_binary(&query_reply_queue(deps)?),
    }
}

///////////////
/// MIGRATE ///
///////////////

#[entry_point]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    // TODO: note implement yet
    Ok(Response::new())
}

/////////////
/// SUDO  ///
/////////////

#[cfg_attr(not(feature = "imported"), entry_point)]
pub fn sudo(deps: DepsMut, _env: Env, msg: SudoMsg) -> Result<Response, ContractError> {
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

#[cfg_attr(not(feature = "imported"), entry_point)]
pub fn reply(deps: DepsMut, _env: Env, reply: Reply) -> Result<Response, ContractError> {
    let ibc_waiting_result = IBC_WAITING_FOR_REPLY.load(deps.storage, reply.id);
    match ibc_waiting_result {
        Ok(_ibc_waiting_for_reply) => handle_ibc_reply(deps, reply),
        Err(_) => Err(ContractError::InvalidReplyID { id: reply.id }),
    }
}
