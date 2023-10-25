use cosmwasm_std::{StdError, Uint128};
use cw_controllers::AdminError;
use cw_utils::PaymentError;
use thiserror::Error;

pub type ContractResult<T> = core::result::Result<T, ContractError>;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},
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
}
