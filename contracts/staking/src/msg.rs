use crate::state::{MultisigAddressConfig, ProtocolFeeConfig};
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Uint128;
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
    pub node_operators: Vec<String>,
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
    /// Minimum staking rewards to collect on Celestia
    pub minimum_rewards_to_collect: Uint128,
}

#[cw_serde]
pub enum ExecuteMsg {
    LiquidStake {},
    LiquidUnstake {},
    SubmitBatch { batch_id: u64 },
    Claim {},
    AddValidator { new_validator: String },
    RemoveValidator { validator: String },
    TransferOwnership { new_owner: String },
    AcceptOwnership {},
    RevokeOwnershipTransfer {},
}

#[cw_serde]
pub enum IbcExecuteMsg {
    ReceiveBatch { batch_id: u64, batch_amount: Uint128 },
    ReceiveRewards {reward_amount: Uint128},
}


#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct MigrateMsg {}
