use cosmwasm_std::{StdError, Uint128};
use cw_controllers::AdminError;
use cw_utils::PaymentError;
use milky_way::staking::BatchStatus;
use thiserror::Error;

pub type ContractResult<T> = core::result::Result<T, ContractError>;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized: {sender}")]
    Unauthorized { sender: String },
    // Add any other custom errors you like here.
    // Look at https://docs.rs/thiserror/1.0.21/thiserror/ for details.
    #[error("Admin error: {0}")]
    Admin(#[from] AdminError),

    #[error("No pending owner")]
    NoPendingOwner {},

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

    #[error("Batch is not ready to be submitted")]
    BatchNotReady { actual: u64, expected: u64 },

    #[error("No liquid unstake requests in batch")]
    BatchEmpty {},

    #[error("Batch provided doesn't have a request for the user")]
    NoRequestInBatch {},

    #[error("Request has already been redeemed")]
    AlreadyRedeemed {},

    #[error("From wrong channel")]
    FromOtherChannel { channel: String },

    #[error("Foreign token found")]
    NoForeignTokens {},

    #[error("From wrong port")]
    FromOtherPort { port: String },

    #[error("unexpected batch status")]
    UnexpecedBatchStatus {
        actual: BatchStatus,
        expected: BatchStatus,
    },

    #[error("contract was intentionally halted")]
    Halted {},

    #[error("Config provided is wrong")]
    ConfigWrong {},
}
