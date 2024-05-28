use cosmwasm_std::{
    testing::{mock_env, mock_info},
    Coin, CosmosMsg, ReplyOn, SubMsg,
};
use osmosis_std::types::cosmos::base::v1beta1::Coin as OsmosisCoin;
use osmosis_std::types::osmosis::poolmanager::v1beta1::{
    MsgSwapExactAmountIn, MsgSwapExactAmountOut, SwapAmountInRoute, SwapAmountOutRoute,
};

use crate::{
    contract::execute,
    error::ContractError,
    msg::ExecuteMsg,
    state::SwapRoute,
    tests::test_helper::{OSMO_DENOM, OSMO_USDC_POOL, TIA_OSMO_POOL, TIA_USDC_POOL, TRADER},
};

use super::test_helper::{init, TIA_DENOM, USDC_DENOM};

#[test]
fn swap_amount_in_from_user_fail() {
    let mut deps = init();

    let msg = ExecuteMsg::SwapExactAmountIn {
        routes: vec![SwapRoute {
            pool_id: TIA_USDC_POOL,
            token_in_denom: TIA_DENOM.to_string(),
            token_out_denom: USDC_DENOM.to_string(),
        }],
        token_in: Coin::new(1000u128, TIA_DENOM),
        token_out_min_amount: 100,
    };
    let user_addr = deps.api.addr_make("user");
    let error = execute(
        deps.as_mut(),
        mock_env(),
        mock_info(user_addr.as_str(), &[]),
        msg,
    )
    .unwrap_err();

    assert_eq!(
        ContractError::Unauthorized {
            sender: user_addr.to_string()
        },
        error
    );
}

#[test]
fn swap_amount_in_with_empty_routes_fail() {
    let mut deps = init();

    let msg = ExecuteMsg::SwapExactAmountIn {
        routes: vec![],
        token_in: Coin::new(1000u128, TIA_DENOM),
        token_out_min_amount: 100,
    };
    let error = execute(deps.as_mut(), mock_env(), mock_info(TRADER, &[]), msg).unwrap_err();
    assert_eq!(ContractError::SwapRouteNotAllowed {}, error);
}

#[test]
fn swap_amount_in_from_unauthorized_pool_fail() {
    let mut deps = init();

    let msg = ExecuteMsg::SwapExactAmountIn {
        routes: vec![SwapRoute {
            // Not allowed pool
            pool_id: 99,
            token_in_denom: TIA_DENOM.to_string(),
            token_out_denom: USDC_DENOM.to_string(),
        }],
        token_in: Coin::new(1000u128, TIA_DENOM),
        token_out_min_amount: 100,
    };
    let error = execute(deps.as_mut(), mock_env(), mock_info(TRADER, &[]), msg).unwrap_err();
    assert_eq!(ContractError::SwapRouteNotAllowed {}, error);

    // Test multi hop
    let msg = ExecuteMsg::SwapExactAmountIn {
        routes: vec![
            SwapRoute {
                // Allowed pool
                pool_id: TIA_OSMO_POOL,
                token_in_denom: TIA_DENOM.to_string(),
                token_out_denom: OSMO_DENOM.to_string(),
            },
            SwapRoute {
                // Not allowed pool
                pool_id: 99,
                token_in_denom: OSMO_DENOM.to_string(),
                token_out_denom: USDC_DENOM.to_string(),
            },
        ],
        token_in: Coin::new(1000u128, TIA_DENOM),
        token_out_min_amount: 100,
    };
    let error = execute(deps.as_mut(), mock_env(), mock_info(TRADER, &[]), msg).unwrap_err();
    assert_eq!(ContractError::SwapRouteNotAllowed {}, error);
}

