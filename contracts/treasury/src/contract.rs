#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_json_binary, to_json_string, Binary, Deps, DepsMut, Env, MessageInfo, Response,
};
use cw2::set_contract_version;

use crate::error::{ContractError, ContractResult};
use crate::execute::{
    execute_accept_ownership, execute_revoke_ownership_transfer, execute_spend_funds,
    execute_swap_exact_amount_in, execute_swap_exact_amount_out, execute_transfer_ownership,
    execute_update_config,
};
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::query::query_config;
use crate::state::{Config, State, ADMIN, CONFIG, STATE};

const CONTRACT_NAME: &str = env!("CARGO_PKG_NAME");
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    mut deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let admin = msg
        .admin
        .map(|admin_str| deps.api.addr_validate(&admin_str))
        .transpose()?
        .unwrap_or(info.sender.clone());
    ADMIN.set(deps.branch(), Some(admin))?;

    // Init State
    let state = State {
        pending_owner: None,
        owner_transfer_min_time: None,
    };
    STATE.save(deps.storage, &state)?;

    // Init Config
    let config = Config {
        trader: msg
            .trader
            .map(|trader_str| deps.api.addr_validate(&trader_str))
            .transpose()?
            .unwrap_or(info.sender.clone()),
        allowed_swap_routes: msg.allowed_swap_routes,
    };
    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new()
        .add_attribute("action", "instantiate")
        .add_attribute("owner", info.sender)
        .add_attribute("trader", config.trader)
        .add_attribute(
            "allowed_swap_routes",
            to_json_string(&config.allowed_swap_routes)?,
        ))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::TransferOwnership { new_owner } => {
            execute_transfer_ownership(deps, env, info, new_owner)
        }
        ExecuteMsg::AcceptOwnership {} => execute_accept_ownership(deps, env, info),
        ExecuteMsg::RevokeOwnershipTransfer {} => {
            execute_revoke_ownership_transfer(deps, env, info)
        }
        ExecuteMsg::SpendFunds {
            amount,
            receiver,
            channel_id,
        } => execute_spend_funds(deps, env, info, amount, receiver, channel_id),
        ExecuteMsg::SwapExactAmountIn {
            routes,
            token_in,
            token_out_min_amount,
        } => execute_swap_exact_amount_in(deps, env, info, routes, token_in, token_out_min_amount),
        ExecuteMsg::SwapExactAmountOut {
            routes,
            token_out,
            token_in_max_amount,
        } => execute_swap_exact_amount_out(deps, env, info, routes, token_out, token_in_max_amount),
        ExecuteMsg::UpdateConfig {
            trader,
            allowed_swap_routes,
        } => execute_update_config(deps, info, trader, allowed_swap_routes),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> ContractResult<Binary> {
    match msg {
        QueryMsg::Config {} => to_json_binary(&query_config(deps)?),
    }
    .map_err(ContractError::from)
}
