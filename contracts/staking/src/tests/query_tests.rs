#[cfg(test)]
mod query_tests {
    // use serde_json;
    use crate::contract::{execute, query};
    use crate::msg::{
        BatchResponse, BatchesResponse, ConfigResponse, ExecuteMsg, LiquidUnstakeRequestResponse,
        QueryMsg, StateResponse,
    };
    use crate::query::query_pending_batch;
    use crate::state::{CONFIG, STATE};
    use crate::tests::test_helper::{
        init, CELESTIAVAL1, CELESTIAVAL2, CHANNEL_ID, NATIVE_TOKEN, OSMO1, OSMO2, OSMO3,
    };
    use cosmwasm_std::testing::{mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary, Decimal, Uint128};

    #[test]
    fn get_config() {
        let deps = init();
        let msg = QueryMsg::Config {};
        let bin = query(deps.as_ref(), mock_env(), msg.clone()).unwrap();
        let result = from_binary::<ConfigResponse>(&bin);
        match result {
            Ok(res) => {
                assert_eq!(res.native_token_denom, NATIVE_TOKEN.to_string());
                assert_eq!(
                    res.liquid_stake_token_denom,
                    "factory/cosmos2contract/stTIA".to_string()
                );
                assert_eq!(res.treasury_address, OSMO1.to_string());
                assert_eq!(res.monitors, vec![OSMO2.to_string(), OSMO3.to_string()]);
                assert_eq!(
                    res.validators,
                    vec![CELESTIAVAL1.to_string(), CELESTIAVAL2.to_string()]
                );
                assert_eq!(res.batch_period, 86400);
                assert_eq!(res.unbonding_period, 1209600);
                assert_eq!(res.minimum_liquid_stake_amount, Uint128::from(100u128));
                assert_eq!(res.ibc_channel_id, CHANNEL_ID.to_string());
                assert_eq!(res.stopped, false);
            }
            Err(e) => match e {
                _ => panic!("Unexpected error: {:?}", e),
            },
        }
    }

    #[test]
    fn get_state() {
        let mut deps = init();
        let msg = QueryMsg::State {};
        let mut bin = query(deps.as_ref(), mock_env(), msg.clone()).unwrap();
        let mut result = from_binary::<StateResponse>(&bin);

        match result {
            Ok(res) => {
                assert_eq!(res.total_native_token, Uint128::from(0u128));
                assert_eq!(res.total_liquid_stake_token, Uint128::from(0u128));
                assert_eq!(res.rate, Decimal::zero());
                assert_eq!(res.pending_owner, "".to_string());
                assert_eq!(res.total_reward_amount, Uint128::from(0u128));
            }
            Err(e) => match e {
                _ => panic!("Unexpected error: {:?}", e),
            },
        }

        // stake
        let info = mock_info("creator", &coins(1000, NATIVE_TOKEN));
        let stake_msg = ExecuteMsg::LiquidStake {
            original_sender: None,
            expected_mint_amount: None,
        };
        let res = execute(deps.as_mut(), mock_env(), info, stake_msg);
        assert!(res.is_ok());

        // check the state
        bin = query(deps.as_ref(), mock_env(), msg.clone()).unwrap();
        result = from_binary::<StateResponse>(&bin);
        match result {
            Ok(res) => {
                assert_eq!(res.total_native_token, Uint128::from(1000u128));
                assert_eq!(res.total_liquid_stake_token, Uint128::from(1000u128));
                assert_eq!(
                    res.rate,
                    Decimal::from_ratio(res.total_liquid_stake_token, res.total_native_token)
                );
            }
            Err(e) => match e {
                _ => panic!("Unexpected error: {:?}", e),
            },
        }
    }

