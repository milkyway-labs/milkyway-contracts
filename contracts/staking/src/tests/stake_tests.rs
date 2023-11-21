#[cfg(test)]
mod staking_tests {
    use crate::contract::{execute, IBC_TIMEOUT};
    use crate::error::ContractError;
    use crate::helpers::derive_intermediate_sender;
    use crate::msg::ExecuteMsg;
    use crate::state::{BATCHES, STATE};
    use crate::tests::test_helper::{init, CELESTIA1, CHANNEL_ID, NATIVE_TOKEN, OSMO3};
    use cosmwasm_std::testing::{mock_env, mock_info, MOCK_CONTRACT_ADDR};
    use cosmwasm_std::{
        attr, coins, Addr, CosmosMsg, IbcMsg, IbcTimeout, Order, ReplyOn, SubMsg, Timestamp,
        Uint128,
    };
    use milky_way::staking::BatchStatus;
    use osmosis_std::types::cosmos::base::v1beta1::Coin;
    use osmosis_std::types::osmosis::tokenfactory::v1beta1::MsgMint;

    #[test]
    fn proper_liquid_stake() {
        let mut deps = init();
        let env = mock_env();
        let info = mock_info(OSMO3, &coins(1000, NATIVE_TOKEN));
        let msg = ExecuteMsg::LiquidStake {
            original_sender: None,
        };
        let res = execute(deps.as_mut(), mock_env(), info, msg.clone());

        let timeout = IbcTimeout::with_timestamp(Timestamp::from_nanos(
            env.block.time.nanos() + IBC_TIMEOUT.nanos(),
        ));

        let ibc_coin = cosmwasm_std::Coin {
            denom: NATIVE_TOKEN.to_string(),
            amount: Uint128::from(1000u128),
        };

        match res {
            Ok(ref result) => {
                assert_eq!(
                    result.attributes,
                    vec![
                        attr("action", "liquid_stake"),
                        attr("sender", OSMO3),
                        attr("amount", "1000")
                    ]
                );
                assert_eq!(result.messages.len(), 2);
                assert_eq!(
                    result.messages[1],
                    SubMsg {
                        id: 0,
                        msg: <cosmwasm_std::IbcMsg as Into<CosmosMsg>>::into(IbcMsg::Transfer {
                            channel_id: CHANNEL_ID.to_string(),
                            to_address: Addr::unchecked(CELESTIA1).to_string(),
                            amount: ibc_coin,
                            timeout,
                        }),
                        gas_limit: None,
                        reply_on: ReplyOn::Never,
                    }
                );
                assert_eq!(
                    result.messages[0],
                    SubMsg {
                        id: 0,
                        msg: <MsgMint as Into<CosmosMsg>>::into(MsgMint {
                            sender: Addr::unchecked(MOCK_CONTRACT_ADDR).to_string(),
                            amount: Some(Coin {
                                denom: "factory/cosmos2contract/stTIA".to_string(),
                                amount: "1000".to_string(),
                            }),
                            mint_to_address: OSMO3.to_string(),
                        }),
                        gas_limit: None,
                        reply_on: ReplyOn::Never,
                    }
                );
            }
            Err(e) => {
                panic!("Unexpected error: {:?}", e);
            }
        }

        let pending_batch = BATCHES
            .range(&deps.storage, None, None, Order::Descending)
            .find(|r| r.is_ok() && r.as_ref().unwrap().1.status == BatchStatus::Pending)
            .unwrap()
            .unwrap()
            .1;
        assert!(pending_batch.id == 1);

        // Use the previously unwrapped value
        let state = STATE.load(&deps.storage).unwrap();
        assert_eq!(state.total_liquid_stake_token, Uint128::from(1000u128));
        assert_eq!(state.total_native_token, Uint128::from(1000u128));

        let info = mock_info("bob", &coins(10000, NATIVE_TOKEN));
        let res = execute(deps.as_mut(), mock_env(), info.clone(), msg.clone());
        assert!(res.is_ok());
        let state_for_bob = STATE.load(&deps.storage).unwrap();
        assert_eq!(
            state_for_bob.total_liquid_stake_token,
            Uint128::from(11000u128)
        );
        assert_eq!(state_for_bob.total_native_token, Uint128::from(11000u128));

        // set total_liquid_stake_token: 1_000_000_000,
        // native_token: 1_000_000
        deps = init();
        let mut state = STATE.load(&deps.storage).unwrap();
        state.total_liquid_stake_token = Uint128::from(1_000_000_000u128);
        state.total_native_token = Uint128::from(1_000_000u128);
        STATE.save(&mut deps.storage, &state).unwrap();

        let info = mock_info("bob", &coins(50_000_000, NATIVE_TOKEN));
        let res = execute(deps.as_mut(), mock_env(), info.clone(), msg.clone());
        assert!(res.is_ok());

        let state = STATE.load(&deps.storage).unwrap();
        assert_eq!(
            state.total_liquid_stake_token,
            Uint128::from(51_000_000_000u128)
        );
        assert_eq!(state.total_native_token, Uint128::from(51_000_000u128));

        // set total_liquid_stake_token: 1_000_000,
        // native_token: 1_000_000_000
        deps = init();
        let mut state = STATE.load(&deps.storage).unwrap();
        state.total_liquid_stake_token = Uint128::from(1_000_000u128);
        state.total_native_token = Uint128::from(1_000_000_000u128);
        STATE.save(&mut deps.storage, &state).unwrap();

        let info = mock_info("bob", &coins(50_000_000, NATIVE_TOKEN));
        let res = execute(deps.as_mut(), mock_env(), info.clone(), msg);
        assert!(res.is_ok());

        let state = STATE.load(&deps.storage).unwrap();
        assert_eq!(state.total_liquid_stake_token, Uint128::from(1_050_000u128));
        assert_eq!(state.total_native_token, Uint128::from(1_050_000_000u128));
    }

    #[test]
    fn liquid_stake_less_than_minimum() {
        let mut deps = init();
        let info = mock_info(OSMO3, &coins(10, NATIVE_TOKEN));
        let msg = ExecuteMsg::LiquidStake {
            original_sender: None,
        };

        let res = execute(deps.as_mut(), mock_env(), info.clone(), msg);
        match res {
            Ok(_) => panic!("Expected error"),
            Err(e) => {
                if let ContractError::MinimumLiquidStakeAmount {
                    minimum_stake_amount,
                    sent_amount,
                } = e
                {
                    assert_eq!(minimum_stake_amount, Uint128::from(100u128));
                    assert_eq!(sent_amount, Uint128::from(10u128));
                } else {
                    panic!("Unexpected error: {:?}", e);
                }
            }
        }
    }

    #[test]
    fn proper_ibc_liquid_stake() {
        let mut deps = init();
        let intermediate_sender =
            derive_intermediate_sender(CHANNEL_ID, &CELESTIA1.to_string(), "osmo").unwrap();
        let info = mock_info(&intermediate_sender, &coins(1000, NATIVE_TOKEN));
        let msg = ExecuteMsg::LiquidStake {
            original_sender: Some(CELESTIA1.to_string()),
        };

        let res = execute(deps.as_mut(), mock_env(), info.clone(), msg);
        assert!(res.is_ok());

        let info = mock_info(&intermediate_sender, &coins(1000, NATIVE_TOKEN));
        let msg = ExecuteMsg::LiquidStake {
            original_sender: Some(OSMO3.to_string()),
        };

        let res = execute(deps.as_mut(), mock_env(), info.clone(), msg);
        assert!(res.is_ok());
    }
}
