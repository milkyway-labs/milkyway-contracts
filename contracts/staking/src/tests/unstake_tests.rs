#[cfg(test)]
mod staking_tests {
    use crate::contract::execute;
    use crate::msg::ExecuteMsg;
    use crate::state::{BATCHES, STATE, CONFIG};
    use crate::tests::test_helper::init;
    use cosmwasm_std::testing::{mock_env, mock_info, MOCK_CONTRACT_ADDR};
    use cosmwasm_std::{coins, SubMsg, Addr, Uint128, CosmosMsg, ReplyOn};
    use osmosis_std::types::osmosis::tokenfactory::v1beta1::MsgBurn;
    use osmosis_std::types::cosmos::base::v1beta1::Coin;
    use milky_way::staking::{BatchStatus};

    #[test]
    fn proper_liquid_unstake() {
        let mut deps = init();

        let mut state = STATE.load(&deps.storage).unwrap();

        state.total_liquid_stake_token = Uint128::from(100_000u128);
        STATE.save(&mut deps.storage, &state).unwrap();

        let info = mock_info("bob", &coins(1000, "factory/cosmos2contract/stTIA"));
        let msg = ExecuteMsg::LiquidUnstake {};
        let mut res = execute(deps.as_mut(), mock_env(), info.clone(), msg);
        let resp = res.unwrap();

        let attrs = resp.attributes;
        assert_eq!(attrs[0].value, "liquid_unstake");
        assert_eq!(attrs[2].value, "1000");

        let batch = BATCHES.load(&deps.storage, 1).unwrap();
        assert_eq!(batch.batch_total_liquid_stake, Uint128::from(1000u128));

        // Submit batch
        // currently disabled auto batch submit
        // assert_eq!(resp.messages.len(), 1);
        let mut env = mock_env();
        let config = CONFIG.load(&deps.storage).unwrap();

        env.block.time = env.block.time.plus_seconds(config.batch_period + 1);
        let msg = ExecuteMsg::SubmitBatch { batch_id: 1 };
        res = execute(deps.as_mut(), env.clone(), info.clone(), msg);

        let attrs = res.unwrap().attributes;
        assert_eq!(attrs[0].value, "submit_batch");
        assert_eq!(attrs[2].value, "1000");
    }

