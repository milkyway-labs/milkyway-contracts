use cosmwasm_std::{StdError, Timestamp};
use cw_controllers::AdminError;
use thiserror::Error;

pub type ContractResult<T> = core::result::Result<T, ContractError>;

#[derive(Error, Debug, PartialEq)]
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

    #[error("Ownership transfer not ready")]
    OwnershipTransferNotReady { time_to_claim: Timestamp },

    #[error("swap root not allowed")]
    SwapRouteNotAllowed {},

    #[error("invalid token in denom {denom}")]
    InvalidTokenInDenom { denom: String },

    #[error("invalid token out denom {denom}")]
    InvalidTokenOutDenom { denom: String },
}