#[test]
fn swap_amount_in_with_unauthorized_token_in_denom_fail() {
    let mut deps = init();

    let msg = ExecuteMsg::SwapExactAmountIn {
        routes: vec![SwapRoute {
            pool_id: TIA_USDC_POOL,
            token_in_denom: USDC_DENOM.to_string(),
            token_out_denom: USDC_DENOM.to_string(),
        }],
        token_in: Coin::new(1000u128, TIA_DENOM),
        token_out_min_amount: 100,
    };
    let error = execute(deps.as_mut(), mock_env(), mock_info(TRADER, &[]), msg).unwrap_err();
    assert_eq!(ContractError::SwapRouteNotAllowed {}, error);

    // Test convert USDC back to TIA using the token in denom.
    let msg = ExecuteMsg::SwapExactAmountIn {
        routes: vec![SwapRoute {
            pool_id: TIA_USDC_POOL,
            token_in_denom: TIA_DENOM.to_string(),
            token_out_denom: USDC_DENOM.to_string(),
        }],
        token_in: Coin::new(1000u128, USDC_DENOM),
        token_out_min_amount: 100,
    };
    let error = execute(deps.as_mut(), mock_env(), mock_info(TRADER, &[]), msg).unwrap_err();
    assert_eq!(
        ContractError::InvalidTokenInDenom {
            denom: USDC_DENOM.to_string()
        },
        error
    );

    // Test multi hop
    let msg = ExecuteMsg::SwapExactAmountIn {
        routes: vec![
            SwapRoute {
                pool_id: TIA_OSMO_POOL,
                token_in_denom: TIA_DENOM.to_string(),
                token_out_denom: OSMO_DENOM.to_string(),
            },
            SwapRoute {
                pool_id: OSMO_USDC_POOL,
                token_in_denom: USDC_DENOM.to_string(),
                token_out_denom: OSMO_DENOM.to_string(),
            },
        ],
        token_in: Coin::new(1000u128, TIA_DENOM),
        token_out_min_amount: 100,
    };
    let error = execute(deps.as_mut(), mock_env(), mock_info(TRADER, &[]), msg).unwrap_err();
    assert_eq!(ContractError::SwapRouteNotAllowed {}, error);
}

#[test]
fn swap_amount_in_with_unauthorized_token_out_denom_fail() {
    let mut deps = init();

    let msg = ExecuteMsg::SwapExactAmountIn {
        routes: vec![SwapRoute {
            pool_id: TIA_USDC_POOL,
            token_in_denom: TIA_DENOM.to_string(),
            token_out_denom: TIA_DENOM.to_string(),
        }],
        token_in: Coin::new(1000u128, TIA_DENOM),
        token_out_min_amount: 100,
    };
    let error = execute(deps.as_mut(), mock_env(), mock_info(TRADER, &[]), msg).unwrap_err();

    assert_eq!(ContractError::SwapRouteNotAllowed {}, error);

    let msg = ExecuteMsg::SwapExactAmountIn {
        routes: vec![
            SwapRoute {
                pool_id: TIA_OSMO_POOL,
                token_in_denom: TIA_DENOM.to_string(),
                token_out_denom: OSMO_DENOM.to_string(),
            },
            SwapRoute {
                pool_id: OSMO_USDC_POOL,
                token_in_denom: USDC_DENOM.to_string(),
                token_out_denom: USDC_DENOM.to_string(),
            },
        ],
        token_in: Coin::new(1000u128, TIA_DENOM),
        token_out_min_amount: 100,
    };
    let error = execute(deps.as_mut(), mock_env(), mock_info(TRADER, &[]), msg).unwrap_err();

    assert_eq!(ContractError::SwapRouteNotAllowed {}, error);
}

