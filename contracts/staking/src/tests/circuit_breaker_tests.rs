use crate::contract::execute;
use crate::helpers::derive_intermediate_sender;
use crate::msg::ExecuteMsg;
use crate::state::{new_unstake_request, State, BATCHES, CONFIG, STATE};
use crate::tests::test_helper::{init, NATIVE_TOKEN, OSMO2, OSMO3};
use cosmwasm_std::testing::{message_info, mock_env};
use cosmwasm_std::{coins, Addr, Coin, Uint128};
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

    let contract = env.contract.address.clone();

    // not correct sender
    let info = message_info(&contract, &[]);
    let res = execute(deps.as_mut(), env.clone(), info, msg.clone());

    assert!(res.is_err());

    // correct sender (admin)
    let info = message_info(&Addr::unchecked(OSMO3), &[]);
    let res = execute(deps.as_mut(), env.clone(), info, msg.clone());

    assert!(res.is_ok());

    // correct sender (operator)
    let info = message_info(&Addr::unchecked(OSMO2), &[]);
    let res = execute(deps.as_mut(), env.clone(), info, msg.clone());

    assert!(res.is_ok());

    // liquid stake
    let info = message_info(&Addr::unchecked(OSMO3), &coins(1000, "osmoTIA"));
    let msg = ExecuteMsg::LiquidStake {
        mint_to: None,
        expected_mint_amount: None,
    };
    let res = execute(deps.as_mut(), mock_env(), info.clone(), msg);
    assert!(res.is_err());

    // liquid unstake
    state.total_liquid_stake_token = Uint128::from(100_000u128);
    state.total_native_token = Uint128::from(300_000u128);
    STATE.save(&mut deps.storage, &state).unwrap();
    let info = message_info(
        &Addr::unchecked("bob"),
        &coins(1000, "factory/cosmos2contract/stTIA"),
    );
    let msg = ExecuteMsg::LiquidUnstake {};
    let res = execute(deps.as_mut(), mock_env(), info.clone(), msg);
    assert!(res.is_err());

    // receive rewards
    let msg = ExecuteMsg::ReceiveRewards {};
    let sender = derive_intermediate_sender(
        &config.ibc_channel_id,
        config
            .multisig_address_config
            .reward_collector_address
            .as_ref(),
        "osmo",
    )
    .unwrap();
    let info = message_info(
        &Addr::unchecked(sender.clone()),
        &[Coin {
            amount: Uint128::from(100u128),
            denom: config.native_token_denom.clone(),
        }],
    );
    let res = execute(deps.as_mut(), env.clone(), info, msg.clone());
    assert!(res.is_err());

    // receive unstaked tokens
    let msg = ExecuteMsg::ReceiveUnstakedTokens { batch_id: 1 };
    let info = message_info(
        &Addr::unchecked(sender),
        &[Coin {
            amount: Uint128::from(100u128),
            denom: config.native_token_denom.clone(),
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
    let info = message_info(&Addr::unchecked("bob"), &[]);
    let res = execute(deps.as_mut(), env.clone(), info, msg.clone());
    assert!(res.is_err());

    // submit batch
    env.block.time = env.block.time.plus_seconds(config.batch_period - 1);
    let msg = ExecuteMsg::SubmitBatch {};
    let info = message_info(&contract, &[]);
    let res = execute(deps.as_mut(), env.clone(), info, msg.clone());
    assert!(res.is_err());

    // reenable
    let msg = ExecuteMsg::ResumeContract {
        total_native_token: Uint128::from(100000u128),
        total_liquid_stake_token: Uint128::from(200000u128),
        total_reward_amount: Uint128::from(10000u128),
    };

    // not correct sender
    let info = message_info(&contract, &[]);
    let res = execute(deps.as_mut(), env.clone(), info, msg.clone());

    assert!(res.is_err());

    // correct sender
    let info = message_info(&Addr::unchecked(OSMO3), &[]);
    let res = execute(deps.as_mut(), env.clone(), info, msg.clone());

    assert!(res.is_ok());

    // test accounting update
    let state: State = STATE.load(&deps.storage).unwrap();
    assert_eq!(state.total_liquid_stake_token, Uint128::from(200000u128));
    assert_eq!(state.total_native_token, Uint128::from(100000u128));
    assert_eq!(state.total_reward_amount, Uint128::from(10000u128));

    // test enabled
    let info = message_info(&Addr::unchecked(OSMO3), &coins(1000, NATIVE_TOKEN));
    let msg = ExecuteMsg::LiquidStake {
        expected_mint_amount: None,
        mint_to: None,
    };
    let res = execute(deps.as_mut(), mock_env(), info.clone(), msg);
    assert!(res.is_ok());
}
