#[cfg(test)]
mod staking_tests {
    use crate::contract::execute;
    use crate::helpers::derive_intermediate_sender;
    use crate::msg::ExecuteMsg;
    use crate::state::{Config, BATCHES, CONFIG, STATE};
    use crate::tests::test_helper::init;
    use cosmwasm_std::testing::{mock_env, mock_info};
    use cosmwasm_std::{coins, Addr, Coin, Uint128};
    use milky_way::staking::{Batch, BatchStatus, LiquidUnstakeRequest};

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

        // Submit batch
        // currently disabled auto batch submit
        // assert_eq!(resp.messages.len(), 1);
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
                .shares,
            Uint128::from(1100u128)
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

    #[test]
    fn receive_unstaked_tokens() {
        let mut deps = init();
        let env = mock_env();

        let mut state = STATE.load(&deps.storage).unwrap();
        let config: Config = CONFIG.load(&deps.storage).unwrap();

        state.total_liquid_stake_token = Uint128::from(100_000u128);
        STATE.save(&mut deps.storage, &state).unwrap();

        let msg = ExecuteMsg::ReceiveUnstakedTokens {};

        let sender = derive_intermediate_sender(
            &config.ibc_channel_id,
            &config.multisig_address_config.staker_address.to_string(),
            "osmo",
        )
        .unwrap();

        let info = mock_info(
            &sender,
            &[Coin {
                amount: Uint128::from(100u128),
                denom: config.native_token_denom.clone(),
            }],
        );
        let res = execute(deps.as_mut(), env.clone(), info.clone(), msg.clone());
        assert!(res.is_err());

        let mut batch: Batch = BATCHES.load(&deps.storage, 1).unwrap();
        batch.update_status(BatchStatus::Submitted, Some(env.block.time.seconds() - 1));
        let res = BATCHES.save(&mut deps.storage, 1, &batch);
        assert!(res.is_ok());

        let res = execute(deps.as_mut(), env.clone(), info.clone(), msg.clone());
        assert!(res.is_ok());
    }
}
