use cosmwasm_std::{StdError, Timestamp, Uint128};
use cw2::VersionError;
use cw_controllers::AdminError;
use cw_utils::PaymentError;
use milky_way::staking::BatchStatus;
use thiserror::Error;

pub type ContractResult<T> = core::result::Result<T, ContractError>;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    SerdeJson(#[from] serde_json::Error),

    #[error("Unauthorized: {sender}")]
    Unauthorized { sender: String },
    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.
    #[error("Admin error: {0}")]
    Admin(#[from] AdminError),

    #[error("No pending owner")]
    NoPendingOwner {},

    #[error("Ownership transfer not ready")]
    OwnershipTransferNotReady { time_to_claim: Timestamp },

    #[error("Payment error: {0}")]
    Payment(#[from] PaymentError),

    #[error("Minimum liquid stake amount not met")]
    MinimumLiquidStakeAmount {
        minimum_stake_amount: Uint128,
        sent_amount: Uint128,
    },

    #[error("Unable to mint liquid staking token")]
    MintError {},

    #[error("Validator already exists")]
    DuplicateValidator { validator: String },

    #[error("Validator not found")]
    ValidatorNotFound { validator: String },

    #[error("Address is not valid")]
    InvalidAddress {},

    #[error("MilkyWay only supports unordered channels")]
    OrderedChannel {},

    #[error("Invalid IBC version")]
    InvalidVersion { actual: String, expected: String },

    #[error("No IBC channel found")]
    IbcChannelNotFound {},

    #[error("The prvoided IBC channel and reserve token config is wrong")]
    IbcChannelConfigWrong {},

    #[error("Batch is not ready to be submitted")]
    BatchNotReady { actual: u64, expected: u64 },

    #[error("No liquid unstake requests in batch")]
    BatchEmpty {},

    #[error("Batch is either already closed or is in an error state")]
    BatchNotClaimable { batch_id: u64, status: BatchStatus },

    #[error("Batch {batch_id} don't have the expected native amount")]
    BatchWithoutExpectedNativeAmount { batch_id: u64 },

    #[error(
        "Received wrong batch amount, batch_id {batch_id} expected {expected}, got {received}"
    )]
    ReceivedWrongBatchAmount {
        batch_id: u64,
        expected: Uint128,
        received: Uint128,
    },

    #[error("The tokens in this batch have already been claimed")]
    TokensAlreadyClaimed { batch_id: u64 },

    #[error("Batch provided doesn't have a request for the user")]
    NoRequestInBatch {},

    #[error("From wrong channel")]
    FromOtherChannel { channel: String },

    #[error("Foreign token found")]
    NoForeignTokens {},

    #[error("From wrong port")]
    FromOtherPort { port: String },

    #[error("Invalid reply id")]
    InvalidReplyID { id: u64 },

    #[error("No inflight packages to recover")]
    NoInflightPackets {},

    #[error("Can't recover packet {id} because is not failed or timed out")]
    InvalidPacketStatus { id: u64 },

    #[error("unexpected batch status")]
    UnexpecedBatchStatus { actual: BatchStatus },

    #[error("Minimum liquid stake amount not met")]
    InvalidUnstakeAmount {
        total_liquid_stake_token: Uint128,
        amount_to_unstake: Uint128,
    },

    #[error("contract was intentionally stopped")]
    Stopped {},

    #[error("contract is not stopped")]
    NotStopped {},

    #[error("Config provided is wrong")]
    ConfigWrong {},

    #[error("format error")]
    FormatError {},

    #[error("Failed ibc transfer")]
    FailedIBCTransfer { msg: String },

    #[error("Contract already locked")]
    ContractLocked { msg: String },

    #[error("Receive rewards are smaller then the fee")]
    ReceiveRewardsTooSmall { amount: Uint128, minimum: Uint128 },

    #[error("The computed fees are zero for the received rewards: {received_rewards}")]
    ComputedFeesAreZero { received_rewards: Uint128 },

    #[error("No liquid stake to distribute rewards to")]
    NoLiquidStake {},

    #[error("Calculated mint amount not as expected")]
    MintAmountMismatch { expected: Uint128, actual: Uint128 },

    #[error("Insufficient funds")]
    InsufficientFunds {},

    #[error("If liquid staking is done from a non native Osmosis address you need to provide an address via 'mint_to'")]
    MissingMintAddress {},

    #[error("The treasury address has not been configured")]
    TreasuryNotConfigured {},

    #[error("{0}")]
    Version(#[from] VersionError),

    #[error("Can't recover packets with different receivers")]
    InvalidReceiver {},

    #[error("Can't recover packets with different denoms")]
    InconsistentDenom {},

    #[error("The contract is migrating to a newer version")]
    Migrating {},

    #[error("DAO treasury fee can't be higher then 100000")]
    InvalidDaoTreasuryFee {},
    #[error("The field '{field_name}' has a value of {value}, which exceeds the maximum allowed value of {max}.")]
    ValueTooBig {
        field_name: String,
        value: Uint128,
        max: Uint128,
    },

    #[error("Oracle contract instantiation failed")]
    InstantiateOracleFailed {},
}