    #[test]
    fn get_batch() {
        let mut deps = init();
        let msg = QueryMsg::Batch { id: 1 };
        let bin = query(deps.as_ref(), mock_env(), msg).unwrap();
        let result = from_binary::<BatchResponse>(&bin);

        // we have 1 pending batch at the beginning
        match result {
            Ok(res) => {
                assert_eq!(res.batch_total_liquid_stake, Uint128::from(0u128));
                assert_eq!(res.expected_native_unstaked, Uint128::from(0u128));
                assert_eq!(res.status, "pending".to_string());
                assert_eq!(res.requests.len(), 0);
            }
            Err(e) => match e {
                _ => panic!("Unexpected error: {:?}", e),
            },
        }
        // find Batch with id = 2
        let msg = QueryMsg::Batch { id: 2 };
        let result = query(deps.as_ref(), mock_env(), msg);
        assert!(result.is_err()); //not found

        // unStake 1
        let info = mock_info("bob", &coins(500, "factory/cosmos2contract/stTIA"));
        let unstake_msg = ExecuteMsg::LiquidUnstake {};
        let res = execute(deps.as_mut(), mock_env(), info, unstake_msg.clone());
        assert!(res.is_ok());

        // unStake 2
        let info = mock_info("alice", &coins(1500, "factory/cosmos2contract/stTIA"));
        let res = execute(deps.as_mut(), mock_env(), info, unstake_msg);
        assert!(res.is_ok());

        let msg = QueryMsg::Batch { id: 1 };
        let bin = query(deps.as_ref(), mock_env(), msg).unwrap();
        let result = from_binary::<BatchResponse>(&bin);
        match result {
            Ok(res) => {
                assert_eq!(res.batch_total_liquid_stake, Uint128::from(2000u128));
                assert_eq!(res.expected_native_unstaked, Uint128::from(0u128));
                assert_eq!(res.status, "pending".to_string());
                assert_eq!(res.requests.len(), 2);
                assert_eq!(
                    res.requests,
                    vec![
                        LiquidUnstakeRequestResponse {
                            user: "alice".to_string(),
                            amount: Uint128::from(1500u128),
                            redeemed: false,
                        },
                        LiquidUnstakeRequestResponse {
                            user: "bob".to_string(),
                            amount: Uint128::from(500u128),
                            redeemed: false,
                        },
                    ]
                )
            }
            Err(e) => match e {
                _ => panic!("Unexpected error: {:?}", e),
            },
        }
    }

