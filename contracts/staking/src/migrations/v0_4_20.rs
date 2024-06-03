use crate::{
    contract::{CONTRACT_NAME, CONTRACT_VERSION},
    error::ContractResult,
    state::{Config, CONFIG},
};
use cosmwasm_std::{DepsMut, Env, Response};
use cw2::assert_contract_version;

const FROM_VERSION: &str = "0.4.18";

pub mod v0_4_18_state {
    use cosmwasm_schema::cw_serde;
    use cosmwasm_std::{Addr, Uint128};
    use cw_storage_plus::Item;

    use crate::state::{MultisigAddressConfig, ProtocolFeeConfig};

    pub const CONFIG: Item<Config> = Item::new("config");

    #[cw_serde]
    pub struct Config {
        pub native_token_denom: String,
        pub liquid_stake_token_denom: String,
        pub treasury_address: Addr,
        pub operators: Option<Vec<Addr>>, //depr
        pub monitors: Option<Vec<Addr>>,
        pub validators: Vec<Addr>,
        pub batch_period: u64,
        pub unbonding_period: u64,
        pub protocol_fee_config: ProtocolFeeConfig,
        pub multisig_address_config: MultisigAddressConfig,
        pub minimum_liquid_stake_amount: Uint128,
        pub ibc_channel_id: String,
        pub stopped: bool,
        pub oracle_contract_address: Option<Addr>,
        pub oracle_contract_address_v2: Option<Addr>,
        pub oracle_address: Option<Addr>,
    }
}

pub fn migrate(deps: DepsMut, _env: Env, send_fees_to_treasury: bool) -> ContractResult<Response> {
    // Ensure that we are migrating from the correct version.
    assert_contract_version(deps.storage, CONTRACT_NAME, FROM_VERSION)?;

    let old_config = v0_4_18_state::CONFIG.load(deps.storage)?;
    // Convert the old config format to the new one.
    let new_config = Config {
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
    CONFIG.save(deps.storage, &new_config)?;

    Ok(Response::new()
        .add_attribute("action", "migrate")
        .add_attribute("from_version", FROM_VERSION)
        .add_attribute("to_version", CONTRACT_VERSION))
}
