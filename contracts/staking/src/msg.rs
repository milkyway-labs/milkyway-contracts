use crate::{
    state::{
        ibc::IBCTransfer, IbcWaitingForReply, NativeChainConfig, ProtocolChainConfig,
        ProtocolFeeConfig, UnstakeRequest,
    },
    types::{
        BatchExpectedAmount, UnsafeNativeChainConfig, UnsafeProtocolChainConfig,
        UnsafeProtocolFeeConfig,
    },
};
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Decimal, Timestamp, Uint128};
use milky_way::staking::BatchStatus;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[cw_serde]
pub struct InstantiateMsg {
    pub native_chain_config: UnsafeNativeChainConfig,
    pub protocol_chain_config: UnsafeProtocolChainConfig,
    pub protocol_fee_config: UnsafeProtocolFeeConfig,

    /// Denomination of the liquid staking token (stTIA)
    pub liquid_stake_token_denom: String,

    pub batch_period: u64,
    pub monitors: Vec<String>,
}

#[cw_serde]
#[allow(clippy::large_enum_variant)]
pub enum ExecuteMsg {
    LiquidStake {
        mint_to: Option<String>,
        expected_mint_amount: Option<Uint128>,
        /// If the native chain and protocol chain share the same address prefix,
        /// the contract uses this value to determine whether the LST token
        /// should be sent to an account on the same chain or transferred
        /// over IBC to the native chain.
        /// If not provided, this defaults to `false`.
        transfer_to_native_chain: Option<bool>,
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
        native_chain_config: Option<UnsafeNativeChainConfig>,
        protocol_chain_config: Option<UnsafeProtocolChainConfig>,
        protocol_fee_config: Option<UnsafeProtocolFeeConfig>,
        monitors: Option<Vec<String>>,
        batch_period: Option<u64>,
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
    SlashBatches {
        new_amounts: Vec<BatchExpectedAmount>,
    },
    RecoverPendingIbcTransfers {
        paginated: Option<bool>,
        selected_packets: Option<Vec<u64>>,
        receiver: Option<String>,
    },
    FeeWithdraw {
        amount: Uint128,
    },
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct ConfigResponse {
    pub native_chain_config: NativeChainConfig,
    pub protocol_chain_config: ProtocolChainConfig,
    pub protocol_fee_config: ProtocolFeeConfig,
    pub monitors: Vec<Addr>,
    pub liquid_stake_token_denom: String,
    pub batch_period: u64,
    pub stopped: bool,
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
    V0_4_18ToV0_4_20 {
        send_fees_to_treasury: bool,
    },
    V0_4_20ToV1_0_0 {
        native_account_address_prefix: String,
        native_validator_address_prefix: String,
        native_token_denom: String,
        protocol_account_address_prefix: String,
    },
    V1_0_0ToV1_1_0 {
        limit: Option<usize>,
    },
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
