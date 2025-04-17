use crate::contract::execute;
use crate::error::ContractError;
use crate::msg::ExecuteMsg;
use crate::state::{BATCHES, CONFIG, PENDING_BATCH_ID, STATE};
use crate::tests::test_helper::{init, OSMO1};
use cosmwasm_std::testing::{mock_env, mock_info};
use cosmwasm_std::Uint128;
use milky_way::staking::{Batch, BatchStatus};

#[test]
fn empty_submit_batch() {
    let mut deps = init();
    let mut env = mock_env();

    let state = STATE.load(&deps.storage).unwrap();
    let config = CONFIG.load(&deps.storage).unwrap();

    STATE.save(&mut deps.storage, &state).unwrap();

    env.block.time = env.block.time.plus_seconds(config.batch_period + 1);
    let msg = ExecuteMsg::SubmitBatch {};

    let contract = env.contract.address.clone().to_string();

    let info = mock_info(&contract, &[]);
    let res = execute(deps.as_mut(), env, info, msg);
    assert!(res.is_err());
}

#[test]
fn not_ready_submit_batch() {
    let mut deps = init();
    let mut env = mock_env();

    let mut state = STATE.load(&deps.storage).unwrap();
    let config = CONFIG.load(&deps.storage).unwrap();

    state.total_liquid_stake_token = Uint128::from(100_000u128);
    state.total_native_token = Uint128::from(300_000u128);
    STATE.save(&mut deps.storage, &state).unwrap();

    // batch isnt ready
    env.block.time = env.block.time.plus_seconds(config.batch_period - 1);
    let msg = ExecuteMsg::SubmitBatch {};

    let contract = env.contract.address.clone().to_string();

    let info = mock_info(&contract, &[]);
    let res = execute(deps.as_mut(), env, info, msg);

    assert!(res.is_err());
}

#[test]
fn pending_batch_with_to_many_lst_tokens_fails() {
    let mut deps = init();
    let mut env = mock_env();

    let mut state = STATE.load(&deps.storage).unwrap();
    let config = CONFIG.load(&deps.storage).unwrap();

    state.total_liquid_stake_token = Uint128::from(100_000u128);
    state.total_native_token = Uint128::from(300_000u128);
    STATE.save(&mut deps.storage, &state).unwrap();

    // Create a pending batch with to many tokens
    PENDING_BATCH_ID.save(&mut deps.storage, &1).unwrap();
    BATCHES
        .save(
            &mut deps.storage,
            1,
            &Batch {
                id: 1,
                unstake_requests_count: Some(1),
                batch_total_liquid_stake: Uint128::new(state.total_liquid_stake_token.u128() + 1),
                status: BatchStatus::Received,
                next_batch_action_time: Some(
                    env.block.time.plus_seconds(config.batch_period).seconds(),
                ),
                liquid_unstake_requests: None,
                expected_native_unstaked: None,
                received_native_unstaked: None,
            },
        )
        .unwrap();

    // Update the time to simulate batch readiness.
    env.block.time = env.block.time.plus_seconds(config.batch_period + 1);

    let msg = ExecuteMsg::SubmitBatch {};
    let info = mock_info(OSMO1, &[]);
    let err = execute(deps.as_mut(), env, info, msg).unwrap_err();

    assert!(match err {
        ContractError::InvalidUnstakeAmount { .. } => true,
        _ => false,
    })
}
