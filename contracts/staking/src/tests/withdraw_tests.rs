use crate::contract::{execute, query};
use crate::msg::{ExecuteMsg, QueryMsg};
use crate::state::{new_unstake_request, UnstakeRequest, BATCHES, CONFIG, STATE};
use crate::tests::test_helper::{init, NATIVE_TOKEN, OSMO1, OSMO3};
use cosmwasm_std::testing::{mock_env, mock_info, MOCK_CONTRACT_ADDR};
use cosmwasm_std::{from_json, Addr, CosmosMsg, ReplyOn, SubMsg, Uint128};
use milky_way::staking::Batch;
use osmosis_std::types::cosmos::bank::v1beta1::MsgSend;
use osmosis_std::types::cosmos::base::v1beta1::Coin;

#[test]
fn withdraw() {
    let mut deps = init();
    let env = mock_env();
    let mut state = STATE.load(&deps.storage).unwrap();

    state.total_liquid_stake_token = Uint128::from(130_000u128);
    state.total_native_token = Uint128::from(300_000u128);
    STATE.save(&mut deps.storage, &state).unwrap();

    let mut pending_batch: Batch =
        Batch::new(1, Uint128::new(130_000), env.block.time.seconds() + 10_000);
    new_unstake_request(
        &mut deps.as_mut(),
        "bob".to_string(),
        1,
        Uint128::from(40_000u128),
    )
    .unwrap();
    new_unstake_request(
        &mut deps.as_mut(),
        "tom".to_string(),
        1,
        Uint128::from(90_000u128),
    )
    .unwrap();
    let res = BATCHES.save(&mut deps.storage, 1, &pending_batch);
    assert!(res.is_ok());

    // batch not ready
    let msg = ExecuteMsg::Withdraw { batch_id: 1 };
    let info = mock_info("bob", &[]);
    let res = execute(deps.as_mut(), env.clone(), info, msg.clone());
    assert!(res.is_err());

    // batch ready
    pending_batch.received_native_unstaked = Some(Uint128::new(130_000));
    pending_batch.status = milky_way::staking::BatchStatus::Received;
    let res = BATCHES.save(&mut deps.storage, 1, &pending_batch);
    assert!(res.is_ok());

    // no request in batch
    let msg = ExecuteMsg::Withdraw { batch_id: 2 };
    let info = mock_info("bob", &[]);
    let res = execute(deps.as_mut(), env.clone(), info, msg.clone());
    assert!(res.is_err());

    let msg = ExecuteMsg::Withdraw { batch_id: 1 };
    let info = mock_info("alice", &[]);
    let res = execute(deps.as_mut(), env.clone(), info, msg.clone());
    assert!(res.is_err());

    // success
    let msg = ExecuteMsg::Withdraw {
        batch_id: pending_batch.id,
    };
    let info = mock_info("bob", &[]);
    let res = execute(deps.as_mut(), env.clone(), info, msg.clone());
    assert!(res.is_ok());
    let messages = res.unwrap().messages;
    assert_eq!(messages.len(), 2); // withdraw and redemption/purchase rate update

    let msg = QueryMsg::UnstakeRequests {
        user: Addr::unchecked("bob"),
    };
    let res = query(deps.as_ref(), env.clone(), msg);
    assert!(res.is_ok());
    let resp: Vec<UnstakeRequest> = from_json(res.unwrap()).unwrap();

    assert!(resp.is_empty());

    let config = CONFIG.load(&deps.storage).unwrap();
    let coin = Coin {
        denom: config.protocol_chain_config.ibc_token_denom.clone(),
        amount: "40000".to_string(),
    };

    // check the MsgSend
    let coins = vec![coin];
    assert_eq!(
        messages[0],
        SubMsg {
            id: 0,
            msg: <MsgSend as Into<CosmosMsg>>::into(MsgSend {
                from_address: Addr::unchecked(MOCK_CONTRACT_ADDR).to_string(),
                to_address: "bob".to_string(),
                amount: coins,
            }),
            gas_limit: None,
            reply_on: ReplyOn::Never,
        }
    );

    // Tom withdraw
    let msg = ExecuteMsg::Withdraw {
        batch_id: pending_batch.id,
    };
    let info = mock_info("tom", &[]);
    let res = execute(deps.as_mut(), env.clone(), info, msg.clone());
    assert!(res.is_ok());
    let messages = res.unwrap().messages;
    assert_eq!(messages.len(), 2); // withdraw and redemption/purchase rate update

    let msg = QueryMsg::UnstakeRequests {
        user: Addr::unchecked("tom"),
    };
    let res = query(deps.as_ref(), env.clone(), msg);
    assert!(res.is_ok());
    let resp: Vec<UnstakeRequest> = from_json(res.unwrap()).unwrap();

    assert!(resp.is_empty());

    let config = CONFIG.load(&deps.storage).unwrap();
    let coin = Coin {
        denom: config.protocol_chain_config.ibc_token_denom.clone(),
        amount: "90000".to_string(),
    };

    // check the MsgSend
    let coins = vec![coin];
    assert_eq!(
        messages[0],
        SubMsg {
            id: 0,
            msg: <MsgSend as Into<CosmosMsg>>::into(MsgSend {
                from_address: Addr::unchecked(MOCK_CONTRACT_ADDR).to_string(),
                to_address: "tom".to_string(),
                amount: coins,
            }),
            gas_limit: None,
            reply_on: ReplyOn::Never,
        }
    );
}

