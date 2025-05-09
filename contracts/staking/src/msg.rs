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
use cw_controllers::AdminResponse;
use milky_way::staking::BatchStatus;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[cw_serde]
pub struct InstantiateMsg {
    /// Information about the chain from which the token
    /// (for which we are creating the LST token) originates.
    pub native_chain_config: UnsafeNativeChainConfig,

    /// Information about the chain where the contract is deployed.
    pub protocol_chain_config: UnsafeProtocolChainConfig,

    /// Protocol fee configuration.
    pub protocol_fee_config: UnsafeProtocolFeeConfig,

    /// Denomination of the liquid staking token (e.g., stTIA).
    pub liquid_stake_token_denom: String,

    /// Frequency (in seconds) at which the unbonding queue is executed.
    pub batch_period: u64,

    /// Set of addresses allowed to trigger a circuit break.
    pub monitors: Vec<String>,
    pub admin: Option<String>,
}

#[cw_serde]
#[allow(clippy::large_enum_variant)]
pub enum ExecuteMsg {
    /// Initiates the bonding process for a user.
    LiquidStake {
        /// Address to receive the minted LST tokens.
        /// Can belong to either the chain where this contract is deployed
        /// or the chain from which the native token originates.
        /// If `None`, the tokens are sent to the message sender.
        mint_to: Option<String>,

        /// When both native and protocol chains share the same address prefix,
        /// this flag determines whether to send tokens to the native or protocol chain.
        transfer_to_native_chain: Option<bool>,

        /// Minimum expected amount of LST tokens to be received
        /// for the operation to be considered valid.
        expected_mint_amount: Option<Uint128>,
    },

    /// Initiates the unbonding process for a user.
    LiquidUnstake {},

    /// Withdraws unstaked tokens.
    Withdraw {
        /// ID of the batch from which to withdraw.
        batch_id: u64,
    },

    /// Processes the pending batch.
    SubmitBatch {},

    /// Adds a validator to the validator set; callable by the owner.
    AddValidator {
        /// Address of the validator to add.
        new_validator: String,
    },

    /// Removes a validator from the validator set; callable by the owner.
    RemoveValidator {
        /// Address of the validator to remove.
        validator: String,
    },

    /// Transfers ownership to another account; callable by the owner.
    /// The new owner must accept the transfer for it to take effect.
    TransferOwnership {
        /// Address of the new owner on the protocol chain.
        new_owner: String,
    },

    /// Accepts ownership transfer; callable by the new owner.
    AcceptOwnership {},

    /// Revokes ownership transfer; callable by the current owner.
    RevokeOwnershipTransfer {},

    /// Updates contract configuration; callable by the owner.
    UpdateConfig {
        /// Updated native chain configuration.
        native_chain_config: Option<UnsafeNativeChainConfig>,

        /// Updated protocol chain configuration.
        protocol_chain_config: Option<UnsafeProtocolChainConfig>,

        /// Updated protocol fee configuration.
        protocol_fee_config: Option<UnsafeProtocolFeeConfig>,

        /// Updated list of circuit breaker monitors.
        monitors: Option<Vec<String>>,

        /// Updated unbonding batch execution frequency (in seconds).
        batch_period: Option<u64>,
    },

    /// Receives rewards from the native chain.
    ReceiveRewards {},

    /// Receives unstaked tokens from the native chain.
    ReceiveUnstakedTokens {
        /// ID of the batch that originated the unstake request.
        batch_id: u64,
    },

    /// Stops the contract due to irregularities; callable by monitors and admin.
    CircuitBreaker {},

    /// Resumes the contract; callable by the admin.
    ResumeContract {
        /// Updated total native tokens delegated (used post-slashing).
        total_native_token: Uint128,

        /// Updated total issued liquid staked tokens.
        total_liquid_stake_token: Uint128,

        /// Updated total protocol rewards.
        total_reward_amount: Uint128,
    },
    SlashBatches {
        new_amounts: Vec<BatchExpectedAmount>,
    },

    /// Recovers IBC transfers that timed out or failed.
    RecoverPendingIbcTransfers {
        /// If true and neither `selected_packets` nor `receiver` are specified,
        /// recovers only the 10 oldest failed IBC transfers.
        paginated: Option<bool>,

        /// Specific packet IDs to recover.
        /// Overrides other parameters if provided.
        selected_packets: Option<Vec<u64>>,

        /// Recovers packets addressed to this account.
        /// Considered only if `selected_packets` is not provided.
        receiver: Option<String>,
    },

    /// Sends the protocol fee to the treasury.
    FeeWithdraw {
        /// Amount to send to the treasury.
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
    /// Queries the contract configuration.
    /// Returns the current `native_chain_config`, `protocol_chain_config`,
    /// `protocol_fee_config`, `liquid_stake_token_denom`, and other settings.
    #[returns(ConfigResponse)]
    Config {},

    /// Queries the current state of the contract.
    /// Returns totals such as delegated native tokens, LST supply, and rewards.
    #[returns(StateResponse)]
    State {},

    /// Queries the information of a specific batch by its ID.
    #[returns(BatchResponse)]
    Batch {
        /// ID of the batch to query.
        id: u64,
    },

    /// Queries a paginated list of all batches stored in contract storage.
    #[returns(BatchesResponse)]
    Batches {
        /// If provided, starts listing batches after this batch ID.
        start_after: Option<u64>,

        /// Maximum number of batches to return.
        limit: Option<u32>,

        /// Optional filter to return only batches with the given status.
        status: Option<BatchStatus>,
    },

    /// Queries the batches with the provided list of IDs.
    #[returns(BatchesResponse)]
    BatchesByIds {
        /// List of batch IDs to fetch.
        ids: Vec<u64>,
    },

    /// Queries the current batch that is pending processing (if any).
    #[returns(BatchResponse)]
    PendingBatch {},

    /// Queries the unstake requests made by a specific user.
    #[returns(Vec<UnstakeRequest>)]
    UnstakeRequests {
        /// Address of the user whose unstake requests are to be queried.
        user: Addr,
    },

    /// Queries all unstake requests in the contract.
    #[returns(Vec<UnstakeRequestResponse>)]
    AllUnstakeRequests {
        /// If provided, starts listing unstake requests after this ID.
        start_after: Option<u64>,

        /// Maximum number of unstake requests to return.
        limit: Option<u32>,
    },

    #[returns(IBCQueueResponse)]
    IbcQueue {
        /// If provided, starts listing IBC packets after this packet ID.
        start_after: Option<u64>,

        /// Maximum number of IBC packets to return.
        limit: Option<u32>,
    },

    /// Queries IBC packets that have been sent and are still waiting for a reply.
    #[returns(IBCReplyQueueResponse)]
    IbcReplyQueue {
        /// If provided, starts listing sent-but-unreplied packets after this ID.
        start_after: Option<u64>,

        /// Maximum number of IBC reply queue entries to return.
        limit: Option<u32>,
    },

    /// Queries the current admin.
    #[returns(AdminResponse)]
    Admin {},
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
    V1_1_0ToV1_2_0 {},
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
