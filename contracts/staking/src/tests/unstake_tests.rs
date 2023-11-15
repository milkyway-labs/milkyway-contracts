#[cfg(test)]
mod staking_tests {
    use crate::contract::execute;
    use crate::msg::ExecuteMsg;
    use crate::state::{BATCHES, STATE};
    use crate::tests::test_helper::init;
    use cosmwasm_std::testing::{mock_env, mock_info};
    use cosmwasm_std::{coins, Addr, Uint128};
    use milky_way::staking::LiquidUnstakeRequest;

    #[test]
    fn proper_liquid_unstake() {
        let mut deps = init();

        let mut state = STATE.load(&deps.storage).unwrap();

        state.total_liquid_stake_token = Uint128::from(100_000u128);
        STATE.save(&mut deps.storage, &state).unwrap();

        let info = mock_info("bob", &coins(1000, "factory/cosmos2contract/stTIA"));
        let msg = ExecuteMsg::LiquidUnstake {};

        let res = execute(deps.as_mut(), mock_env(), info.clone(), msg);

        let resp = res.unwrap();

        let attrs = resp.attributes;
        assert_eq!(attrs[0].value, "liquid_unstake");
        assert_eq!(attrs[2].value, "1000");

        let batch = BATCHES.load(&deps.storage, 1).unwrap();
        assert_eq!(batch.batch_total_liquid_stake, Uint128::from(1000u128));

        // Submit batch
        // currently disabled auto batch submit
        // assert_eq!(resp.messages.len(), 1);


        // print!("{:?}", msgs);
        // env.block.time = env.block.time.plus_seconds(config.batch_period + 1);
        // let msg = ExecuteMsg::SubmitBatch { batch_id: 1 };
    }
    #[test]
    fn double_liquid_unstake() {
        let mut deps = init();

        let mut state = STATE.load(&deps.storage).unwrap();

        state.total_liquid_stake_token = Uint128::from(100_000u128);
        STATE.save(&mut deps.storage, &state).unwrap();

        let mut pending_batch = BATCHES.load(&deps.storage, 1).unwrap();
        pending_batch.liquid_unstake_requests.insert(
            "bob".to_string(),
            LiquidUnstakeRequest::new(Addr::unchecked("bob"), Uint128::from(100u128)),
        );
        BATCHES.save(&mut deps.storage, 1, &pending_batch).unwrap();

        let info = mock_info("bob", &coins(1000, "factory/cosmos2contract/stTIA"));
        let msg = ExecuteMsg::LiquidUnstake {};

        let res = execute(deps.as_mut(), mock_env(), info.clone(), msg);
        assert!(res.is_ok());

        let pending_batch = BATCHES.load(&deps.storage, 1).unwrap();
        assert!(pending_batch.liquid_unstake_requests.len() == 1);
        assert_eq!(
            pending_batch
                .liquid_unstake_requests
                .get("bob")
                .unwrap()
                .shares
                , Uint128::from(1100u128)
        );
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
}