    #[test]
    fn get_batches() {
        let mut deps = init();
        let mut env = mock_env();
        let msg = QueryMsg::Batches {
            start_after: None,
            limit: None,
            status: None,
        };
        let mut bin = query(deps.as_ref(), env.clone(), msg.clone()).unwrap();
        let mut result = from_binary::<BatchesResponse>(&bin);

        // TODO: need to do something like circuit break, add the test as well
        // 1. total_liquid_stake_token < unstake amount
        // 2. total_liquid_stake_token == 0
        let mut state = STATE.load(&deps.storage).unwrap();
        state.total_liquid_stake_token = Uint128::from(100_000_000u128);
        STATE.save(&mut deps.storage, &state).unwrap();

        match result {
            Ok(res) => {
                assert_eq!(res.batches.len(), 1);
                if let Some(first_batch) = res.batches.get(0) {
                    assert_eq!(first_batch.batch_total_liquid_stake, Uint128::from(0u128));
                    assert_eq!(first_batch.expected_native_unstaked, Uint128::from(0u128));
                    assert_eq!(first_batch.status, "pending".to_string());
                } else {
                    panic!("batches is empty");
                }
            }
            Err(e) => match e {
                _ => panic!("Unexpected error: {:?}", e),
            },
        }

        // unStake 1
        let info = mock_info("bob", &coins(500, "factory/cosmos2contract/stTIA"));
        let unstake_msg = ExecuteMsg::LiquidUnstake {};
        let res = execute(deps.as_mut(), env.clone(), info, unstake_msg.clone());
        assert!(res.is_ok());

        // submit batch
        let config = CONFIG.load(&deps.storage).unwrap();
        env.block.time = env.block.time.plus_seconds(config.batch_period + 1);
        let submit_batch_msg = ExecuteMsg::SubmitBatch {};
        let contract = env.contract.address.clone().to_string();
        let submit_info = mock_info(&contract, &[]);
        let res = execute(deps.as_mut(), env.clone(), submit_info, submit_batch_msg);
        assert!(res.is_ok());

        // unStake 2 - for the next batch
        let info = mock_info("alice", &coins(1500, "factory/cosmos2contract/stTIA"));
        let res = execute(deps.as_mut(), env.clone(), info, unstake_msg);
        assert!(res.is_ok());

        // check the state
        bin = query(deps.as_ref(), env.clone(), msg).unwrap();
        result = from_binary::<BatchesResponse>(&bin);
        match result {
            Ok(res) => {
                assert_eq!(res.batches.len(), 2);
                if let Some(first_batch) = res.batches.get(0) {
                    assert_eq!(first_batch.batch_total_liquid_stake, Uint128::from(500u128));
                    assert_eq!(first_batch.expected_native_unstaked, Uint128::from(0u128));
                    assert_eq!(first_batch.status, "submitted".to_string());
                    assert_eq!(first_batch.requests.len(), 1);
                    assert_eq!(
                        first_batch.requests,
                        vec![LiquidUnstakeRequestResponse {
                            user: "bob".to_string(),
                            amount: Uint128::from(500u128),
                            redeemed: false,
                        }]
                    )
                } else {
                    panic!("batches is empty");
                }

                if let Some(first_batch) = res.batches.get(1) {
                    assert_eq!(
                        first_batch.batch_total_liquid_stake,
                        Uint128::from(1500u128)
                    );
                    assert_eq!(first_batch.expected_native_unstaked, Uint128::from(0u128));
                    assert_eq!(first_batch.status, "pending".to_string());
                    assert_eq!(first_batch.requests.len(), 1);
                    assert_eq!(
                        first_batch.requests,
                        vec![LiquidUnstakeRequestResponse {
                            user: "alice".to_string(),
                            amount: Uint128::from(1500u128),
                            redeemed: false,
                        }]
                    )
                } else {
                    panic!("batches is empty");
                }
            }
            Err(e) => match e {
                _ => panic!("Unexpected error: {:?}", e),
            },
        }

        // query only submitted batches
        let msg = QueryMsg::Batches {
            start_after: None,
            limit: None,
            status: Some(milky_way::staking::BatchStatus::Submitted),
        };
        bin = query(deps.as_ref(), env.clone(), msg).unwrap();
        result = from_binary::<BatchesResponse>(&bin);
        let result = result.unwrap();
        assert_eq!(result.batches.len(), 1);
    }

    #[test]
    fn get_pending_batch() {
        let mut deps = init();
        let mut env = mock_env();

        let pending_batch_id = query_pending_batch(deps.as_ref());
        assert!(pending_batch_id.unwrap().id == 1);

        let config = CONFIG.load(&deps.storage).unwrap();

        let mut state = STATE.load(&deps.storage).unwrap();

        state.total_liquid_stake_token = Uint128::from(100_000u128);
        STATE.save(&mut deps.storage, &state).unwrap();

        let info = mock_info("bob", &coins(1000, "factory/cosmos2contract/stTIA"));
        let msg = ExecuteMsg::LiquidUnstake {};
        let _res = execute(deps.as_mut(), mock_env(), info.clone(), msg);

        env.block.time = env.block.time.plus_seconds(config.batch_period + 1);
        let submit_batch_msg = ExecuteMsg::SubmitBatch {};
        let contract = env.contract.address.clone().to_string();
        let submit_info = mock_info(&contract, &[]);
        let res = execute(deps.as_mut(), env.clone(), submit_info, submit_batch_msg);
        if res.is_err() {
            // print error
            let err = res.err().unwrap();
            panic!("Error: {:?}", err);
        }
        assert!(res.is_ok());

        let pending_batch_id = query_pending_batch(deps.as_ref());
        assert!(pending_batch_id.unwrap().id == 2);
    }
}
