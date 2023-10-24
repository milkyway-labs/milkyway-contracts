use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};

#[cw_serde]
pub struct Config {
    pub native_token_denom: String,
    pub liquid_stake_token_denom: String,
    pub treasury_address: Addr,
    pub node_operators: Vec<Addr>,
    pub validators: Vec<Addr>,
    pub batch_period: u64,
    pub unbonding_period: u64,
    pub protocol_fee_config: ProtocolFeeConfig,
    pub multisig_address_config: MultisigAddressConfig,
    pub minimum_liquid_stake_amount: Uint128,
    pub minimum_rewards_to_collect: Uint128,
}
// TODO: PENDING - DOCS DEFINE THESE AS MAPS?
pub struct State<'a> {
    pub total_native_token: Item<'a, Uint128>,
    pub total_liquid_stake_token: Item<'a, Uint128>,
    pub native_token_to_stake: Item<'a, Uint128>,
}

impl<'a> State<'a> {
    pub fn new() -> Self {
        Self {
            total_native_token: Item::new("total_native_token"),
            total_liquid_stake_token: Item::new("total_liquid_stake_token"),
            native_token_to_stake: Item::new("native_token_to_stake"),
        }
    }
}

#[cw_serde]
pub struct ProtocolFeeConfig {
    pub dao_treasury_fee: Uint128,
}

#[cw_serde]
pub struct MultisigAddressConfig {
    pub controller_address: Addr,
    pub staker_address: Addr,
    pub reward_collector_address: Addr,
}

pub const CONFIG: Item<Config> = Item::new("config");
