use cosmwasm_std::{Addr, Deps, StdError, StdResult};
use osmosis_std::types::osmosis::poolmanager::v1beta1::{PoolmanagerQuerier, SwapAmountInRoute};

use crate::{
    error::{ContractError, ContractResult},
    state::SwapRoute,
};

pub fn validate_address(address: &String, prefix: &str) -> StdResult<Addr> {
    let validated_addr =
        bech32::decode(address).map_err(|_| StdError::generic_err("Invalid address"))?;

    if validated_addr.0 != prefix {
        return Err(StdError::generic_err("Invalid address prefix"));
    }

    Ok(Addr::unchecked(address))
}

/// Ensure that all the provided swap routes exists.
pub fn validate_swap_routes(deps: Deps, routes: &[Vec<SwapRoute>]) -> ContractResult<()> {
    if routes.is_empty() {
        return Ok(());
    }

    let querier = PoolmanagerQuerier::new(&deps.querier);
    for (index, route) in routes.iter().enumerate() {
        if route.is_empty() {
            return Err(ContractError::InvalidSwapRoute {
                index,
                reason: "empty".to_string(),
            });
        }

        let start_pool = route.first().unwrap();
        querier
            .estimate_swap_exact_amount_in(
                start_pool.pool_id,
                start_pool.token_in_denom.clone(),
                route
                    .iter()
                    .map(|swap_route| SwapAmountInRoute {
                        pool_id: swap_route.pool_id,
                        token_out_denom: swap_route.token_out_denom.clone(),
                    })
                    .collect(),
            )
            .map_err(|e| ContractError::InvalidSwapRoute {
                index,
                reason: e.to_string(),
            })?;
    }

    Ok(())
}
