#[cfg(test)]
mod staking_tests {
    use crate::contract::{execute, reply, IBC_TIMEOUT};
    use crate::error::ContractError;
    use crate::helpers::{derive_intermediate_sender, get_rates};
    use crate::msg::ExecuteMsg;
    use crate::state::{State, BATCHES, CONFIG, STATE};
    use crate::tests::test_helper::{init, CELESTIA1, CHANNEL_ID, NATIVE_TOKEN, OSMO3};
    use cosmwasm_std::testing::{message_info, mock_env, MOCK_CONTRACT_ADDR};
    use cosmwasm_std::{
        attr, coins, Addr, Binary, CosmosMsg, Decimal, IbcTimeout, Order, Reply, ReplyOn, SubMsg,
        SubMsgResponse, SubMsgResult, Timestamp, Uint128,
    };
    use milky_way::staking::BatchStatus;
    use osmosis_std::types::cosmos::base::v1beta1::Coin;
    use osmosis_std::types::ibc::applications::transfer::v1::MsgTransfer;
    use osmosis_std::types::osmosis::tokenfactory::v1beta1::MsgMint;
    use std::vec::Vec;

    #[test]
    fn proper_liquid_stake() {
        let mut deps = init();
        let env = mock_env();
        let info = message_info(&Addr::unchecked(OSMO3), &coins(1000, NATIVE_TOKEN));
        let msg = ExecuteMsg::LiquidStake {
            mint_to: None,
            expected_mint_amount: None,
        };
        let res = execute(deps.as_mut(), mock_env(), info, msg.clone());

        let timeout = IbcTimeout::with_timestamp(Timestamp::from_nanos(
            env.block.time.nanos() + IBC_TIMEOUT.nanos(),
        ));

        let ibc_coin = Coin {
            denom: NATIVE_TOKEN.to_string(),
            amount: "1000".to_string(),
        };

        let ibc_sub_msg_id = env.block.time.nanos() + env.transaction.unwrap().index as u64;
        match res {
            Ok(ref result) => {
                assert_eq!(
                    result.attributes,
                    vec![
                        attr("action", "liquid_stake"),
                        attr("sender", OSMO3),
                        attr("in_amount", "1000"),
                        attr("mint_amount", "1000"),
                    ]
                );
                assert_eq!(result.messages.len(), 3); // transfer, mint, redemption rate update
                assert_eq!(
                    result.messages[2],
                    SubMsg {
                        payload: Binary::new(vec![]),
                        id: ibc_sub_msg_id,
                        msg: <MsgTransfer as Into<CosmosMsg>>::into(MsgTransfer {
                            source_channel: CHANNEL_ID.to_string(),
                            source_port: "transfer".to_string(),
                            sender: env.contract.address.to_string(),
                            receiver: Addr::unchecked(CELESTIA1).to_string(),
                            token: Some(ibc_coin),
                            timeout_height: None,
                            timeout_timestamp: timeout.timestamp().unwrap().nanos(),
                            memo: format!("{{\"ibc_callback\":\"{}\"}}", env.contract.address),
                        }),
                        gas_limit: None,
                        reply_on: ReplyOn::Always,
                    }
                );
                assert_eq!(
                    result.messages[0],
                    SubMsg {
                        payload: Binary::new(vec![]),
                        id: 0,
                        msg: <MsgMint as Into<CosmosMsg>>::into(MsgMint {
                            sender: MOCK_CONTRACT_ADDR.to_string(),
                            amount: Some(Coin {
                                denom: format!(
                                    "factory/{}/stTIA",
                                    deps.api.addr_make("cosmos2contract").to_string()
                                ),
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

        // need to do this or we can't send more ibc messages
        // IBC_WAITING_FOR_REPLY.remove(deps.as_mut().storage);
        let _result = reply(
            deps.as_mut(),
            mock_env(),
            Reply {
                gas_used: 0,
                payload: Binary::new(vec![]),
                id: ibc_sub_msg_id,
                result: SubMsgResult::Ok(SubMsgResponse {
                    data: None,
                    events: Vec::new(),        // No events
                    msg_responses: Vec::new(), // No messages
                }),
            },
        );

        let pending_batch = BATCHES
            .range(deps.as_ref().storage, None, None, Order::Descending)
            .find(|r| r.is_ok() && r.as_ref().unwrap().1.status == BatchStatus::Pending)
            .unwrap()
            .unwrap()
            .1;
        assert!(pending_batch.id == 1);

        // Use the previously unwrapped value
        let state = STATE.load(deps.as_ref().storage).unwrap();
        assert_eq!(state.total_liquid_stake_token, Uint128::from(1000u128));
        assert_eq!(state.total_native_token, Uint128::from(1000u128));

        let info = message_info(&Addr::unchecked(OSMO3), &coins(10000, NATIVE_TOKEN));
        let res = execute(deps.as_mut(), mock_env(), info.clone(), msg.clone());

        assert!(res.is_ok());
        let state_for_osmo3 = STATE.load(&deps.storage).unwrap();
        assert_eq!(
            state_for_osmo3.total_liquid_stake_token,
            Uint128::from(11000u128)
        );
        assert_eq!(state_for_osmo3.total_native_token, Uint128::from(11000u128));

        // set total_liquid_stake_token: 1_000_000_000,
        // native_token: 1_000_000
        deps = init();
        let mut state = STATE.load(&deps.storage).unwrap();
        state.total_liquid_stake_token = Uint128::from(1_000_000_000u128);
        state.total_native_token = Uint128::from(1_000_000u128);
        STATE.save(&mut deps.storage, &state).unwrap();

        let info = message_info(&Addr::unchecked(OSMO3), &coins(50_000_000, NATIVE_TOKEN));
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

        let info = message_info(&Addr::unchecked(OSMO3), &coins(50_000_000, NATIVE_TOKEN));
        let res = execute(deps.as_mut(), mock_env(), info.clone(), msg);
        assert!(res.is_ok());

        let state = STATE.load(&deps.storage).unwrap();
        assert_eq!(state.total_liquid_stake_token, Uint128::from(1_050_000u128));
        assert_eq!(state.total_native_token, Uint128::from(1_050_000_000u128));

        // test redemption rate, purchase rate
        let (redemption_rate, purchase_rate) = get_rates(&deps.as_ref());
        assert_eq!(
            redemption_rate,
            Decimal::from_ratio(1_050_000_000u128, 1_050_000u128)
        );
        assert_eq!(
            purchase_rate,
            Decimal::from_ratio(1_050_000u128, 1_050_000_000u128)
        );
    }

    #[test]
    fn liquid_stake_less_than_minimum() {
        let mut deps = init();
        let info = message_info(&Addr::unchecked(OSMO3), &coins(10, NATIVE_TOKEN));
        let msg = ExecuteMsg::LiquidStake {
            mint_to: None,
            expected_mint_amount: None,
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
            derive_intermediate_sender(CHANNEL_ID, CELESTIA1, "osmo").unwrap();

        let info = message_info(
            &Addr::unchecked(&intermediate_sender),
            &coins(1000, NATIVE_TOKEN),
        );
        let msg: ExecuteMsg = ExecuteMsg::LiquidStake {
            mint_to: Some(OSMO3.to_string()),
            expected_mint_amount: None,
        };

        let res = execute(deps.as_mut(), mock_env(), info.clone(), msg);
        if res.is_err() {
            panic!("Unexpected error: {:?}", res);
        }
        assert!(res.is_ok());
    }

    #[test]
    fn receive_rewards_before_minting() {
        let mut deps = init();
        let env = mock_env();

        let config = CONFIG.load(&deps.storage).unwrap();

        // received rewards in advance of any liquid stake requests
        let sender = derive_intermediate_sender(
            &config.ibc_channel_id,
            config
                .multisig_address_config
                .reward_collector_address
                .as_ref(),
            "osmo",
        )
        .unwrap();
        let sender_addr = Addr::unchecked(&sender);
        let resp = execute(
            deps.as_mut(),
            env.clone(),
            message_info(&sender_addr, &coins(1_000, NATIVE_TOKEN)),
            ExecuteMsg::ReceiveRewards {},
        );

        assert!(resp.is_err());
    }
    #[test]
    fn mint_amount_divergence() {
        let mut deps = init();
        let mut state: State = STATE.load(&deps.storage).unwrap();
        state.total_liquid_stake_token = Uint128::from(1_000_000_000u128);
        state.total_native_token = Uint128::from(1_000_000u128);
        STATE.save(&mut deps.storage, &state).unwrap();

        let info = message_info(&Addr::unchecked(OSMO3), &coins(1000, NATIVE_TOKEN));
        let msg = ExecuteMsg::LiquidStake {
            mint_to: None,
            expected_mint_amount: Some(Uint128::from(2_000_000u128)),
        };
        let res: Result<cosmwasm_std::Response, ContractError> =
            execute(deps.as_mut(), mock_env(), info.clone(), msg.clone());
        assert!(res.is_err()); // minted amount is lower than expected

        let msg = ExecuteMsg::LiquidStake {
            mint_to: None,
            expected_mint_amount: Some(Uint128::from(1_000_000u128)),
        };
        let res = execute(deps.as_mut(), mock_env(), info.clone(), msg.clone());
        if res.is_err() {
            panic!("Unexpected error: {:?}", res);
        }
        assert!(res.is_ok());
    }

    #[test]
    fn zero_liquid_stake_but_native_tokens() {
        let mut deps = init();

        let mut state: State = STATE.load(&deps.storage).unwrap();
        state.total_native_token = Uint128::from(1000u128);
        state.total_liquid_stake_token = Uint128::from(0u128);
        state.total_fees = Uint128::from(100u128);
        STATE.save(&mut deps.storage, &state).unwrap();

        let info = message_info(&Addr::unchecked(OSMO3), &coins(1000, NATIVE_TOKEN));
        let msg = ExecuteMsg::LiquidStake {
            mint_to: None,
            expected_mint_amount: None,
        };
        let res = execute(deps.as_mut(), mock_env(), info, msg.clone());
        assert!(res.is_ok());

        let state: State = STATE.load(&deps.storage).unwrap();
        assert_eq!(state.total_native_token, Uint128::from(1000u128));
        assert_eq!(state.total_liquid_stake_token, Uint128::from(1000u128));
        assert_eq!(state.total_fees, Uint128::from(1100u128));
    }
}
