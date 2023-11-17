#[cfg(test)]
mod query_tests {
    use crate::contract::{query, execute};
    use crate::msg::{QueryMsg, ExecuteMsg, ConfigResponse, StateResponse};
    use crate::tests::test_helper::{init, OSMO1, OSMO2, OSMO3, CELESTIA1, CELESTIA2, NATIVE_TOKEN};
    use cosmwasm_std::testing::{mock_env, mock_info};
    use cosmwasm_std::{from_binary, coins, Uint128, Decimal};

    #[test]
    fn get_config() {
        let mut deps = init();
        let msg = QueryMsg::Config {};
        let bin = query(deps.as_mut(), mock_env(), msg.clone()).unwrap();
        let result = from_binary::<ConfigResponse>(&bin);
        println!("result {:?}", result);
        match result {
            Ok(res) => {
                assert_eq!(res.native_token_denom, "osmoTIA".to_string());
                assert_eq!(res.liquid_stake_token_denom, "factory/cosmos2contract/stTIA".to_string());
                assert_eq!(res.treasury_address, OSMO1.to_string());
                assert_eq!(res.operators, vec![OSMO2.to_string(), OSMO3.to_string()]);
                assert_eq!(res.validators, vec![CELESTIA1.to_string(), CELESTIA2.to_string()]);
                assert_eq!(res.batch_period, 86400);
                assert_eq!(res.unbonding_period, 1209600);
                assert_eq!(res.minimum_liquid_stake_amount, Uint128::from(100u128));
            },
            Err(e) => match e {
                _ => panic!("Unexpected error: {:?}", e),
            },
        }

    }

    #[test]
    fn get_state() {
        let mut deps = init();
        let msg = QueryMsg::State {};
        let mut bin = query(deps.as_mut(), mock_env(), msg.clone()).unwrap();
        let mut result = from_binary::<StateResponse>(&bin);

        println!("result {:?}", result);
        match result {
            Ok(res) => {
                assert_eq!(res.total_native_token, Uint128::from(0u128));
                assert_eq!(res.total_liquid_stake_token, Uint128::from(0u128));
                assert_eq!(res.rate, Decimal::zero());
                assert_eq!(res.pending_owner, "".to_string());
                assert_eq!(res.total_reward_amount, Uint128::from(0u128));
            },
            Err(e) => match e {
                _ => panic!("Unexpected error: {:?}", e),
            },
        }

        // stake
        let info = mock_info("creator", &coins(1000, NATIVE_TOKEN));
        let stake_msg = ExecuteMsg::LiquidStake {};
        let res = execute(deps.as_mut(), mock_env(), info, stake_msg);
        assert!(res.is_ok());

        // check the state
        bin = query(deps.as_mut(), mock_env(), msg.clone()).unwrap();
        result = from_binary::<StateResponse>(&bin);

        println!("result {:?}", result);
        match result {
            Ok(res) => {
                assert_eq!(res.total_native_token, Uint128::from(1000u128));
                assert_eq!(res.total_liquid_stake_token, Uint128::from(1000u128));
                assert_eq!(res.rate, Decimal::from_ratio(res.total_liquid_stake_token, res.total_native_token));
            },
            Err(e) => match e {
                _ => panic!("Unexpected error: {:?}", e),
            },
        }
    }
}