use cosmwasm_std::{
    testing::{message_info, mock_env},
    Addr,
};

use crate::{
    contract::execute, error::ContractError, msg::ExecuteMsg, state::CONFIG,
    tests::test_helper::ADMIN,
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
        message_info(&new_trader, &[]),
        update_config,
    )
    .unwrap_err();

    assert!(matches!(err, ContractError::Admin { .. }));
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
        message_info(&Addr::unchecked(ADMIN), &[]),
        update_config,
    )
    .unwrap();

    let config = CONFIG.load(&deps.storage).unwrap();
    assert_eq!(new_trader, config.trader);
    assert!(config.allowed_swap_routes.is_empty());
}
