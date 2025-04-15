use cosmwasm_std::{StdError, Timestamp};
use cw2::VersionError;
use cw_controllers::AdminError;
use thiserror::Error;

pub type ContractResult<T> = core::result::Result<T, ContractError>;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    Version(#[from] VersionError),

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

    #[error("Swap root not allowed")]
    SwapRouteNotAllowed {},

    #[error("Invalid token in denom {denom}")]
    InvalidTokenInDenom { denom: String },

    #[error("Invalid token out denom {denom}")]
    InvalidTokenOutDenom { denom: String },

    #[error("Invalid swap route: {index}, reason: {reason}")]
    InvalidSwapRoute { index: usize, reason: String },
}
