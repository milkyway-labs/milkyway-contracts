use crate::contract::{execute, query};
use crate::msg::{
    BatchResponse, BatchesResponse, ConfigResponse, ExecuteMsg, QueryMsg, StateResponse,
};
use crate::query::{query_admin, query_pending_batch};
use crate::state::{CONFIG, STATE};
use crate::tests::test_helper::{
    init, ADMIN, CELESTIA2, CELESTIAVAL1, CELESTIAVAL2, CHANNEL_ID, LIQUID_STAKE_TOKEN_DENOM,
    NATIVE_TOKEN, OSMO2, OSMO3, OSMO4, STAKER_ADDRESS,
};
use cosmwasm_std::testing::{mock_env, mock_info};
use cosmwasm_std::{coins, from_json, Addr, Decimal, Uint128};

#[test]
fn get_config() {
    let deps = init();
    let msg = QueryMsg::Config {};
    let bin = query(deps.as_ref(), mock_env(), msg.clone()).unwrap();
    let result = from_json::<ConfigResponse>(&bin).unwrap();

    // Native chain config
    assert_eq!(result.native_chain_config.token_denom, "utia");
    assert_eq!(
        result.native_chain_config.account_address_prefix,
        "celestia"
    );
    assert_eq!(
        result.native_chain_config.validator_address_prefix,
        "celestiavaloper"
    );
    assert_eq!(
        result.native_chain_config.validators,
        vec![CELESTIAVAL1.to_string(), CELESTIAVAL2.to_string()]
    );
    assert_eq!(result.native_chain_config.unbonding_period, 1209600);
    assert_eq!(result.native_chain_config.staker_address, STAKER_ADDRESS);
    assert_eq!(
        result.native_chain_config.reward_collector_address,
        CELESTIA2
    );

    // Protocol chain config
    assert_eq!(result.protocol_chain_config.account_address_prefix, "osmo");
    assert_eq!(result.protocol_chain_config.ibc_token_denom, NATIVE_TOKEN);
    assert_eq!(result.protocol_chain_config.ibc_channel_id, CHANNEL_ID);
    assert_eq!(
        result.protocol_chain_config.oracle_address,
        Some(Addr::unchecked(OSMO4))
    );
    assert_eq!(
        result.protocol_chain_config.minimum_liquid_stake_amount,
        Uint128::from(100u128)
    );

    // other Config struct fields
    assert_eq!(
        result.liquid_stake_token_denom,
        format!("factory/cosmos2contract/{}", LIQUID_STAKE_TOKEN_DENOM)
    );
    assert_eq!(result.monitors, vec![OSMO2.to_string(), OSMO3.to_string()]);
    assert_eq!(result.batch_period, 86400);
    assert!(!result.stopped);
}

#[test]
fn get_state() {
    let mut deps = init();

    let msg = QueryMsg::State {};
    let mut bin = query(deps.as_ref(), mock_env(), msg.clone()).unwrap();
    let mut result = from_json::<StateResponse>(&bin);

    match result {
        Ok(res) => {
            assert_eq!(res.total_native_token, Uint128::from(0u128));
            assert_eq!(res.total_liquid_stake_token, Uint128::from(0u128));
            assert_eq!(res.rate, Decimal::one());
            assert_eq!(res.pending_owner, "".to_string());
            assert_eq!(res.total_reward_amount, Uint128::from(0u128));
        }
        Err(e) => panic!("Unexpected error: {:?}", e),
    }

    // stake
    let info = mock_info(OSMO3, &coins(1000, NATIVE_TOKEN));
    let stake_msg = ExecuteMsg::LiquidStake {
        mint_to: None,
        transfer_to_native_chain: None,
        expected_mint_amount: None,
    };
    let res = execute(deps.as_mut(), mock_env(), info, stake_msg);
    assert!(res.is_ok());

    let mut state = STATE.load(&deps.storage).unwrap();
    state.total_fees = Uint128::from(100u128);
    STATE.save(&mut deps.storage, &state).unwrap();

    // check the state
    let msg = QueryMsg::State {};
    bin = query(deps.as_ref(), mock_env(), msg.clone()).unwrap();
    result = from_json::<StateResponse>(&bin);
    match result {
        Ok(res) => {
            assert_eq!(res.total_native_token, Uint128::from(1000u128));
            assert_eq!(res.total_liquid_stake_token, Uint128::from(1000u128));
            assert_eq!(
                res.rate,
                Decimal::from_ratio(res.total_liquid_stake_token, res.total_native_token)
            );
            assert_eq!(res.total_fees, Uint128::from(100u128))
        }
        Err(e) => panic!("Unexpected error: {:?}", e),
    }
}