    #[test]
    fn double_liquid_unstake() {
        let mut deps = init();

        let mut state = STATE.load(&deps.storage).unwrap();

        state.total_liquid_stake_token = Uint128::from(100_000u128);
        STATE.save(&mut deps.storage, &state).unwrap();
        let msg = ExecuteMsg::LiquidUnstake {};

        // Bob unstakes 100 
        let info = mock_info("bob", &coins(500, "factory/cosmos2contract/stTIA"));
        let mut res = execute(deps.as_mut(), mock_env(), info, msg.clone());
        assert!(res.is_ok());


        // Bob unstakes 1_000
        let info = mock_info("bob", &coins(1_000, "factory/cosmos2contract/stTIA"));
        res = execute(deps.as_mut(), mock_env(), info, msg.clone());
        assert!(res.is_ok());

        // Check pending batch
        let pending_batch = BATCHES.load(&deps.storage, 1).unwrap();
        assert!(pending_batch.liquid_unstake_requests.len() == 1);
        assert_eq!(
            pending_batch
                .liquid_unstake_requests
                .get("bob")
                .unwrap()
                .shares
                , Uint128::from(1500u128)
        );

        // Alice unstakes 5_000
        let info = mock_info("alice", &coins(5_000, "factory/cosmos2contract/stTIA"));
        res = execute(deps.as_mut(), mock_env(), info.clone(), msg);
        assert!(res.is_ok());

        // Check pending batch
        let pending_batch = BATCHES.load(&deps.storage, 1).unwrap();
        assert_eq!(pending_batch.batch_total_liquid_stake, Uint128::from(6_500u128));
        assert!(pending_batch.liquid_unstake_requests.len() == 2); //for bob & alice

        assert_eq!(
            pending_batch
                .liquid_unstake_requests
                .get("bob")
                .unwrap()
                .shares
                , Uint128::from(1_500u128)
        );
        assert_eq!(
            pending_batch
                .liquid_unstake_requests
                .get("alice")
                .unwrap()
                .shares
                , Uint128::from(5_000u128)
        );

        // submit batch
        let mut env = mock_env();
        let config = CONFIG.load(&deps.storage).unwrap();
        env.block.time = env.block.time.plus_seconds(config.batch_period + 1);

        let msg = ExecuteMsg::SubmitBatch { batch_id: 1 };
        res = execute(deps.as_mut(), env.clone(), info.clone(), msg);
        let resp = res.unwrap();
        let attrs = resp.attributes;
        assert_eq!(attrs[0].value, "submit_batch");
        assert_eq!(attrs[1].value, "1");
        assert_eq!(attrs[2].value, "6500");

        let messages = resp.messages;
        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0], SubMsg {
            id: 0,
            msg: <MsgBurn as Into<CosmosMsg>>::into(MsgBurn {
                sender: Addr::unchecked(MOCK_CONTRACT_ADDR).to_string(),
                amount: Some(Coin {
                    denom: "factory/cosmos2contract/stTIA".to_string(),
                    amount: "6500".to_string(),
                }),
                burn_from_address: Addr::unchecked(MOCK_CONTRACT_ADDR).to_string(),
            }
            ),
            gas_limit: None,
            reply_on: ReplyOn::Never,
        });

        // check the batch
        let batch = BATCHES.load(&deps.storage, 1).unwrap();
        assert_eq!(batch.batch_total_liquid_stake, Uint128::from(6500u128));
        assert_eq!(batch.status, BatchStatus::Submitted);
    }


    #[test]
    fn minimum_liquid_unstake() {
        let mut deps = init();

        let mut state = STATE.load(&deps.storage).unwrap();

        state.total_liquid_stake_token = Uint128::from(100_000u128);
        STATE.save(&mut deps.storage, &state).unwrap();

        // CHK
        // If minimum == 100, shouldn't we allow 100?
        // amount >= minimum
        let info = mock_info("bob", &coins(100, "factory/bob/stTIA"));
        let msg = ExecuteMsg::LiquidUnstake {};
        let res = execute(deps.as_mut(), mock_env(), info, msg);

        assert!(res.is_err());
    }

    #[test]
    fn invalid_denom_liquid_unstake() {
        let mut deps = init();

        let mut state = STATE.load(&deps.storage).unwrap();

        state.total_liquid_stake_token = Uint128::from(100_000u128);
        STATE.save(&mut deps.storage, &state).unwrap();

        let info = mock_info("bob", &coins(1000, "factory/bob/stTIA"));
        let msg = ExecuteMsg::LiquidUnstake {};

        let res = execute(deps.as_mut(), mock_env(), info, msg);

        assert!(res.is_err());
    }


    #[test]
    fn invalid_amount_liquid_unstake() {
        let mut deps = init();

        let mut state = STATE.load(&deps.storage).unwrap();

        state.total_liquid_stake_token = Uint128::from(100_000u128);
        STATE.save(&mut deps.storage, &state).unwrap();

        let info = mock_info("bob", &coins(1_000_000_000, "factory/cosmos2contract/stTIA"));
        let msg = ExecuteMsg::LiquidUnstake {};

        let res = execute(deps.as_mut(), mock_env(), info.clone(), msg);
        let mut resp = res.unwrap();

        let attrs = resp.attributes;
        assert_eq!(attrs[0].value, "liquid_unstake");
        assert_eq!(attrs[2].value, "1000000000");

        // CHK: Is that possible unstaking large amount than total_liquid_stake_token?
        // total_liquid_stake_token = 100_000
        // unstake = 1_000_000_000 
        let batch = BATCHES.load(&deps.storage, 1).unwrap();
        assert_eq!(batch.batch_total_liquid_stake, Uint128::from(1_000_000_000u128));

        // Submit batch
        // currently disabled auto batch submit
        // assert_eq!(resp.messages.len(), 1);
        let mut env = mock_env();
        let config = CONFIG.load(&deps.storage).unwrap();

        env.block.time = env.block.time.plus_seconds(config.batch_period + 1);
        let msg = ExecuteMsg::SubmitBatch { batch_id: 1 };
        let res = execute(deps.as_mut(), env.clone(), info.clone(), msg);
        resp = res.unwrap();

        let attrs = resp.attributes;
        assert_eq!(attrs[0].value, "submit_batch");
        assert_eq!(attrs[2].value, "1000000000");

        let messages = resp.messages;
        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0], SubMsg {
            id: 0,
            msg: <MsgBurn as Into<CosmosMsg>>::into(MsgBurn {
                sender: Addr::unchecked(MOCK_CONTRACT_ADDR).to_string(),
                amount: Some(Coin {
                    denom: "factory/cosmos2contract/stTIA".to_string(),
                    amount: "1000000000".to_string(),
                }),
                burn_from_address: Addr::unchecked(MOCK_CONTRACT_ADDR).to_string(),
            }
            ),
            gas_limit: None,
            reply_on: ReplyOn::Never,
        });

        // check the state
        state = STATE.load(&deps.storage).unwrap();
        assert_eq!(state.total_liquid_stake_token, Uint128::from(0u128));
        assert_eq!(state.total_native_token, Uint128::from(0u128));

        // check the batch
        let batch = BATCHES.load(&deps.storage, 1).unwrap();
        assert_eq!(batch.batch_total_liquid_stake, Uint128::from(1000000000u128));
        assert_eq!(batch.status, BatchStatus::Submitted);
    }
}