#[test]
fn withdraw_slashing() {
    let mut deps = init();
    let env = mock_env();
    let mut state = STATE.load(&deps.storage).unwrap();

    state.total_liquid_stake_token = Uint128::from(130_000u128);
    state.total_native_token = Uint128::from(300_000u128);
    STATE.save(&mut deps.storage, &state).unwrap();

    let mut pending_batch: Batch =
        Batch::new(1, Uint128::new(130_000), env.block.time.seconds() + 10_000);
    new_unstake_request(
        &mut deps.as_mut(),
        "bob".to_string(),
        1,
        Uint128::from(40_000u128),
    )
    .unwrap();
    new_unstake_request(
        &mut deps.as_mut(),
        "tom".to_string(),
        1,
        Uint128::from(90_000u128),
    )
    .unwrap();
    let res = BATCHES.save(&mut deps.storage, 1, &pending_batch);
    assert!(res.is_ok());

    // batch ready
    pending_batch.received_native_unstaked = Some(Uint128::new(990_000)); // slashing happened
    pending_batch.status = milky_way::staking::BatchStatus::Received;
    let res = BATCHES.save(&mut deps.storage, 1, &pending_batch);
    assert!(res.is_ok());

    // success
    let msg = ExecuteMsg::Withdraw {
        batch_id: pending_batch.id,
    };
    let info = mock_info("bob", &[]);
    let res = execute(deps.as_mut(), env.clone(), info, msg.clone());
    assert!(res.is_ok());
    let messages = res.unwrap().messages;
    assert_eq!(messages.len(), 2); // withdraw and redemption rate update

    let msg = QueryMsg::UnstakeRequests {
        user: Addr::unchecked("bob"),
    };
    let res = query(deps.as_ref(), env.clone(), msg);
    assert!(res.is_ok());
    let resp: Vec<UnstakeRequest> = from_json(res.unwrap()).unwrap();

    assert!(resp.is_empty());

    let config = CONFIG.load(&deps.storage).unwrap();
    let coin = Coin {
        denom: config.protocol_chain_config.ibc_token_denom.clone(),
        amount: "304615".to_string(), //304615.384... = 304615
    };

    // check the MsgSend
    let coins = vec![coin];
    assert_eq!(
        messages[0],
        SubMsg {
            id: 0,
            msg: <MsgSend as Into<CosmosMsg>>::into(MsgSend {
                from_address: Addr::unchecked(MOCK_CONTRACT_ADDR).to_string(),
                to_address: "bob".to_string(),
                amount: coins,
            }),
            gas_limit: None,
            reply_on: ReplyOn::Never,
        }
    );

    // Tom withdraw
    let msg = ExecuteMsg::Withdraw {
        batch_id: pending_batch.id,
    };
    let info = mock_info("tom", &[]);
    let res = execute(deps.as_mut(), env.clone(), info, msg.clone());
    assert!(res.is_ok());
    let messages = res.unwrap().messages;
    assert_eq!(messages.len(), 2); // withdraw and redemption/purchase rate update

    let msg = QueryMsg::UnstakeRequests {
        user: Addr::unchecked("tom"),
    };
    let res = query(deps.as_ref(), env.clone(), msg);
    assert!(res.is_ok());
    let resp: Vec<UnstakeRequest> = from_json(res.unwrap()).unwrap();

    assert!(resp.is_empty());

    let config = CONFIG.load(&deps.storage).unwrap();
    let coin = Coin {
        denom: config.protocol_chain_config.ibc_token_denom.clone(),
        amount: "685384".to_string(), //685,384.615... = 685384
    };

    // check the MsgSend
    let coins = vec![coin];
    assert_eq!(
        messages[0],
        SubMsg {
            id: 0,
            msg: <MsgSend as Into<CosmosMsg>>::into(MsgSend {
                from_address: Addr::unchecked(MOCK_CONTRACT_ADDR).to_string(),
                to_address: "tom".to_string(),
                amount: coins,
            }),
            gas_limit: None,
            reply_on: ReplyOn::Never,
        }
    );
}

#[test]
fn fee_withdraw() {
    let mut deps = init();
    let env = mock_env();
    let mut state = STATE.load(&deps.storage).unwrap();
    state.total_fees = Uint128::from(1000u128);
    STATE.save(&mut deps.storage, &state).unwrap();

    let msg = ExecuteMsg::FeeWithdraw {
        amount: Uint128::from(2000u128),
    };
    let info = mock_info("bob", &[]);
    let res = execute(deps.as_mut(), env.clone(), info, msg.clone());
    assert!(res.is_err()); // because not admin

    let info = mock_info(OSMO3, &[]);
    let res = execute(deps.as_mut(), env.clone(), info, msg.clone());
    assert!(res.is_err()); // because too high amount

    let msg = ExecuteMsg::FeeWithdraw {
        amount: Uint128::from(1000u128),
    };
    let info = mock_info(OSMO3, &[]);
    let res = execute(deps.as_mut(), env.clone(), info, msg.clone());
    assert!(res.is_ok());
    assert_eq!(
        res.unwrap().messages[0],
        SubMsg {
            id: 0,
            msg: <MsgSend as Into<CosmosMsg>>::into(MsgSend {
                from_address: Addr::unchecked(MOCK_CONTRACT_ADDR).to_string(),
                to_address: OSMO1.to_string(),
                amount: vec![Coin {
                    denom: NATIVE_TOKEN.to_string(),
                    amount: "1000".to_string()
                }],
            }),
            gas_limit: None,
            reply_on: ReplyOn::Never,
        }
    );
}
