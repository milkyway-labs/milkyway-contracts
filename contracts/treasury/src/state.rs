use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cosmwasm_std::Timestamp;
use cw_controllers::Admin;
use cw_storage_plus::Item;

use crate::error::ContractError;
use crate::error::ContractResult;

#[cw_serde]
pub struct State {
    pub pending_owner: Option<Addr>,
    pub owner_transfer_min_time: Option<Timestamp>,
}

pub const ADMIN: Admin = Admin::new("admin");
pub const STATE: Item<State> = Item::new("state");

#[cw_serde]
pub struct SwapRoute {
    /// Id of the pool where the swap will be performed.
    pub pool_id: u64,
    /// Denom of the coin that will be swapped.
    pub token_in_denom: String,
    /// Denom of the coin that will be received after the swap.
    pub token_out_denom: String,
}

#[cw_serde]
pub struct Config {
    /// User that will be allowed to perform the swap operations.
    pub trader: Addr,
    /// List of allowed swap routes that can be taken when performing a SwapExactAmountIn.
    pub allowed_swap_routes: Vec<Vec<SwapRoute>>,
    /// Config related to the chain for which we are creating
    /// the LST token.
    /// For example Celestia is the native chain of milkTIA LST token.
    pub native_chain_config: NativeChainConfig,
    /// Config related to the chain where the smart contract is deployed.
    pub protocol_chain_config: ProtocolChainConfig,
}

/// Config related to the chain for which we are creating
/// the LST token.
/// For example Celestia is the native chain of milkTIA LST token.
#[cw_serde]
pub struct NativeChainConfig {
    /// Bech32 prefix for accounts (e.g. "celestia", "initia", etc)
    pub account_address_prefix: String,
}

/// Config related to the chain where the smart contract is deployed.
#[cw_serde]
pub struct ProtocolChainConfig {
    /// Bech32 prefix for accounts (e.g. "osmo", "milk", etc)
    pub account_address_prefix: String,
}

pub const CONFIG: Item<Config> = Item::new("config");

impl Config {
    pub fn assert_trader(&self, address: &Addr) -> ContractResult<()> {
        if self.trader.eq(address) {
            Ok(())
        } else {
            Err(ContractError::Unauthorized {
                sender: address.to_string(),
            })
        }
    }

    pub fn assert_allowed_swap_route(&self, swap_route: &[SwapRoute]) -> ContractResult<()> {
        if swap_route.is_empty() {
            return Err(ContractError::SwapRouteNotAllowed {});
        }
        if self
            .allowed_swap_routes
            .iter()
            .any(|allowed_route| allowed_route.eq(swap_route))
        {
            Ok(())
        } else {
            Err(ContractError::SwapRouteNotAllowed {})
        }
    }
}
