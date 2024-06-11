use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Coin};

use crate::state::SwapRoute;

#[cw_serde]
pub struct InstantiateMsg {
    pub admin: Option<String>,
    pub trader: Option<String>,
    pub allowed_swap_routes: Vec<Vec<SwapRoute>>,
}

#[cw_serde]
pub enum ExecuteMsg {
    TransferOwnership {
        new_owner: String,
    },
    AcceptOwnership {},
    RevokeOwnershipTransfer {},
    SpendFunds {
        amount: Coin,
        receiver: String,
        channel_id: Option<String>,
    },
    /// Performs an osmosis SwapExactAmountIn operation
    /// using the treasury's balance.
    SwapExactAmountIn {
        /// The swap route to take.
        routes: Vec<SwapRoute>,
        /// The amount to swap in from the treasury balance.
        token_in: Coin,
        /// The minimum amount that we are willing to accept as
        /// result of the swap.
        token_out_min_amount: u128,
    },
    /// Performs an osmosis SwapExactAmountOut operation
    /// using the treasury's balance.
    SwapExactAmountOut {
        /// The swap route to take.
        routes: Vec<SwapRoute>,
        /// The coin that we want to receive from the swap.
        token_out: Coin,
        /// The maximum amount that we are willing to spend
        /// in order to receive the requested `token_out`.
        token_in_max_amount: u128,
    },
    /// Updates the contract configuration.
    UpdateConfig {
        /// Optional new trader address.
        /// If `None`, the trader address will not change.
        trader: Option<String>,
        /// Optional new allowed swap routes.
        /// If `None`, the allowed swap routes will not change.
        allowed_swap_routes: Option<Vec<Vec<SwapRoute>>>,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ConfigResponse)]
    Config {},
}

#[cw_serde]
pub struct ConfigResponse {
    pub admin: Addr,
    pub trader: Addr,
    pub allowed_swap_routes: Vec<Vec<SwapRoute>>,
}

#[cw_serde]
pub enum MigrateMsg {}
