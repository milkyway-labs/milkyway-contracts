use crate::error::ContractResult;
use cosmwasm_std::{Coin, DepsMut, Env, MessageInfo, Response};

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

pub fn execute_transfer_ownership(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    new_owner: String,
) -> ContractResult<Response> {
    unimplemented!()
}

pub fn execute_accept_ownership(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
) -> ContractResult<Response> {
    unimplemented!()
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
