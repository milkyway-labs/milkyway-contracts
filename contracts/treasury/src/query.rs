use cosmwasm_std::Deps;

use crate::{
    error::ContractResult,
    msg::ConfigResponse,
    state::{ADMIN, CONFIG},
};

pub fn query_config(deps: Deps) -> ContractResult<ConfigResponse> {
    let config = CONFIG.load(deps.storage)?;
    let admin = ADMIN
        .get(deps)?
        .expect("admin not present in the contract state");

    Ok(ConfigResponse {
        admin,
        trader: config.trader,
        allowed_swap_routes: config.allowed_swap_routes,
    })
}
