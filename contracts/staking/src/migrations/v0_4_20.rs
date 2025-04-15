use crate::{
    contract::{CONTRACT_NAME, CONTRACT_VERSION},
    error::ContractResult,
    migrations::states::v0_4_18,
    migrations::states::v0_4_20,
};
use cosmwasm_std::{DepsMut, Env, Response};
use cw2::{assert_contract_version, set_contract_version};

const FROM_VERSION: &str = "0.4.18";
const TO_VERSION: &str = "0.4.20";

pub fn migrate(deps: DepsMut, _env: Env, send_fees_to_treasury: bool) -> ContractResult<Response> {
    // Ensure that we are migrating from the correct version.
    assert_contract_version(deps.storage, CONTRACT_NAME, FROM_VERSION)?;

    let old_config = v0_4_18::CONFIG.load(deps.storage)?;
    // Convert the old config format to the new one.
    let new_config = v0_4_20::Config {
        native_token_denom: old_config.native_token_denom,
        liquid_stake_token_denom: old_config.liquid_stake_token_denom,
        treasury_address: old_config.treasury_address,
        monitors: old_config.monitors,
        validators: old_config.validators,
        batch_period: old_config.batch_period,
        unbonding_period: old_config.unbonding_period,
        protocol_fee_config: old_config.protocol_fee_config,
        multisig_address_config: old_config.multisig_address_config,
        minimum_liquid_stake_amount: old_config.minimum_liquid_stake_amount,
        ibc_channel_id: old_config.ibc_channel_id,
        stopped: old_config.stopped,
        oracle_address: old_config.oracle_address,
        send_fees_to_treasury,
    };
    // Save the new config.
    v0_4_20::CONFIG.save(deps.storage, &new_config)?;

    set_contract_version(deps.storage, CONTRACT_NAME, TO_VERSION)?;

    Ok(Response::new()
        .add_attribute("action", "migrate")
        .add_attribute("from_version", FROM_VERSION)
        .add_attribute("to_version", CONTRACT_VERSION))
}
