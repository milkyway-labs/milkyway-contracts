use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::Item;

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

#[cw_serde]
#[derive(Default)]
pub struct ProtocolFeeConfig {
    pub dao_treasury_fee: Uint128, // not using a fraction, fee percentage=x/100000
}

#[cw_serde]
pub struct MultisigAddressConfig {
    pub staker_address: Addr,
    pub reward_collector_address: Addr,
}

pub const CONFIG: Item<Config> = Item::new("config");
