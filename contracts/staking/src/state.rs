use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Timestamp, Uint128};
use cw_controllers::Admin;
use cw_storage_plus::{Item, Map};
use milky_way::staking::Batch;

#[cw_serde]
pub struct Config {
    pub native_token_denom: String,
    pub liquid_stake_token_denom: String,
    pub treasury_address: Addr,
    pub operators: Vec<Addr>,
    pub validators: Vec<Addr>,
    pub batch_period: u64,
    pub unbonding_period: u64,
    pub protocol_fee_config: ProtocolFeeConfig,
    pub multisig_address_config: MultisigAddressConfig,
    pub minimum_liquid_stake_amount: Uint128,
    pub ibc_channel_id: String,
    pub stopped: bool,
}
// TODO: PENDING - DOCS DEFINE THESE AS MAPS?
// Discuss: Do we want to add or remove any state?
#[cw_serde]
pub struct State {
    pub total_native_token: Uint128,
    pub total_liquid_stake_token: Uint128,
    pub pending_owner: Option<Addr>,
    pub total_reward_amount: Uint128,
    pub rate: Uint128,
    pub total_fees: Uint128,
}

#[cw_serde]
pub struct ProtocolFeeConfig {
    pub dao_treasury_fee: Uint128, // not using a fraction, fee percentage=x/100000
}

#[cw_serde]
pub struct MultisigAddressConfig {
    pub staker_address: Addr,
    pub reward_collector_address: Addr,
}

// TODO: consider having this as just channel id
// since we are not interacting with the channel as far as I know
#[cw_serde]
pub struct IbcConfig {
    pub channel_id: String,
    pub default_timeout: Timestamp,
}

pub const CONFIG: Item<Config> = Item::new("config");
pub const ADMIN: Admin = Admin::new("admin");
pub const STATE: Item<State> = Item::new("state");
pub const BATCHES: Map<u64, Batch> = Map::new("batches");
pub const PENDING_BATCH_ID: Item<u64> = Item::new("pending_batch_id");
pub const IBC_CONFIG: Item<IbcConfig> = Item::new("ibc_config");