#[test]
fn swap_amount_in() {
    let mut deps = init();

    let msg = ExecuteMsg::SwapExactAmountIn {
        routes: vec![SwapRoute {
            pool_id: TIA_USDC_POOL,
            token_in_denom: TIA_DENOM.to_string(),
            token_out_denom: USDC_DENOM.to_string(),
        }],
        token_in: Coin::new(1000u128, TIA_DENOM),
        token_out_min_amount: 100,
    };
    let env = mock_env();
    let response = execute(deps.as_mut(), env.clone(), mock_info(TRADER, &[]), msg).unwrap();

    let messages = response.messages;
    assert_eq!(1, messages.len());
    assert_eq!(
        messages[0],
        SubMsg {
            id: 0,
            msg: <MsgSwapExactAmountIn as Into<CosmosMsg>>::into(MsgSwapExactAmountIn {
                sender: env.contract.address.to_string(),
                routes: vec![SwapAmountInRoute {
                    pool_id: TIA_USDC_POOL,
                    token_out_denom: USDC_DENOM.to_string(),
                }],
                token_in: Some(OsmosisCoin {
                    denom: TIA_DENOM.to_string(),
                    amount: "1000".to_string(),
                }),
                token_out_min_amount: "100".to_string(),
            }),
            gas_limit: None,
            reply_on: ReplyOn::Never,
        }
    );

    let msg = ExecuteMsg::SwapExactAmountIn {
        routes: vec![
            SwapRoute {
                pool_id: TIA_OSMO_POOL,
                token_in_denom: TIA_DENOM.to_string(),
                token_out_denom: OSMO_DENOM.to_string(),
            },
            SwapRoute {
                pool_id: OSMO_USDC_POOL,
                token_in_denom: OSMO_DENOM.to_string(),
                token_out_denom: USDC_DENOM.to_string(),
            },
        ],
        token_in: Coin::new(1000u128, TIA_DENOM),
        token_out_min_amount: 100,
    };
    let env = mock_env();
    let response = execute(deps.as_mut(), env.clone(), mock_info(TRADER, &[]), msg).unwrap();

    let messages = response.messages;
    assert_eq!(1, messages.len());
    assert_eq!(
        messages[0],
        SubMsg {
            id: 0,
            msg: <MsgSwapExactAmountIn as Into<CosmosMsg>>::into(MsgSwapExactAmountIn {
                sender: env.contract.address.to_string(),
                routes: vec![
                    SwapAmountInRoute {
                        pool_id: TIA_OSMO_POOL,
                        token_out_denom: OSMO_DENOM.to_string(),
                    },
                    SwapAmountInRoute {
                        pool_id: OSMO_USDC_POOL,
                        token_out_denom: USDC_DENOM.to_string(),
                    }
                ],
                token_in: Some(OsmosisCoin {
                    denom: TIA_DENOM.to_string(),
                    amount: "1000".to_string(),
                }),
                token_out_min_amount: "100".to_string(),
            }),
            gas_limit: None,
            reply_on: ReplyOn::Never,
        }
    );
}

#[test]
fn swap_amount_out_from_user_fail() {
    let mut deps = init();

    let msg = ExecuteMsg::SwapExactAmountOut {
        routes: vec![SwapRoute {
            pool_id: TIA_USDC_POOL,
            token_in_denom: TIA_DENOM.to_string(),
            token_out_denom: USDC_DENOM.to_string(),
        }],
        token_out: Coin::new(1000u128, USDC_DENOM),
        token_in_max_amount: 100,
    };
    let user_addr = deps.api.addr_make("user");
    let error = execute(
        deps.as_mut(),
        mock_env(),
        mock_info(user_addr.as_str(), &[]),
        msg,
    )
    .unwrap_err();

    assert_eq!(
        ContractError::Unauthorized {
            sender: user_addr.to_string()
        },
        error
    );
}

#[test]
fn swap_amount_out_with_empty_routes_fail() {
    let mut deps = init();

    let msg = ExecuteMsg::SwapExactAmountOut {
        routes: vec![],
        token_out: Coin::new(1000u128, USDC_DENOM),
        token_in_max_amount: 100,
    };
    let error = execute(deps.as_mut(), mock_env(), mock_info(TRADER, &[]), msg).unwrap_err();
    assert_eq!(ContractError::SwapRouteNotAllowed {}, error);
}

