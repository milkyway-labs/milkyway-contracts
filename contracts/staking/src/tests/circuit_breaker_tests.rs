use crate::contract::execute;
use crate::helpers::derive_intermediate_sender;
use crate::msg::ExecuteMsg;
use crate::state::{new_unstake_request, State, BATCHES, CONFIG, STATE};
use crate::tests::test_helper::{init, NATIVE_TOKEN, OSMO2, OSMO3};
use cosmwasm_std::testing::{mock_env, mock_info};
use cosmwasm_std::{coins, Coin, Uint128};
use milky_way::staking::Batch;

#[test]
fn circuit_breaker() {
    let mut deps = init();
    let mut env = mock_env();

    let mut state = STATE.load(&deps.storage).unwrap();
    let config = CONFIG.load(&deps.storage).unwrap();

    state.total_liquid_stake_token = Uint128::from(100_000u128);
    state.total_native_token = Uint128::from(300_000u128);
    STATE.save(&mut deps.storage, &state).unwrap();

    let msg = ExecuteMsg::CircuitBreaker {};

    let contract = env.contract.address.clone().to_string();

    // not correct sender
    let info = mock_info(&contract, &[]);
    let res = execute(deps.as_mut(), env.clone(), info, msg.clone());

    assert!(res.is_err());

    // correct sender (admin)
    let info = mock_info(OSMO3, &[]);
    let res = execute(deps.as_mut(), env.clone(), info, msg.clone());

    assert!(res.is_ok());

    // correct sender (operator)
    let info = mock_info(OSMO2, &[]);
    let res = execute(deps.as_mut(), env.clone(), info, msg.clone());

    assert!(res.is_ok());

    // liquid stake
    let info = mock_info(OSMO3, &coins(1000, "osmoTIA"));
    let msg = ExecuteMsg::LiquidStake {
        mint_to: None,
        transfer_to_native_chain: None,
        expected_mint_amount: None,
    };
    let res = execute(deps.as_mut(), mock_env(), info.clone(), msg);
    assert!(res.is_err());

    // liquid unstake
    state.total_liquid_stake_token = Uint128::from(100_000u128);
    state.total_native_token = Uint128::from(300_000u128);
    STATE.save(&mut deps.storage, &state).unwrap();
    let info = mock_info("bob", &coins(1000, "factory/cosmos2contract/stTIA"));
    let msg = ExecuteMsg::LiquidUnstake {};
    let res = execute(deps.as_mut(), mock_env(), info.clone(), msg);
    assert!(res.is_err());

    // receive rewards
    let msg = ExecuteMsg::ReceiveRewards {};
    let sender = derive_intermediate_sender(
        &config.protocol_chain_config.ibc_channel_id,
        config.native_chain_config.reward_collector_address.as_ref(),
        &config.protocol_chain_config.account_address_prefix,
    )
    .unwrap();
    let info = mock_info(
        &sender,
        &[Coin {
            amount: Uint128::from(100u128),
            denom: config.protocol_chain_config.ibc_token_denom.clone(),
        }],
    );
    let res = execute(deps.as_mut(), env.clone(), info, msg.clone());
    assert!(res.is_err());

    // receive unstaked tokens
    let msg = ExecuteMsg::ReceiveUnstakedTokens { batch_id: 1 };
    let info = mock_info(
        &sender,
        &[Coin {
            amount: Uint128::from(100u128),
            denom: config.protocol_chain_config.ibc_token_denom.clone(),
        }],
    );
    let res = execute(deps.as_mut(), env.clone(), info, msg.clone());
    assert!(res.is_err());

    // execute withdraw
    let mut pending_batch: Batch = Batch::new(1, Uint128::zero(), env.block.time.seconds() + 10000);
    new_unstake_request(
        &mut deps.as_mut(),
        "bob".to_string(),
        1,
        Uint128::from(10u128),
    )
    .unwrap();
    pending_batch.status = milky_way::staking::BatchStatus::Received;
    let _res = BATCHES.save(&mut deps.storage, 1, &pending_batch);
    let msg = ExecuteMsg::Withdraw { batch_id: 1 };
    let info = mock_info("bob", &[]);
    let res = execute(deps.as_mut(), env.clone(), info, msg.clone());
    assert!(res.is_err());

    // submit batch
    env.block.time = env.block.time.plus_seconds(config.batch_period - 1);
    let msg = ExecuteMsg::SubmitBatch {};
    let info = mock_info(&contract, &[]);
    let res = execute(deps.as_mut(), env.clone(), info, msg.clone());
    assert!(res.is_err());

    // reenable
    let msg = ExecuteMsg::ResumeContract {
        total_native_token: Uint128::from(100000u128),
        total_liquid_stake_token: Uint128::from(200000u128),
        total_reward_amount: Uint128::from(10000u128),
    };

    // not correct sender
    let info = mock_info(&contract, &[]);
    let res = execute(deps.as_mut(), env.clone(), info, msg.clone());

    assert!(res.is_err());

    // correct sender
    let info = mock_info(OSMO3, &[]);
    let res = execute(deps.as_mut(), env.clone(), info, msg.clone());

    assert!(res.is_ok());

    // test accounting update
    let state: State = STATE.load(&deps.storage).unwrap();
    assert_eq!(state.total_liquid_stake_token, Uint128::from(200000u128));
    assert_eq!(state.total_native_token, Uint128::from(100000u128));
    assert_eq!(state.total_reward_amount, Uint128::from(10000u128));

    // test can't resume contract
    let info = mock_info(OSMO3, &[]);
    let res = execute(deps.as_mut(), env.clone(), info, msg.clone());
    assert!(res.is_err());

    // test enabled
    let info = mock_info(OSMO3, &coins(1000, NATIVE_TOKEN));
    let msg = ExecuteMsg::LiquidStake {
        expected_mint_amount: None,
        transfer_to_native_chain: None,
        mint_to: None,
    };
    let res = execute(deps.as_mut(), mock_env(), info.clone(), msg);
    assert!(res.is_ok());
}
