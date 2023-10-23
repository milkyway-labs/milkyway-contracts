#[cfg(not(feature = "library"))]
use crate::{
    error::ContractError,
    execute::{
        execute_accept_ownership, execute_add_validator, execute_claim, execute_liquid_stake,
        execute_liquid_unstake, execute_remove_validator, execute_transfer_ownership,
    },
    msg::{ExecuteMsg, InstantiateMsg, QueryMsg},
};
use cosmwasm_std::{entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:staking";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

///////////////////
/// INSTANTIATE ///
///////////////////

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    unimplemented!()
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
    match msg {
        ExecuteMsg::LiquidStake { coin } => execute_liquid_stake(deps, env, info, coin),
        ExecuteMsg::LiquidUnstake { coin } => execute_liquid_unstake(deps, env, info, coin),
        ExecuteMsg::Claim {} => execute_claim(deps, env, info),
        ExecuteMsg::TransferOwnership { new_owner } => {
            execute_transfer_ownership(deps, env, info, new_owner)
        }
        ExecuteMsg::AcceptOwnership {} => execute_accept_ownership(deps, env, info),
        ExecuteMsg::AddValidator { new_validator } => {
            execute_add_validator(deps, env, info, new_validator)
        }
        ExecuteMsg::RemoveValidator { validator } => {
            execute_remove_validator(deps, env, info, validator)
        }
    }
}

/////////////
/// QUERY ///
/////////////

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    unimplemented!()
}

#[cfg(test)]
mod tests {}
