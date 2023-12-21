use cosmwasm_std::{CosmosMsg, DepsMut, Env, MessageInfo, Response, Timestamp};
use osmosis_std::types::{
    cosmos::base::v1beta1::Coin, ibc::applications::transfer::v1::MsgTransfer,
};

use crate::{
    error::{ContractError, ContractResult},
    state::{State, ADMIN, STATE},
};

pub const IBC_TIMEOUT: Timestamp = Timestamp::from_nanos(1000000000000); // TODO: Placeholder value for IBC timeout

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

pub fn execute_spend_funds(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    amount: cosmwasm_std::Coin,
    receiver: String,
    channel_id: Option<String>,
) -> ContractResult<Response> {
    ADMIN.assert_admin(deps.as_ref(), &info.sender)?;

    let msg_send: CosmosMsg;

    if channel_id.is_none() {
        msg_send = cosmwasm_std::BankMsg::Send {
            to_address: receiver.clone(),
            amount: vec![amount.clone()],
        }
        .into();
    } else {
        // not using the ibc queue here. if this fails, we just reexecute
        msg_send = MsgTransfer {
            source_channel: channel_id.unwrap().clone(),
            source_port: "transfer".to_string(),
            token: Some(Coin {
                denom: amount.denom,
                amount: amount.amount.to_string(),
            }),
            receiver: receiver.clone(),
            sender: env.contract.address.to_string(),
            timeout_height: None,
            timeout_timestamp: env.block.time.nanos() + IBC_TIMEOUT.nanos(),
            memo: format!(
                "{{\"ibc_callback\":\"{}\"}}",
                env.contract.address.to_string()
            ),
        }
        .into();
    }

    let res = Response::new()
        .add_message(msg_send)
        .add_attribute("action", "spend_funds")
        .add_attribute("receiver", receiver.clone())
        .add_attribute("amount", amount.amount)
        .add_attribute("denom", amount.denom);

    if channel_id.is_some() {
        res.add_attribute("channel_id", channel_id.unwrap().clone());
    }

    Ok(res)
}