#[test]
fn swap_amount_out_from_unauthorized_pool_fail() {
    let mut deps = init();

    let msg = ExecuteMsg::SwapExactAmountOut {
        routes: vec![SwapRoute {
            pool_id: 99,
            token_in_denom: TIA_DENOM.to_string(),
            token_out_denom: USDC_DENOM.to_string(),
        }],
        token_out: Coin::new(1000u128, USDC_DENOM),
        token_in_max_amount: 100,
    };
    let error = execute(deps.as_mut(), mock_env(), mock_info(TRADER, &[]), msg).unwrap_err();
    assert_eq!(ContractError::SwapRouteNotAllowed {}, error);

    // Multi hop
    let msg = ExecuteMsg::SwapExactAmountOut {
        routes: vec![
            SwapRoute {
                pool_id: TIA_OSMO_POOL,
                token_in_denom: TIA_DENOM.to_string(),
                token_out_denom: OSMO_DENOM.to_string(),
            },
            SwapRoute {
                pool_id: 99,
                token_in_denom: OSMO_DENOM.to_string(),
                token_out_denom: USDC_DENOM.to_string(),
            },
        ],
        token_out: Coin::new(1000u128, USDC_DENOM),
        token_in_max_amount: 100,
    };
    let error = execute(deps.as_mut(), mock_env(), mock_info(TRADER, &[]), msg).unwrap_err();
    assert_eq!(ContractError::SwapRouteNotAllowed {}, error);
}

#[test]
fn swap_amount_out_with_unauthorized_token_in_denom_fail() {
    let mut deps = init();

    let msg = ExecuteMsg::SwapExactAmountOut {
        routes: vec![SwapRoute {
            pool_id: TIA_USDC_POOL,
            token_in_denom: USDC_DENOM.to_string(),
            token_out_denom: USDC_DENOM.to_string(),
        }],
        token_out: Coin::new(1000u128, USDC_DENOM),
        token_in_max_amount: 100,
    };
    let error = execute(deps.as_mut(), mock_env(), mock_info(TRADER, &[]), msg).unwrap_err();
    assert_eq!(ContractError::SwapRouteNotAllowed {}, error);

    // Multi hop
    let msg = ExecuteMsg::SwapExactAmountOut {
        routes: vec![
            SwapRoute {
                pool_id: TIA_OSMO_POOL,
                token_in_denom: TIA_DENOM.to_string(),
                token_out_denom: OSMO_DENOM.to_string(),
            },
            SwapRoute {
                pool_id: OSMO_USDC_POOL,
                token_in_denom: USDC_DENOM.to_string(),
                token_out_denom: OSMO_DENOM.to_string(),
            },
        ],
        token_out: Coin::new(1000u128, USDC_DENOM),
        token_in_max_amount: 100,
    };
    let error = execute(deps.as_mut(), mock_env(), mock_info(TRADER, &[]), msg).unwrap_err();
    assert_eq!(ContractError::SwapRouteNotAllowed {}, error);
}

#[test]
fn swap_amount_out_with_unauthorized_token_out_denom_fail() {
    let mut deps = init();

    let msg = ExecuteMsg::SwapExactAmountOut {
        routes: vec![SwapRoute {
            pool_id: TIA_USDC_POOL,
            token_in_denom: TIA_DENOM.to_string(),
            token_out_denom: TIA_DENOM.to_string(),
        }],
        token_out: Coin::new(1000u128, USDC_DENOM),
        token_in_max_amount: 100,
    };
    let error = execute(deps.as_mut(), mock_env(), mock_info(TRADER, &[]), msg).unwrap_err();

    assert_eq!(ContractError::SwapRouteNotAllowed {}, error);

    // Try using the token out coin denom.
    let msg = ExecuteMsg::SwapExactAmountOut {
        routes: vec![SwapRoute {
            pool_id: TIA_USDC_POOL,
            token_in_denom: TIA_DENOM.to_string(),
            token_out_denom: USDC_DENOM.to_string(),
        }],
        token_out: Coin::new(1000u128, TIA_DENOM),
        token_in_max_amount: 100,
    };
    let error = execute(deps.as_mut(), mock_env(), mock_info(TRADER, &[]), msg).unwrap_err();
    assert_eq!(
        ContractError::InvalidTokenOutDenom {
            denom: TIA_DENOM.to_string()
        },
        error
    );

    let msg = ExecuteMsg::SwapExactAmountOut {
        routes: vec![
            SwapRoute {
                pool_id: TIA_OSMO_POOL,
                token_in_denom: TIA_DENOM.to_string(),
                token_out_denom: OSMO_DENOM.to_string(),
            },
            SwapRoute {
                pool_id: OSMO_USDC_POOL,
                token_in_denom: OSMO_DENOM.to_string(),
                token_out_denom: OSMO_DENOM.to_string(),
            },
        ],
        token_out: Coin::new(1000u128, USDC_DENOM),
        token_in_max_amount: 100,
    };
    let error = execute(deps.as_mut(), mock_env(), mock_info(TRADER, &[]), msg).unwrap_err();
    assert_eq!(ContractError::SwapRouteNotAllowed {}, error);
}

