use crate::state::{MultisigAddressConfig, ProtocolFeeConfig};
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Decimal, Timestamp, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[cw_serde]
pub struct InstantiateMsg {
    /// Denomination of underlying token (IBC denom of TIA)
    pub native_token_denom: String,
    /// Denomination of the liquid staking token (stTIA)
    pub liquid_stake_token_denom: String,
    /// Treasury contract address
    pub treasury_address: String,
    /// Set of node operators who will operate the protocol
    pub operators: Vec<String>,
    /// Set of validators who will receive the delegations
    pub validators: Vec<String>,
    /// How often the unbonding queue is to be executed in seconds
    pub batch_period: u64,
    /// The staking module's unbonding period for Celestia in seconds
    pub unbonding_period: u64,
    /// Protocol fee configuration
    pub protocol_fee_config: ProtocolFeeConfig,
    /// Multisig address configuration
    pub multisig_address_config: MultisigAddressConfig,
    /// Minimum amount to liquid stake
    pub minimum_liquid_stake_amount: Uint128,
    // IBC channel id on Osmosis to Celestia
    pub ibc_channel_id: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    LiquidStake {},
    LiquidUnstake {},
    SubmitBatch {
        batch_id: u64,
    },
    Withdraw {
        batch_id: u64,
    },
    AddValidator {
        new_validator: String,
    },
    RemoveValidator {
        validator: String,
    },
    TransferOwnership {
        new_owner: String,
    },
    AcceptOwnership {},
    RevokeOwnershipTransfer {},
    UpdateConfig {
        batch_period: Option<u64>,
        unbonding_period: Option<u64>,
        minimum_liquid_stake_amount: Option<Uint128>,
        multisig_address_config: Option<MultisigAddressConfig>,
        protocol_fee_config: Option<ProtocolFeeConfig>,
    },
    ReceiveRewards {},
    ReceiveUnstakedTokens {},
    CircuitBreaker {},
    ResumeContract {},
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug, Default)]
pub struct ConfigResponse {
    pub native_token_denom: String,
    pub liquid_stake_token_denom: String,
    pub treasury_address: String,
    pub operators: Vec<String>,
    pub validators: Vec<String>,
    pub batch_period: u64,
    pub unbonding_period: u64,
    pub minimum_liquid_stake_amount: Uint128,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug, Default)]
pub struct StateResponse {
    pub total_native_token: Uint128,
    pub total_liquid_stake_token: Uint128,
    pub rate: Decimal,
    pub pending_owner: String,
    pub total_reward_amount: Uint128,
}
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug, Default)]
pub struct LiquidUnstakeRequestResponse {
    pub user: String,
    pub amount: Uint128,
}
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug, Default)]
pub struct BatchResponse {
    pub batch_total_liquid_stake: Uint128,
    pub expected_native_unstaked: Uint128,
    pub next_batch_action_time: Timestamp,
    pub status: String,
    pub requests: Vec<LiquidUnstakeRequestResponse>,
}
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug, Default)]
pub struct BatchesResponse {
    pub batches: Vec<BatchResponse>,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ConfigResponse)]
    Config {},
    #[returns(StateResponse)]
    State {},
    #[returns(BatchResponse)]
    Batch { id: u64 },
    #[returns(BatchesResponse)]
    Batches {},
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct MigrateMsg {}
