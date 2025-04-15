use crate::{
    contract::{CONTRACT_NAME, CONTRACT_VERSION},
    error::ContractResult,
    helpers::{validate_address, validate_address_prefix, validate_denom},
    migrations::states::v0_4_20,
    state::{Config, NativeChainConfig, ProtocolChainConfig, ProtocolFeeConfig, CONFIG},
};
use cosmwasm_std::{DepsMut, Env, Response};
use cw2::{assert_contract_version, set_contract_version};

const FROM_VERSION: &str = "0.4.20";
const TO_VERSION: &str = "1.0.0";

pub fn migrate(
    deps: DepsMut,
    _env: Env,
    native_account_address_prefix: String,
    native_validator_address_prefix: String,
    native_token_denom: String,
    protocol_account_address_prefix: String,
) -> ContractResult<Response> {
    // Ensure that we are migrating from the correct version.
    assert_contract_version(deps.storage, CONTRACT_NAME, FROM_VERSION)?;

    // Ensure the address prefixes are valid
    let native_account_address_prefix = validate_address_prefix(&native_account_address_prefix)?;
    let native_validator_address_prefix =
        validate_address_prefix(&native_validator_address_prefix)?;
    let protocol_account_address_prefix =
        validate_address_prefix(&protocol_account_address_prefix)?;

    // Ensure that the token denom is valid
    validate_denom(&native_token_denom)?;

    let old_config = v0_4_20::CONFIG.load(deps.storage)?;

    // Ensure the currently configured native chain addresses have the provided prefixes
    validate_address(
        old_config.multisig_address_config.staker_address.as_str(),
        &native_account_address_prefix,
    )?;
    validate_address(
        old_config
            .multisig_address_config
            .reward_collector_address
            .as_str(),
        &native_account_address_prefix,
    )?;
    for address in old_config.validators.iter() {
        validate_address(address.as_str(), &native_validator_address_prefix)?;
    }

    // Ensure the currently configured protocol chain addresses have the provided prefixes
    if let Some(address) = &old_config.oracle_address {
        validate_address(address.as_str(), &protocol_account_address_prefix)?;
    }
    if old_config.send_fees_to_treasury {
        validate_address(
            old_config.treasury_address.as_str(),
            &protocol_account_address_prefix,
        )?;
    }

    // Convert the old config format to the new one.
    let new_config = Config {
        native_chain_config: NativeChainConfig {
            account_address_prefix: native_account_address_prefix,
            validator_address_prefix: native_validator_address_prefix,
            validators: old_config.validators,
            token_denom: native_token_denom,
            reward_collector_address: old_config.multisig_address_config.reward_collector_address,
            staker_address: old_config.multisig_address_config.staker_address,
            unbonding_period: old_config.unbonding_period,
        },
        protocol_chain_config: ProtocolChainConfig {
            account_address_prefix: protocol_account_address_prefix,
            ibc_channel_id: old_config.ibc_channel_id,
            ibc_token_denom: old_config.native_token_denom,
            minimum_liquid_stake_amount: old_config.minimum_liquid_stake_amount,
            oracle_address: old_config.oracle_address,
        },
        protocol_fee_config: ProtocolFeeConfig {
            dao_treasury_fee: old_config.protocol_fee_config.dao_treasury_fee,
            treasury_address: if old_config.send_fees_to_treasury {
                Some(old_config.treasury_address)
            } else {
                None
            },
        },
        liquid_stake_token_denom: old_config.liquid_stake_token_denom,
        batch_period: old_config.batch_period,
        monitors: old_config.monitors.unwrap_or_default(),
        stopped: old_config.stopped,
    };
    // Save the new config.
    CONFIG.save(deps.storage, &new_config)?;

    set_contract_version(deps.storage, CONTRACT_NAME, TO_VERSION)?;

    Ok(Response::new()
        .add_attribute("action", "migrate")
        .add_attribute("from_version", FROM_VERSION)
        .add_attribute("to_version", CONTRACT_VERSION))
}
