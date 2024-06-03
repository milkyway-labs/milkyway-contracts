use crate::state::{
    ibc::IBCTransfer, IbcWaitingForReply, MultisigAddressConfig, ProtocolFeeConfig, UnstakeRequest,
};
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Decimal, Timestamp, Uint128};
use milky_way::staking::BatchStatus;
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
    /// Set of addresses allowed to trigger a circuit break
    pub monitors: Vec<String>,
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
    // The redemption / purchase rate oracle address
    pub oracle_address: Option<String>,
    // Whether to automatically send the collected fees to the treasury
    pub send_fees_to_treasury: bool,
}

#[cw_serde]
#[allow(clippy::large_enum_variant)]
pub enum ExecuteMsg {
    LiquidStake {
        mint_to: Option<String>,
        expected_mint_amount: Option<Uint128>,
    },
    LiquidUnstake {},
    SubmitBatch {},
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
        native_token_denom: Option<String>,
        channel_id: Option<String>,
        monitors: Option<Vec<String>>,
        treasury_address: Option<String>,
        oracle_address: Option<String>,
        send_fees_to_treasury: Option<bool>,
    },
    ReceiveRewards {},
    ReceiveUnstakedTokens {
        batch_id: u64,
    },
    CircuitBreaker {},
    ResumeContract {
        total_native_token: Uint128,
        total_liquid_stake_token: Uint128,
        total_reward_amount: Uint128,
    },
    RecoverPendingIbcTransfers {
        paginated: Option<bool>,
        selected_packets: Option<Vec<u64>>,
    },
    FeeWithdraw {
        amount: Uint128,
    },
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug, Default)]
pub struct ConfigResponse {
    pub native_token_denom: String,
    pub liquid_stake_token_denom: String,
    pub treasury_address: String,
    pub monitors: Vec<String>,
    pub validators: Vec<String>,
    pub batch_period: u64,
    pub unbonding_period: u64,
    pub minimum_liquid_stake_amount: Uint128,
    pub staker_address: String,
    pub reward_collector_address: String,
    pub protocol_fee_config: ProtocolFeeConfig,
    pub ibc_channel_id: String,
    pub stopped: bool,
    pub oracle_address: String,
    pub send_fees_to_treasury: bool,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug, Default)]
pub struct StateResponse {
    pub total_native_token: Uint128,
    pub total_liquid_stake_token: Uint128,
    pub rate: Decimal,
    pub pending_owner: String,
    pub total_reward_amount: Uint128,
    pub total_fees: Uint128,
}
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug, Default)]
pub struct BatchResponse {
    pub id: u64,
    pub batch_total_liquid_stake: Uint128,
    pub expected_native_unstaked: Uint128,
    pub received_native_unstaked: Uint128,
    pub unstake_request_count: u64,
    pub next_batch_action_time: Timestamp,
    pub status: String,
}
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug, Default)]
pub struct BatchesResponse {
    pub batches: Vec<BatchResponse>,
}
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug, Default)]
pub struct UnstakeRequestResponse {
    pub batch_id: u64,
    pub batch_total_liquid_stake: Uint128,
    pub expected_native_unstaked: Uint128,
    pub received_native_unstaked: Uint128,
    pub status: String,
    pub unstake_amount: Uint128,
    pub user: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug, Default)]
pub struct IBCQueueResponse {
    pub ibc_queue: Vec<IBCTransfer>,
}
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug, Default)]
pub struct IBCReplyQueueResponse {
    pub ibc_queue: Vec<IbcWaitingForReply>,
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
    Batches {
        start_after: Option<u64>,
        limit: Option<u32>,
        status: Option<BatchStatus>,
    },
    #[returns(BatchesResponse)]
    BatchesByIds { ids: Vec<u64> },
    #[returns(BatchResponse)]
    PendingBatch {},
    #[returns(Vec<UnstakeRequest>)]
    UnstakeRequests { user: Addr },
    #[returns(Vec<UnstakeRequestResponse>)]
    AllUnstakeRequests {
        start_after: Option<u64>,
        limit: Option<u32>,
    },
    #[returns(Vec<(String, u64, Uint128)>)]
    AllUnstakeRequestsV2 {
        start_after: Option<u64>,
        limit: Option<u32>,
    },
    #[returns(IBCQueueResponse)]
    IbcQueue {
        start_after: Option<u64>,
        limit: Option<u32>,
    },
    #[returns(IBCReplyQueueResponse)]
    IbcReplyQueue {
        start_after: Option<u64>,
        limit: Option<u32>,
    },
}

#[cw_serde]
pub enum MigrateMsg {
    V0_4_18ToV0_4_20 { send_fees_to_treasury: bool },
}

#[cw_serde]
pub enum IBCLifecycleComplete {
    #[serde(rename = "ibc_ack")]
    IBCAck {
        /// The source channel (osmosis side) of the IBC packet
        channel: String,
        /// The sequence number that the packet was sent with
        sequence: u64,
        /// String encoded version of the ack as seen by OnAcknowledgementPacket(..)
        ack: String,
        /// Weather an ack is a success of failure according to the transfer spec
        success: bool,
    },
    #[serde(rename = "ibc_timeout")]
    IBCTimeout {
        /// The source channel (osmosis side) of the IBC packet
        channel: String,
        /// The sequence number that the packet was sent with
        sequence: u64,
    },
}

/// Message type for `sudo` entry_point
#[cw_serde]
pub enum SudoMsg {
    #[serde(rename = "ibc_lifecycle_complete")]
    IBCLifecycleComplete(IBCLifecycleComplete),
}
