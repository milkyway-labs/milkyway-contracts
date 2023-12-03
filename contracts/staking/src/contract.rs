use crate::execute::{
    circuit_breaker, execute_submit_batch, fee_withdraw, handle_ibc_reply, receive_rewards,
    receive_unstaked_tokens, recover, resume_contract, update_config,
};
use crate::helpers::validate_addresses;
use crate::ibc::{receive_ack, receive_timeout};
use crate::query::{
    query_batch, query_batches, query_claimable, query_config, query_ibc_queue,
    query_pending_batch, query_reply_queue, query_state,
};
use crate::state::{
    Config, IbcConfig, MultisigAddressConfig, ProtocolFeeConfig, State, ADMIN, BATCHES, CONFIG,
    IBC_CONFIG, IBC_WAITING_FOR_REPLY, PENDING_BATCH_ID, STATE,
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
    entry_point, to_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response,
    StdError, StdResult, Uint128,
};
use cosmwasm_std::{CosmosMsg, Timestamp};
use cw2::set_contract_version;
use cw_utils::must_pay;
use milky_way::staking::Batch;
use osmosis_std::types::osmosis::tokenfactory::v1beta1::MsgCreateDenom;
use semver::Version;

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

    // TODO: determine if info.sender is the admin or if we want to pass in with msg
    ADMIN.set(deps.branch(), Some(info.sender.clone()))?;

    // validations
    let validators = validate_addresses(&msg.validators, CELESTIA_VALIDATOR_PREFIX)?;
    assert!(
        msg.liquid_stake_token_denom.len() > 3,
        "liquid_stake_token_denom is required"
    );
    assert!(
        msg.liquid_stake_token_denom
            .chars()
            .all(|c| c.is_ascii_alphabetic()),
        "liquid_stake_token_denom must be alphabetic"
    );

    // Init Config
    let config = Config {
        native_token_denom: "".to_string(),
        liquid_stake_token_denom: format!(
            "factory/{0}/{1}",
            env.contract.address, msg.liquid_stake_token_denom
        ),
        treasury_address: Addr::unchecked(""),
        operators: vec![],
        validators,
        batch_period: 0,
        unbonding_period: 0,
        protocol_fee_config: ProtocolFeeConfig {
            dao_treasury_fee: Uint128::zero(),
        },
        multisig_address_config: MultisigAddressConfig {
            staker_address: Addr::unchecked(""),
            reward_collector_address: Addr::unchecked(""),
        },
        minimum_liquid_stake_amount: Uint128::zero(),
        ibc_channel_id: "".to_string(),
        stopped: true, // we start stopped
    };

    CONFIG.save(deps.storage, &config)?;

    update_config(
        deps.branch(),
        env.clone(),
        info.clone(),
        Some(msg.batch_period),
        Some(msg.unbonding_period),
        Some(msg.minimum_liquid_stake_amount),
        Some(msg.multisig_address_config),
        Some(msg.protocol_fee_config),
        Some(msg.native_token_denom),
        Some(msg.ibc_channel_id.clone()),
        Some(msg.operators),
        Some(msg.treasury_address),
    )?;

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
        ExecuteMsg::LiquidStake {
            expected_mint_amount,
        } => {
            let payment = must_pay(&info, &config.native_token_denom)?;
            execute_liquid_stake(deps, env, info, payment, expected_mint_amount)
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
            treasury_address,
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
            treasury_address,
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
        ExecuteMsg::RecoverPendingIbcTransfers { paginated } => {
            recover(deps, env, info, paginated.unwrap_or(false))
        }
        ExecuteMsg::FeeWithdraw { amount } => fee_withdraw(deps, env, info, amount),
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
        QueryMsg::Batches {
            start_after,
            limit,
            status,
        } => to_binary(&query_batches(deps, start_after, limit, status)?),
        QueryMsg::PendingBatch {} => to_binary(&query_pending_batch(deps)?),
        QueryMsg::ClaimableBatches {
            user,
            start_after,
            limit,
        } => to_binary(&query_claimable(deps, user, start_after, limit)?),

        // dev only, depr
        QueryMsg::IbcQueue { start_after, limit } => {
            to_binary(&query_ibc_queue(deps, start_after, limit)?)
        }
        QueryMsg::IbcReplyQueue { start_after, limit } => {
            to_binary(&query_reply_queue(deps, start_after, limit)?)
        }
    }
}

///////////////
/// MIGRATE ///
///////////////

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(deps: DepsMut, _env: Env, _msg: MigrateMsg) -> Result<Response, ContractError> {
    let current_version = cw2::get_contract_version(deps.storage)?;
    if &CONTRACT_NAME != &current_version.contract.as_str() {
        return Err(StdError::generic_err("Cannot upgrade to a different contract").into());
    }

    let version: Version = current_version
        .version
        .parse()
        .map_err(|_| StdError::generic_err("Invalid contract version"))?;
    let new_version: Version = CONTRACT_VERSION
        .parse()
        .map_err(|_| StdError::generic_err("Invalid contract version"))?;

    // current version not launchpad v2
    if version > new_version {
        return Err(StdError::generic_err("Cannot upgrade to a previous contract version").into());
    }
    // if same version return
    if version == new_version {
        return Err(StdError::generic_err("Cannot migrate to the same version.").into());
    }

    // migrate data
    // NONE currently

    // set new contract version
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
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
