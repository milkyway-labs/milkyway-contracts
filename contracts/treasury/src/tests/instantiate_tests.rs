use cosmwasm_std::{
    coins,
    testing::{mock_env, mock_info},
};

use crate::{
    contract::instantiate,
    msg::InstantiateMsg,
    state::SwapRoute,
    tests::{
        osmosis_querier::INVALID_POOL_DENOM,
        test_helper::{mock_deps, ADMIN, TRADER},
    },
};

#[test]
fn instantiate_with_invalid_swap_route_fails() {
    let mut deps = mock_deps();

    let msg = InstantiateMsg {
        admin: Some(ADMIN.to_string()),
        trader: Some(TRADER.to_string()),
        allowed_swap_routes: vec![vec![SwapRoute {
            pool_id: 1,
            token_in_denom: INVALID_POOL_DENOM.to_string(),
            token_out_denom: INVALID_POOL_DENOM.to_string(),
        }]],
    };
    let info = mock_info(ADMIN, &coins(1000, "uosmo"));

    let res = instantiate(deps.as_mut(), mock_env(), info, msg);
    assert!(res.is_err());
}
