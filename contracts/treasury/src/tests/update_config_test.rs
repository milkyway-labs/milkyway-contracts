use cosmwasm_std::testing::{mock_env, mock_info};

use crate::{
    contract::execute,
    error::ContractError,
    msg::ExecuteMsg,
    state::{SwapRoute, CONFIG},
    tests::{osmosis_querier::INVALID_POOL_DENOM, test_helper::ADMIN},
};

use super::test_helper::init;

#[test]
fn update_config_not_admin_fail() {
    let mut deps = init();

    let new_trader = deps.api.addr_make("new_trader");
    let update_config = ExecuteMsg::UpdateConfig {
        allowed_swap_routes: Some(vec![]),
        trader: Some(new_trader.to_string()),
    };
    let err = execute(
        deps.as_mut(),
        mock_env(),
        mock_info(new_trader.as_str(), &[]),
        update_config,
    )
    .unwrap_err();

    assert!(matches!(err, ContractError::Admin { .. }));
}

#[test]
fn update_config_with_invalid_route_fails() {
    let mut deps = init();

    let new_trader = deps.api.addr_make("new_trader");
    let update_config = ExecuteMsg::UpdateConfig {
        allowed_swap_routes: Some(vec![vec![SwapRoute {
            pool_id: 3,
            token_in_denom: INVALID_POOL_DENOM.to_string(),
            token_out_denom: INVALID_POOL_DENOM.to_string(),
        }]]),
        trader: Some(new_trader.to_string()),
    };
    let err = execute(
        deps.as_mut(),
        mock_env(),
        mock_info(ADMIN, &[]),
        update_config,
    );

    assert!(err.is_err());
}

#[test]
fn update_config() {
    let mut deps = init();

    let new_trader = deps.api.addr_make("new_trader");
    let update_config = ExecuteMsg::UpdateConfig {
        allowed_swap_routes: Some(vec![]),
        trader: Some(new_trader.to_string()),
    };
    execute(
        deps.as_mut(),
        mock_env(),
        mock_info(ADMIN, &[]),
        update_config,
    )
    .unwrap();

    let config = CONFIG.load(&deps.storage).unwrap();
    assert_eq!(new_trader, config.trader);
    assert!(config.allowed_swap_routes.is_empty());
}