#[test]
fn get_batch() {
    let mut deps = init();
    let msg = QueryMsg::Batch { id: 1 };
    let bin = query(deps.as_ref(), mock_env(), msg).unwrap();
    let result = from_json::<BatchResponse>(&bin);

    // we have 1 pending batch at the beginning
    match result {
        Ok(res) => {
            assert_eq!(res.batch_total_liquid_stake, Uint128::from(0u128));
            assert_eq!(res.expected_native_unstaked, Uint128::from(0u128));
            assert_eq!(res.status, "pending".to_string());
        }
        Err(e) => panic!("Unexpected error: {:?}", e),
    }
    // find Batch with id = 2
    let msg = QueryMsg::Batch { id: 2 };
    let result = query(deps.as_ref(), mock_env(), msg);
    assert!(result.is_err()); //not found

    // unStake 1
    let info = mock_info(
        "bob",
        &coins(
            500,
            format!("factory/cosmos2contract/{}", LIQUID_STAKE_TOKEN_DENOM),
        ),
    );
    let unstake_msg = ExecuteMsg::LiquidUnstake {};
    let res = execute(deps.as_mut(), mock_env(), info, unstake_msg.clone());
    assert!(res.is_ok());

    // unStake 2
    let info = mock_info(
        "alice",
        &coins(
            1500,
            format!("factory/cosmos2contract/{}", LIQUID_STAKE_TOKEN_DENOM),
        ),
    );
    let res = execute(deps.as_mut(), mock_env(), info, unstake_msg);
    assert!(res.is_ok());

    let msg = QueryMsg::Batch { id: 1 };
    let bin = query(deps.as_ref(), mock_env(), msg).unwrap();
    let result = from_json::<BatchResponse>(&bin);
    match result {
        Ok(res) => {
            assert_eq!(res.batch_total_liquid_stake, Uint128::from(2000u128));
            assert_eq!(res.expected_native_unstaked, Uint128::from(0u128));
            assert_eq!(res.status, "pending".to_string());
        }
        Err(e) => panic!("Unexpected error: {:?}", e),
    }
}

