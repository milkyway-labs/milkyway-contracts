use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cw_storage_plus::Item;

use crate::state::SwapRoute;

#[cw_serde]
pub struct Config {
    /// User that will be allowed to perform the swap operations.
    pub trader: Addr,
    /// List of allowed swap routes that can be taken when performing a SwapExactAmountIn.
    pub allowed_swap_routes: Vec<Vec<SwapRoute>>,
}

pub const CONFIG: Item<Config> = Item::new("config");
