use cosmwasm_std::{
    attr, to_json_string, CosmosMsg, DepsMut, Env, MessageInfo, Response, Timestamp,
};
use osmosis_std::types::{
    cosmos::base::v1beta1::Coin, ibc::applications::transfer::v1::MsgTransfer,
};

use crate::{
    error::{ContractError, ContractResult},
    helpers::validate_address,
    state::{State, SwapRoute, ADMIN, CONFIG, STATE},
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
        validate_address(&receiver, "osmo")?;
        msg_send = cosmwasm_std::BankMsg::Send {
            to_address: receiver.clone(),
            amount: vec![amount.clone()],
        }
        .into();
    } else {
        validate_address(&receiver, "celestia")?;
        // not using the ibc queue here. if this fails, we just reexecute
        msg_send = MsgTransfer {
            source_channel: channel_id.clone().unwrap().clone(),
            source_port: "transfer".to_string(),
            token: Some(Coin {
                denom: amount.clone().denom,
                amount: amount.clone().amount.to_string(),
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

    let mut attributes = vec![
        attr("action", "spend_funds"),
        attr("receiver", receiver.clone()),
        attr("amount", amount.clone().amount),
        attr("denom", amount.clone().denom.clone()),
    ];

    if channel_id.clone().is_some() {
        attributes.push(attr("channel_id", channel_id.unwrap().clone()));
    }

    let res = Response::new()
        .add_message(msg_send)
        .add_attributes(attributes);
    Ok(res)
}

pub fn execute_swap_exact_amount_in(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    swap_routes: Vec<SwapRoute>,
    token_in: cosmwasm_std::Coin,
    token_out_min_amount: u128,
) -> ContractResult<Response> {
    let config = CONFIG.load(deps.storage)?;

    config.assert_trader(&info.sender)?;
    swap_routes
        .iter()
        .try_for_each(|swap_route| config.assert_allowed_swap_route(swap_route))?;

    let message = osmosis_std::types::osmosis::gamm::v1beta1::MsgSwapExactAmountIn {
        sender: info.sender.to_string(),
        routes: swap_routes
            .iter()
            .map(|swap_route| {
                osmosis_std::types::osmosis::poolmanager::v1beta1::SwapAmountInRoute {
                    pool_id: swap_route.pool_id,
                    token_out_denom: swap_route.token_out_denom.clone(),
                }
            })
            .collect(),
        token_in: Some(Coin {
            denom: token_in.denom.clone(),
            amount: token_in.amount.to_string(),
        }),
        token_out_min_amount: token_out_min_amount.to_string(),
    };

    Ok(Response::new()
        .add_attribute("action", "swap_exact_amount_in")
        .add_attribute("sender", info.sender)
        .add_attribute("routes", to_json_string(&swap_routes)?)
        .add_attribute("token_in", token_in.to_string())
        .add_attribute("token_out_min_amount", token_out_min_amount.to_string())
        .add_message(message))
}

pub fn execute_swap_exact_amount_out(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    swap_routes: Vec<SwapRoute>,
    token_out: cosmwasm_std::Coin,
    token_in_max_amount: u128,
) -> ContractResult<Response> {
    let config = CONFIG.load(deps.storage)?;

    config.assert_trader(&info.sender)?;
    swap_routes
        .iter()
        .try_for_each(|swap_route| config.assert_allowed_swap_route(swap_route))?;

    let message = osmosis_std::types::osmosis::gamm::v1beta1::MsgSwapExactAmountOut {
        sender: info.sender.to_string(),
        routes: swap_routes
            .iter()
            .map(|swap_route| {
                osmosis_std::types::osmosis::poolmanager::v1beta1::SwapAmountOutRoute {
                    pool_id: swap_route.pool_id,
                    token_in_denom: swap_route.token_in_denom.clone(),
                }
            })
            .collect(),
        token_out: Some(Coin {
            denom: token_out.denom.clone(),
            amount: token_out.amount.to_string(),
        }),
        token_in_max_amount: token_in_max_amount.to_string(),
    };

    Ok(Response::new()
        .add_attribute("action", "swap_exact_amount_in")
        .add_attribute("sender", info.sender)
        .add_attribute("routes", to_json_string(&swap_routes)?)
        .add_attribute("token_out", token_out.to_string())
        .add_attribute("token_in_max_amount", token_in_max_amount.to_string())
        .add_message(message))
}