#[test]
fn swap_amount_out() {
    let mut deps = init();

    let msg = ExecuteMsg::SwapExactAmountOut {
        routes: vec![SwapRoute {
            pool_id: TIA_USDC_POOL,
            token_in_denom: TIA_DENOM.to_string(),
            token_out_denom: USDC_DENOM.to_string(),
        }],
        token_out: Coin::new(1000u128, USDC_DENOM),
        token_in_max_amount: 100,
    };
    let env = mock_env();
    let response = execute(deps.as_mut(), env.clone(), mock_info(TRADER, &[]), msg).unwrap();

    let messages = response.messages;
    assert_eq!(1, messages.len());
    assert_eq!(
        messages[0],
        SubMsg {
            id: 0,
            msg: <MsgSwapExactAmountOut as Into<CosmosMsg>>::into(MsgSwapExactAmountOut {
                sender: env.contract.address.to_string(),
                routes: vec![SwapAmountOutRoute {
                    pool_id: TIA_USDC_POOL,
                    token_in_denom: TIA_DENOM.to_string(),
                }],
                token_out: Some(OsmosisCoin {
                    denom: USDC_DENOM.to_string(),
                    amount: "1000".to_string(),
                }),
                token_in_max_amount: "100".to_string(),
            }),
            gas_limit: None,
            reply_on: ReplyOn::Never,
        }
    );

    // Multi hop
    let msg = ExecuteMsg::SwapExactAmountOut {
        routes: vec![
            SwapRoute {
                pool_id: TIA_OSMO_POOL,
                token_in_denom: TIA_DENOM.to_string(),
                token_out_denom: OSMO_DENOM.to_string(),
            },
            SwapRoute {
                pool_id: OSMO_USDC_POOL,
                token_in_denom: OSMO_DENOM.to_string(),
                token_out_denom: USDC_DENOM.to_string(),
            },
        ],
        token_out: Coin::new(1000u128, USDC_DENOM),
        token_in_max_amount: 100,
    };
    let env = mock_env();
    let response = execute(deps.as_mut(), env.clone(), mock_info(TRADER, &[]), msg).unwrap();

    let messages = response.messages;
    assert_eq!(1, messages.len());
    assert_eq!(
        messages[0],
        SubMsg {
            id: 0,
            msg: <MsgSwapExactAmountOut as Into<CosmosMsg>>::into(MsgSwapExactAmountOut {
                sender: env.contract.address.to_string(),
                routes: vec![
                    SwapAmountOutRoute {
                        pool_id: TIA_OSMO_POOL,
                        token_in_denom: TIA_DENOM.to_string(),
                    },
                    SwapAmountOutRoute {
                        pool_id: OSMO_USDC_POOL,
                        token_in_denom: OSMO_DENOM.to_string(),
                    },
                ],
                token_out: Some(OsmosisCoin {
                    denom: USDC_DENOM.to_string(),
                    amount: "1000".to_string(),
                }),
                token_in_max_amount: "100".to_string(),
            }),
            gas_limit: None,
            reply_on: ReplyOn::Never,
        }
    );
}
