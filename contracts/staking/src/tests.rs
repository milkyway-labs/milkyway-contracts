#[cfg(test)]
mod tests {
    use crate::contract::{execute, instantiate, IBC_TIMEOUT};
    use crate::helpers::derive_intermediate_sender;
    use crate::msg::{ExecuteMsg, InstantiateMsg};
    use crate::state::{
        IbcConfig, MultisigAddressConfig, ProtocolFeeConfig, CONFIG, IBC_CONFIG, PENDING_BATCH,
        STATE,
    };

    use cosmwasm_std::testing::{
        mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage,
    };
    use cosmwasm_std::{coins, Addr, Coin, OwnedDeps, Uint128};

    fn init() -> OwnedDeps<MockStorage, MockApi, MockQuerier> {
        let mut deps = mock_dependencies();
        let msg = InstantiateMsg {
            native_token_denom: "osmoTIA".to_string(),
            liquid_stake_token_denom: "stTIA".to_string(),
            treasury_address: "treasury".to_string(),
            node_operators: vec!["node1".to_string(), "node2".to_string()],
            validators: vec!["val1".to_string(), "val2".to_string()],
            batch_period: 86400,
            unbonding_period: 1209600,
            protocol_fee_config: ProtocolFeeConfig {
                dao_treasury_fee: Uint128::from(10u128),
            },
            multisig_address_config: MultisigAddressConfig {
                controller_address: Addr::unchecked("staker"),
                staker_address: Addr::unchecked("staker"),
                reward_collector_address: Addr::unchecked("reward_collector"),
            },
            minimum_liquid_stake_amount: Uint128::from(100u128),
            minimum_rewards_to_collect: Uint128::from(10u128),
            ibc_channel_id: "channel-123".to_string(),
        };
        let info = mock_info("creator", &coins(1000, "uosmo"));

        let _res = instantiate(deps.as_mut(), mock_env(), info, msg);

        let ibc_config = IbcConfig {
            channel_id: "channel-123".to_string(),
            default_timeout: IBC_TIMEOUT,
        };
        IBC_CONFIG.save(&mut deps.storage, &ibc_config).unwrap();

        deps
    }

    #[test]
    fn proper_instantiation() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg {
            native_token_denom: "osmoTIA".to_string(),
            liquid_stake_token_denom: "stTIA".to_string(),
            treasury_address: "treasury".to_string(),
            node_operators: vec!["node1".to_string(), "node2".to_string()],
            validators: vec!["val1".to_string(), "val2".to_string()],
            batch_period: 86400,
            unbonding_period: 1209600,
            protocol_fee_config: ProtocolFeeConfig {
                dao_treasury_fee: Uint128::from(10u128),
            },
            multisig_address_config: MultisigAddressConfig {
                controller_address: Addr::unchecked("staker"),
                staker_address: Addr::unchecked("staker"),
                reward_collector_address: Addr::unchecked("reward_collector"),
            },
            minimum_liquid_stake_amount: Uint128::from(100u128),
            minimum_rewards_to_collect: Uint128::from(10u128),
            ibc_channel_id: "channel-123".to_string(),
        };
        let info = mock_info("creator", &coins(1000, "uosmo"));

        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(1, res.messages.len());
        let attrs = res.attributes;
        assert_eq!(attrs[0].value, "instantiate");

