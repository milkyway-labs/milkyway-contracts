use cosmwasm_std::testing::{mock_env, mock_info};

use crate::{
    contract::execute,
    error::ContractError,
    msg::ExecuteMsg,
    state::{SwapRoute, CONFIG},
    tests::osmosis_querier::INVALID_POOL_DENOM,
    tests::test_helper::{ADMIN, TRADER},
    types::{UnsafeNativeChainConfig, UnsafeProtocolChainConfig},
};

use super::test_helper::{init, NEW_TRADER};

#[test]
fn update_config_not_admin_fail() {
    let mut deps = init();

    let update_config = ExecuteMsg::UpdateConfig {
        allowed_swap_routes: Some(vec![]),
        trader: Some(NEW_TRADER.to_string()),
        protocol_chain_config: None,
        native_chain_config: None,
    };
    let err = execute(
        deps.as_mut(),
        mock_env(),
        mock_info(TRADER, &[]),
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
        protocol_chain_config: None,
        native_chain_config: None,
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
fn update_config_invalid_protocol_chain_account_address_prefix_fails() {
    let mut deps = init();

    let update_config = ExecuteMsg::UpdateConfig {
        allowed_swap_routes: Some(vec![]),
        trader: Some(NEW_TRADER.to_string()),
        protocol_chain_config: Some(UnsafeProtocolChainConfig {
            account_address_prefix: "osmo12".to_string(),
        }),
        native_chain_config: Some(UnsafeNativeChainConfig {
            account_address_prefix: "celestia".to_string(),
        }),
    };
    execute(
        deps.as_mut(),
        mock_env(),
        mock_info(ADMIN, &[]),
        update_config,
    )
    .unwrap_err();
}

#[test]
fn update_config() {
    let mut deps = init();

    let update_config = ExecuteMsg::UpdateConfig {
        allowed_swap_routes: Some(vec![]),
        trader: Some(NEW_TRADER.to_string()),
        protocol_chain_config: Some(UnsafeProtocolChainConfig {
            account_address_prefix: "osmo".to_string(),
        }),
        native_chain_config: Some(UnsafeNativeChainConfig {
            account_address_prefix: "celestia".to_string(),
        }),
    };
    execute(
        deps.as_mut(),
        mock_env(),
        mock_info(ADMIN, &[]),
        update_config,
    )
    .unwrap();

    let config = CONFIG.load(&deps.storage).unwrap();
    assert_eq!(NEW_TRADER, config.trader);
    assert!(config.allowed_swap_routes.is_empty());
    assert_eq!("osmo", config.protocol_chain_config.account_address_prefix);
    assert_eq!(
        "celestia",
        config.native_chain_config.account_address_prefix
    );
}