#[test]
fn get_batches() {
    let mut deps = init();
    let mut env = mock_env();
    let msg = QueryMsg::Batches {
        start_after: None,
        limit: None,
        status: None,
    };
    let mut bin = query(deps.as_ref(), env.clone(), msg.clone()).unwrap();
    let mut result = from_json::<BatchesResponse>(&bin);

    let mut state = STATE.load(&deps.storage).unwrap();
    state.total_liquid_stake_token = Uint128::from(100_000_000u128);
    state.total_native_token = Uint128::from(300_000_000u128);
    STATE.save(&mut deps.storage, &state).unwrap();

    match result {
        Ok(res) => {
            assert_eq!(res.batches.len(), 1);
            if let Some(first_batch) = res.batches.first() {
                assert_eq!(first_batch.batch_total_liquid_stake, Uint128::from(0u128));
                assert_eq!(first_batch.expected_native_unstaked, Uint128::from(0u128));
                assert_eq!(first_batch.status, "pending".to_string());
            } else {
                panic!("batches is empty");
            }
        }
        Err(e) => panic!("Unexpected error: {:?}", e),
    }

    // unStake 1
    let info = mock_info(
        "bob",
        &coins(
            500,
            format!("factory/cosmos2contract/{}", LIQUID_STAKE_TOKEN_DENOM),
        ),
    );
    let unstake_msg = ExecuteMsg::LiquidUnstake {};
    let res = execute(deps.as_mut(), env.clone(), info, unstake_msg.clone());
    assert!(res.is_ok());

    // submit batch
    let config = CONFIG.load(&deps.storage).unwrap();
    env.block.time = env.block.time.plus_seconds(config.batch_period + 1);
    let submit_batch_msg = ExecuteMsg::SubmitBatch {};
    let contract = env.contract.address.clone().to_string();
    let submit_info = mock_info(&contract, &[]);
    let res = execute(deps.as_mut(), env.clone(), submit_info, submit_batch_msg);
    assert!(res.is_ok());

    // unStake 2 - for the next batch
    let info = mock_info(
        "alice",
        &coins(
            1500,
            format!("factory/cosmos2contract/{}", LIQUID_STAKE_TOKEN_DENOM),
        ),
    );
    let res = execute(deps.as_mut(), env.clone(), info, unstake_msg);
    assert!(res.is_ok());

    // check the state
    bin = query(deps.as_ref(), env.clone(), msg).unwrap();
    result = from_json::<BatchesResponse>(&bin);
    match result {
        Ok(res) => {
            assert_eq!(res.batches.len(), 2);
            if let Some(first_batch) = res.batches.first() {
                assert_eq!(first_batch.batch_total_liquid_stake, Uint128::from(500u128));
                assert_eq!(
                    first_batch.expected_native_unstaked,
                    Uint128::from(1500u128)
                );
                assert_eq!(first_batch.status, "submitted".to_string());
            } else {
                panic!("batches is empty");
            }

            if let Some(first_batch) = res.batches.get(1) {
                assert_eq!(
                    first_batch.batch_total_liquid_stake,
                    Uint128::from(1500u128)
                );
                assert_eq!(first_batch.expected_native_unstaked, Uint128::from(0u128));
                assert_eq!(first_batch.status, "pending".to_string());
            } else {
                panic!("batches is empty");
            }
        }
        Err(e) => panic!("Unexpected error: {:?}", e),
    }

    // query only submitted batches
    let msg = QueryMsg::Batches {
        start_after: None,
        limit: None,
        status: Some(milky_way::staking::BatchStatus::Submitted),
    };
    bin = query(deps.as_ref(), env.clone(), msg).unwrap();
    result = from_json::<BatchesResponse>(&bin);
    assert_eq!(result.unwrap().batches.len(), 1);

    // query only pending batches - there must always be 1
    let msg = QueryMsg::Batches {
        start_after: None,
        limit: None,
        status: Some(milky_way::staking::BatchStatus::Pending),
    };
    bin = query(deps.as_ref(), env.clone(), msg).unwrap();
    result = from_json::<BatchesResponse>(&bin);
    assert_eq!(result.unwrap().batches.len(), 1);
}

#[test]
fn get_pending_batch() {
    let mut deps = init();
    let mut env = mock_env();

    let pending_batch_id = query_pending_batch(deps.as_ref());
    assert!(pending_batch_id.unwrap().id == 1);

    let config = CONFIG.load(&deps.storage).unwrap();

    let mut state = STATE.load(&deps.storage).unwrap();

    state.total_liquid_stake_token = Uint128::from(100_000u128);
    state.total_native_token = Uint128::from(300_000u128);
    STATE.save(&mut deps.storage, &state).unwrap();

    let info = mock_info(
        "bob",
        &coins(
            1000,
            format!("factory/cosmos2contract/{}", LIQUID_STAKE_TOKEN_DENOM),
        ),
    );
    let msg = ExecuteMsg::LiquidUnstake {};
    let _res = execute(deps.as_mut(), mock_env(), info.clone(), msg);

    env.block.time = env.block.time.plus_seconds(config.batch_period + 1);
    let submit_batch_msg = ExecuteMsg::SubmitBatch {};
    let contract = env.contract.address.clone().to_string();
    let submit_info = mock_info(&contract, &[]);
    let res = execute(deps.as_mut(), env.clone(), submit_info, submit_batch_msg);
    if res.is_err() {
        // print error
        let err = res.err().unwrap();
        panic!("Error: {:?}", err);
    }
    assert!(res.is_ok());

    let pending_batch_id = query_pending_batch(deps.as_ref());
    assert!(pending_batch_id.unwrap().id == 2);
}

#[test]
fn get_admin() {
    let deps = init();

    let admin_response = query_admin(deps.as_ref()).unwrap();
    assert_eq!(ADMIN, admin_response.admin.unwrap().as_str())
}