        let batch = PENDING_BATCH.load(&deps.storage).unwrap();
        assert!(batch.id == 1);
    }
    #[test]
    fn proper_add_validator() {
        let mut deps = init();
        let info = mock_info("creator", &coins(1000, "uosmo"));
        let msg = ExecuteMsg::AddValidator {
            new_validator: "val3".to_string(),
        };

        let res = execute(deps.as_mut(), mock_env(), info, msg);
        assert!(res.is_ok());

        let attrs = res.unwrap().attributes;
        assert_eq!(attrs[0].value, "add_validator");
        assert_eq!(attrs[1].value, "val3");
    }

    #[test]
    fn duplicate_add_validator() {
        let mut deps = init();
        let info = mock_info("creator", &coins(1000, "uosmo"));
        let msg = ExecuteMsg::AddValidator {
            new_validator: "val1".to_string(),
        };

        let res = execute(deps.as_mut(), mock_env(), info, msg);
        assert!(res.is_err());
    }
    #[test]
    fn proper_remove_validator() {
        let mut deps = init();
        let info = mock_info("creator", &coins(1000, "uosmo"));
        let msg = ExecuteMsg::RemoveValidator {
            validator: "val1".to_string(),
        };

        let res = execute(deps.as_mut(), mock_env(), info, msg);
        assert!(res.is_ok());

        let attrs = res.unwrap().attributes;
        assert_eq!(attrs[0].value, "remove_validator");
        assert_eq!(attrs[1].value, "val1");
    }

    #[test]
    fn invalid_remove_validator() {
        let mut deps = init();
        let info = mock_info("creator", &coins(1000, "uosmo"));
        let msg = ExecuteMsg::RemoveValidator {
            validator: "val3".to_string(),
        };

        let res = execute(deps.as_mut(), mock_env(), info, msg);
        assert!(res.is_err());
    }
    #[test]
    fn non_admin_remove_validator() {
        let mut deps = init();
        let info = mock_info("bob", &coins(1000, "uosmo"));
        let msg = ExecuteMsg::RemoveValidator {
            validator: "val1".to_string(),
        };

        let res = execute(deps.as_mut(), mock_env(), info, msg);
        assert!(res.is_err());
    }
    #[test]
    fn non_admin_add_validator() {
        let mut deps = init();
        let info = mock_info("bob", &coins(1000, "uosmo"));
        let msg = ExecuteMsg::AddValidator {
            new_validator: "val3".to_string(),
        };

        let res = execute(deps.as_mut(), mock_env(), info, msg);
        assert!(res.is_err());
    }
    #[test]
    fn proper_transfer_ownership() {
        let mut deps = init();
        let info = mock_info("creator", &coins(1000, "uosmo"));
        let msg = ExecuteMsg::TransferOwnership {
            new_owner: "new_owner".to_string(),
        };

        let res = execute(deps.as_mut(), mock_env(), info, msg);
        assert!(res.is_ok());

        let attrs = res.unwrap().attributes;
        assert_eq!(attrs[0].value, "transfer_ownership");
        assert_eq!(attrs[1].value, "new_owner");
    }
    #[test]
    fn non_admin_transfer_ownership() {
        let mut deps = init();
        let info = mock_info("bob", &coins(1000, "uosmo"));
        let msg = ExecuteMsg::TransferOwnership {
            new_owner: "new_owner".to_string(),
        };

        let res = execute(deps.as_mut(), mock_env(), info, msg);
        assert!(res.is_err());
    }
    #[test]
    fn proper_claim_ownership() {
        let mut deps = init();
        let info = mock_info("creator", &coins(1000, "uosmo"));
        let msg = ExecuteMsg::TransferOwnership {
            new_owner: "new_owner".to_string(),
        };

        let res = execute(deps.as_mut(), mock_env(), info, msg);
        assert!(res.is_ok());

        let info = mock_info("new_owner", &coins(1000, "uosmo"));
        let msg = ExecuteMsg::AcceptOwnership {};

        let res2 = execute(deps.as_mut(), mock_env(), info, msg);

        let attrs = res2.unwrap().attributes;
        assert_eq!(attrs[0].value, "accept_ownership");
        assert_eq!(attrs[1].value, "new_owner");
    }
    #[test]
    fn unauthorized_claim_ownership() {
        let mut deps = init();
        let info = mock_info("creator", &coins(1000, "uosmo"));
        let msg = ExecuteMsg::TransferOwnership {
            new_owner: "new_owner".to_string(),
        };

        let res = execute(deps.as_mut(), mock_env(), info, msg);
        assert!(res.is_ok());

        let info = mock_info("bob", &coins(1000, "uosmo"));
        let msg = ExecuteMsg::AcceptOwnership {};

        let res2 = execute(deps.as_mut(), mock_env(), info, msg);

        assert!(res2.is_err());
    }
    #[test]
    fn proper_revoke_ownership_transfer() {
        let mut deps = init();
        let info = mock_info("creator", &coins(1000, "uosmo"));
        let msg = ExecuteMsg::TransferOwnership {
            new_owner: "new_owner".to_string(),
        };

        let res = execute(deps.as_mut(), mock_env(), info, msg);
        assert!(res.is_ok());

        let info = mock_info("creator", &coins(1000, "uosmo"));
        let msg = ExecuteMsg::RevokeOwnershipTransfer {};

        let res2 = execute(deps.as_mut(), mock_env(), info, msg);

        let attrs = res2.unwrap().attributes;
        assert_eq!(attrs[0].value, "revoke_ownership_transfer");
    }
    #[test]
    fn non_admin_revoke_ownership_transfer() {
        let mut deps = init();
        let info = mock_info("bob", &coins(1000, "uosmo"));
        let msg = ExecuteMsg::RevokeOwnershipTransfer {};

        let res2 = execute(deps.as_mut(), mock_env(), info, msg);

        assert!(res2.is_err());
    }
    #[test]
    fn proper_liquid_stake() {
        let mut deps = init();
        let info = mock_info("creator", &coins(1000, "osmoTIA"));
        let msg = ExecuteMsg::LiquidStake {};

        let res = execute(deps.as_mut(), mock_env(), info.clone(), msg);

        assert!(res.is_ok());

        // Unwrap once and store in a variable
        let unwrapped_res = res.unwrap();

        let attrs = &unwrapped_res.attributes;
        assert_eq!(attrs[0].value, "liquid_stake");

        let batch = PENDING_BATCH.load(&deps.storage).unwrap();
        assert!(batch.id == 1);

        // Use the previously unwrapped value
        assert_eq!(unwrapped_res.messages.len(), 2);
    }
    // // Create initial stake for bob
    //     fn prep_liquid_stake() -> OwnedDeps<MockStorage, MockApi, MockQuerier> {

    //         let mut deps = init();
    //         let info = mock_info("bob", &coins(1000, "osmoTIA"));
    //         let msg = ExecuteMsg::LiquidStake {};

    //         let res = execute(deps.as_mut(), mock_env(), info.clone(), msg);
    //         let batch = PENDING_BATCH.load(&deps.storage).unwrap();
    //         deps

    //     }

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
    // #[test]
    // fn double_liquid_unstake() {
    //     let mut deps = init();

    //     let mut state = STATE.load(&deps.storage).unwrap();

    //     state.total_liquid_stake_token = Uint128::from(100_000u128);
    //     STATE.save(&mut deps.storage, &state).unwrap();

    //     let mut pending_batch = PENDING_BATCH.load(&deps.storage).unwrap();
    //     pending_batch.liquid_unstake_requests.insert(
    //         Addr::unchecked("bob"),
    //         LiquidUnstakeRequest {
    //             user: Addr::unchecked("bob"),
    //             shares: Uint128::from(100u128),
    //         },
    //     );

    //     // PENDING_BATCH.save(&mut deps.storage, &pending_batch).unwrap();

    //     let info = mock_info("bob", &coins(1000, "factory/cosmos2contract/stTIA"));
    //     let msg = ExecuteMsg::LiquidUnstake {};

    //     let res = execute(deps.as_mut(), mock_env(), info.clone(), msg);

    //     let resp = res.unwrap();

    //     let attrs = resp.attributes;
    //     assert_eq!(attrs[0].value, "liquid_unstake");

    //     // Submit batch
    //     assert_eq!(resp.messages.len(), 1);

    // }
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
    fn empty_submit_batch() {
        let mut deps = init();
        let mut env = mock_env();

        let state = STATE.load(&deps.storage).unwrap();
        let config = CONFIG.load(&deps.storage).unwrap();

        STATE.save(&mut deps.storage, &state).unwrap();

        // print!("{:?}", msgs);
        env.block.time = env.block.time.plus_seconds(config.batch_period + 1);
        let msg = ExecuteMsg::SubmitBatch { batch_id: 1 };

        let contract = env.contract.address.clone().to_string();

        let info = mock_info(&contract, &[]);
        let res = execute(deps.as_mut(), env, info, msg);
        print!("{:?}", res);
        assert!(res.is_err());
    }
    #[test]
    fn not_ready_submit_batch() {
        let mut deps = init();
        let mut env = mock_env();

        let mut state = STATE.load(&deps.storage).unwrap();
        let config = CONFIG.load(&deps.storage).unwrap();

        state.total_liquid_stake_token = Uint128::from(100_000u128);
        STATE.save(&mut deps.storage, &state).unwrap();

        // batch isnt ready
        env.block.time = env.block.time.plus_seconds(config.batch_period - 1);
        let msg = ExecuteMsg::SubmitBatch { batch_id: 1 };

        let contract = env.contract.address.clone().to_string();

        let info = mock_info(&contract, &[]);
        let res = execute(deps.as_mut(), env, info, msg);

        assert!(res.is_err());
    }
    #[test]
    fn receive_rewards() {
        let mut deps = init();
        let mut env = mock_env();

        let mut state = STATE.load(&deps.storage).unwrap();
        let config = CONFIG.load(&deps.storage).unwrap();

        state.total_liquid_stake_token = Uint128::from(100_000u128);
        STATE.save(&mut deps.storage, &state).unwrap();

        let msg = ExecuteMsg::ReceiveRewards {};

        let contract = env.contract.address.clone().to_string();

        let sender = derive_intermediate_sender(
            &config.ibc_channel_id,
            &config
                .multisig_address_config
                .reward_collector_address
                .to_string(),
            "osmo",
        )
        .unwrap();

        let info = mock_info(
            &sender,
            &[Coin {
                amount: Uint128::from(100u128),
                denom: "uosmo".to_string(),
            }],
        );
        let res = execute(deps.as_mut(), env.clone(), info, msg.clone());

        assert!(res.is_err()); // wrong denom

        let info = mock_info(
            &contract,
            &[Coin {
                amount: Uint128::from(100u128),
                denom: config.native_token_denom.clone(),
            }],
        );
        let res = execute(deps.as_mut(), env.clone(), info, msg.clone());

        assert!(res.is_err()); // wrong sender

        let info = mock_info(
            &sender,
            &[Coin {
                amount: Uint128::from(100u128),
                denom: config.native_token_denom.clone(),
            }],
        );
        let res = execute(deps.as_mut(), env.clone(), info, msg.clone());

        assert!(res.is_ok());
        assert!(res.unwrap().messages.len() == 1); // transfer messages as sub message
    }

    #[test]
    fn circuit_breaker() {
        let mut deps = init();
        let mut env = mock_env();

        let mut state = STATE.load(&deps.storage).unwrap();
        let config = CONFIG.load(&deps.storage).unwrap();

        state.total_liquid_stake_token = Uint128::from(100_000u128);
        STATE.save(&mut deps.storage, &state).unwrap();

        let msg = ExecuteMsg::CircuiteBreaker {};

        let contract = env.contract.address.clone().to_string();

        // not correct sender
        let info = mock_info(&contract, &[]);
        let res = execute(deps.as_mut(), env.clone(), info, msg.clone());

        assert!(res.is_err());

        // correct sender
        let info = mock_info(&"node1".to_string(), &[]);
        let res = execute(deps.as_mut(), env.clone(), info, msg.clone());

        assert!(res.is_ok());

        // liquid stake
        let info = mock_info("creator", &coins(1000, "osmoTIA"));
        let msg = ExecuteMsg::LiquidStake {};
        let res = execute(deps.as_mut(), mock_env(), info.clone(), msg);
        assert!(res.is_err());

        // liquid unstake
        // let info = mock_info("bob", &coins(1000, "factory/cosmos2contract/stTIA"));
        // let msg = ExecuteMsg::LiquidUnstake {};
        // let res = execute(deps.as_mut(), mock_env(), info.clone(), msg);
        // assert!(res.is_err());

        // receive rewards
        let msg = ExecuteMsg::ReceiveRewards {};
        let sender = derive_intermediate_sender(
            &config.ibc_channel_id,
            &config
                .multisig_address_config
                .reward_collector_address
                .to_string(),
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
        let res = execute(deps.as_mut(), env.clone(), info, msg.clone());
        assert!(res.is_err());

        // receive unstaked tokens
        let msg = ExecuteMsg::ReceiveUnstakedTokens {};
        let info = mock_info(
            &sender,
            &[Coin {
                amount: Uint128::from(100u128),
                denom: config.native_token_denom.clone(),
            }],
        );
        let res = execute(deps.as_mut(), env.clone(), info, msg.clone());
        assert!(res.is_err());

        // execute withdraw
        // let msg = ExecuteMsg::Withdraw { batch_id: 1 };
        // let info = mock_info("bob", &[]);
        // let res = execute(deps.as_mut(), env.clone(), info, msg.clone());
        // assert!(res.is_err());

        // submit batch
        env.block.time = env.block.time.plus_seconds(config.batch_period - 1);
        let msg = ExecuteMsg::SubmitBatch { batch_id: 1 };
        let info = mock_info(&contract, &[]);
        let res = execute(deps.as_mut(), env.clone(), info, msg.clone());
        assert!(res.is_err());

        // reenable
        let msg = ExecuteMsg::ResumeContract {};

        // not correct sender
        let info = mock_info(&contract, &[]);
        let res = execute(deps.as_mut(), env.clone(), info, msg.clone());

        assert!(res.is_err());

        // correct sender
        let info = mock_info("creator", &[]);
        let res = execute(deps.as_mut(), env.clone(), info, msg.clone());

        assert!(res.is_ok());

        // test enabled
        let info = mock_info("creator", &coins(1000, "osmoTIA"));
        let msg = ExecuteMsg::LiquidStake {};
        let res = execute(deps.as_mut(), mock_env(), info.clone(), msg);
        assert!(res.is_ok());
    }
}
