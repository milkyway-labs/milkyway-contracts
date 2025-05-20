use crate::{contract::CONTRACT_NAME, error::ContractResult};
use cosmwasm_std::{DepsMut, Env, Response};
use cw2::{assert_contract_version, set_contract_version};

const FROM_VERSION: &str = "1.2.0";
const TO_VERSION: &str = "1.3.0";

pub fn migrate(deps: DepsMut, _env: Env) -> ContractResult<Response> {
    assert_contract_version(deps.storage, CONTRACT_NAME, FROM_VERSION)?;
    set_contract_version(deps.storage, CONTRACT_NAME, TO_VERSION)?;

    Ok(Response::new()
        .add_attribute("action", "migrate")
        .add_attribute("from_version", FROM_VERSION)
        .add_attribute("to_version", TO_VERSION)
        .add_attribute("completed", "true"))
}
