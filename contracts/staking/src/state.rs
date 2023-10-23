use cosmwasm_schema::cw_serde;
use cosmwasm_std::Uint128;
use cw20::Denom;

#[cw_serde]
pub struct Config {
    pub native_token_denom: Denom,
    pub liquid_stake_token_denom: Denom,
    pub treasury_address: String,
    pub node_operators: Vec<String>,
    pub validators: Vec<String>,
    pub epoch_period: u64,
    pub unbonding_period: u64,
    pub protocol_fee_config: ProtocolFeeConfig,
    pub multisig_address_config: MultisigAddressConfig,
    pub minimum_liquid_stake_amount: Uint128,
    pub minimum_rewards_to_collect: Uint128,
}

#[cw_serde]
pub struct ProtocolFeeConfig {
    pub dao_treasury_fee: Uint128,
}

#[cw_serde]
pub struct MultisigAddressConfig {
    pub controller_address: String,
    pub staker_address: String,
    pub reward_collector_address: String,
}
