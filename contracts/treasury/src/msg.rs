use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Coin;

use crate::state::SwapRoute;

#[cw_serde]
pub struct InstantiateMsg {
    pub trader: Option<String>,
    pub allowed_swap_routes: Vec<SwapRoute>,
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
    SwapExactAmountIn {
        routes: Vec<SwapRoute>,
        token_in: Coin,
        token_out_min_amount: u128,
    },
    SwapExactAmountOut {
        routes: Vec<SwapRoute>,
        token_out: Coin,
        token_in_max_amount: u128,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {}
