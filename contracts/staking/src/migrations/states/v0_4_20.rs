use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::Item;

use super::v0_4_18;

#[cw_serde]
pub struct Config {
    pub native_token_denom: String,
    pub liquid_stake_token_denom: String,
    pub treasury_address: Addr,
    pub monitors: Option<Vec<Addr>>,
    pub validators: Vec<Addr>,
    pub batch_period: u64,
    pub unbonding_period: u64,
    pub protocol_fee_config: v0_4_18::ProtocolFeeConfig,
    pub multisig_address_config: v0_4_18::MultisigAddressConfig,
    pub minimum_liquid_stake_amount: Uint128,
    pub ibc_channel_id: String,
    pub stopped: bool,
    pub oracle_address: Option<Addr>,
    // Tells if the contract will automatically send the collected fees
    // to the treasury.
    pub send_fees_to_treasury: bool,
}

pub const CONFIG: Item<Config> = Item::new("config");
