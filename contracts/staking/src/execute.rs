use crate::error::{ContractError, ContractResult};
use cosmwasm_std::{Addr, Coin, DepsMut, Env, MessageInfo, Response};

use crate::state::{ADMIN, STATE};
use cw_controllers::Admin;

pub fn execute_liquid_stake(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    coin: Coin,
) -> ContractResult<Response> {
    unimplemented!()
}

pub fn execute_liquid_unstake(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    coin: Coin,
) -> ContractResult<Response> {
    unimplemented!()
}

pub fn execute_claim(deps: DepsMut, env: Env, info: MessageInfo) -> ContractResult<Response> {
    unimplemented!()
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

pub fn execute_add_validator(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    new_validator: String,
) -> ContractResult<Response> {
    unimplemented!()
}

pub fn execute_remove_validator(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    validator: String,
) -> ContractResult<Response> {
    unimplemented!()
}
