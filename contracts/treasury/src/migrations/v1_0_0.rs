use crate::{
    contract::CONTRACT_NAME,
    error::ContractResult,
    state::{Config, CONFIG},
    types::{UnsafeNativeChainConfig, UnsafeProtocolChainConfig},
};
use cosmwasm_std::{DepsMut, Env, Response};
use cw2::{assert_contract_version, set_contract_version};
use milky_way::utils::validate_address;

use super::states::v0_4_20;

pub const FROM_VERSION: &str = "0.4.20";
pub const TO_VERSION: &str = "1.0.0";

pub fn migrate(
    deps: DepsMut,
    _env: Env,
    native_chain_config: UnsafeNativeChainConfig,
    protocol_chain_config: UnsafeProtocolChainConfig,
) -> ContractResult<Response> {
    // Ensure that we are migrating from the correct version.
    assert_contract_version(deps.storage, CONTRACT_NAME, FROM_VERSION)?;

    // Ensure the address prefixes are valid

    let old_config = v0_4_20::CONFIG.load(deps.storage)?;

    // Validate the new config
    let native_chain_config = native_chain_config.validate()?;
    let protocol_chain_config = protocol_chain_config.validate()?;

    // Ensure the currently configured native chain addresses have the provided prefixes
    validate_address(
        old_config.trader.as_str(),
        &protocol_chain_config.account_address_prefix,
    )?;

    // Convert the old config format to the new one.
    let new_config = Config {
        native_chain_config,
        protocol_chain_config,
        trader: old_config.trader,
        allowed_swap_routes: old_config.allowed_swap_routes,
    };
    // Save the new config.
    CONFIG.save(deps.storage, &new_config)?;

    // Update the contract version
    set_contract_version(deps.storage, CONTRACT_NAME, TO_VERSION)?;

    Ok(Response::new()
        .add_attribute("action", "migrate")
        .add_attribute("from_version", FROM_VERSION)
        .add_attribute("to_version", TO_VERSION))
}
