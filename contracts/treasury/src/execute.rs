use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, Timestamp};

use crate::{
    error::{ContractError, ContractResult},
    state::{State, ADMIN, STATE},
};

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
