use crate::state::{NativeChainConfig, ProtocolChainConfig, ProtocolFeeConfig, BATCHES, CONFIG};
use crate::tests::test_helper::{
    mock_init_msg, CELESTIA1, CELESTIA2, CELESTIAVAL1, CELESTIAVAL2, CHANNEL_ID, NATIVE_TOKEN,
    OSMO1, OSMO2, OSMO3, OSMO4, STAKER_ADDRESS,
};
use crate::types::MAX_UNBONDING_PERIOD;

use cosmwasm_std::testing::{mock_dependencies, mock_info};
use cosmwasm_std::{Addr, Order, Uint128};
use milky_way::staking::BatchStatus;

#[test]
fn invalid_native_chain_account_address_prefix_fails() {
    let mut deps = mock_dependencies();
    let info = mock_info(OSMO3, &[]);
    let mut msg = mock_init_msg();

    msg.native_chain_config.account_address_prefix = "".to_string();
    let res = crate::contract::instantiate(
        deps.as_mut(),
        cosmwasm_std::testing::mock_env(),
        info.clone(),
        msg,
    );
    assert!(res.is_err());
}

#[test]
fn invalid_native_chain_validator_address_prefix_fails() {
    let mut deps = mock_dependencies();
    let info = mock_info(OSMO3, &[]);
    let mut msg = mock_init_msg();

    msg.native_chain_config.validator_address_prefix = "".to_string();
    let res = crate::contract::instantiate(
        deps.as_mut(),
        cosmwasm_std::testing::mock_env(),
        info.clone(),
        msg,
    );
    assert!(res.is_err());
}

#[test]
fn invalid_native_token_denom_fails() {
    let mut deps = mock_dependencies();
    let info = mock_info(OSMO3, &[]);
    let mut msg = mock_init_msg();

    msg.native_chain_config.token_denom = "".to_string();
    let res = crate::contract::instantiate(
        deps.as_mut(),
        cosmwasm_std::testing::mock_env(),
        info.clone(),
        msg,
    );
    assert!(res.is_err());
}

#[test]
fn invalid_native_chain_validators_fails() {
    let mut deps = mock_dependencies();
    let info = mock_info(OSMO3, &[]);
    let mut msg = mock_init_msg();

    msg.native_chain_config.validators[1] = OSMO1.to_string();
    let res = crate::contract::instantiate(
        deps.as_mut(),
        cosmwasm_std::testing::mock_env(),
        info.clone(),
        msg,
    );
    assert!(res.is_err());
}

#[test]
fn invalid_staker_fails() {
    let mut deps = mock_dependencies();
    let info = mock_info(OSMO3, &[]);
    let mut msg = mock_init_msg();

    msg.native_chain_config.staker_address = OSMO1.to_string();
    let res = crate::contract::instantiate(
        deps.as_mut(),
        cosmwasm_std::testing::mock_env(),
        info.clone(),
        msg,
    );
    assert!(res.is_err());
}

#[test]
fn invalid_rewards_collector_fails() {
    let mut deps = mock_dependencies();
    let info = mock_info(OSMO3, &[]);
    let mut msg = mock_init_msg();

    msg.native_chain_config.reward_collector_address = OSMO1.to_string();
    let res = crate::contract::instantiate(
        deps.as_mut(),
        cosmwasm_std::testing::mock_env(),
        info.clone(),
        msg,
    );
    assert!(res.is_err());
}

#[test]
fn invalid_protocol_chain_account_address_prefix_fails() {
    let mut deps = mock_dependencies();
    let info = mock_info(OSMO3, &[]);
    let mut msg = mock_init_msg();

    msg.protocol_chain_config.account_address_prefix = "".to_string();
    let res = crate::contract::instantiate(
        deps.as_mut(),
        cosmwasm_std::testing::mock_env(),
        info.clone(),
        msg,
    );
    assert!(res.is_err());
}

#[test]
fn invalid_protocol_ibc_token_denom_fails() {
    let mut deps = mock_dependencies();
    let info = mock_info(OSMO3, &[]);
    let mut msg = mock_init_msg();

    msg.protocol_chain_config.ibc_token_denom = "utia".to_string();
    let res = crate::contract::instantiate(
        deps.as_mut(),
        cosmwasm_std::testing::mock_env(),
        info.clone(),
        msg,
    );
    assert!(res.is_err());
}

#[test]
fn invalid_lst_token_denom_fails() {
    let mut deps = mock_dependencies();
    let info = mock_info(OSMO3, &[]);
    let mut msg = mock_init_msg();

    msg.liquid_stake_token_denom = "".to_string();
    let res = crate::contract::instantiate(
        deps.as_mut(),
        cosmwasm_std::testing::mock_env(),
        info.clone(),
        msg,
    );
    assert!(res.is_err());
}

#[test]
fn invalid_protocol_ibc_channel_id_fails() {
    let mut deps = mock_dependencies();
    let info = mock_info(OSMO3, &[]);
    let mut msg = mock_init_msg();

    msg.protocol_chain_config.ibc_channel_id = "".to_string();
    let res = crate::contract::instantiate(
        deps.as_mut(),
        cosmwasm_std::testing::mock_env(),
        info.clone(),
        msg,
    );
    assert!(res.is_err());
}

#[test]
fn invalid_protocol_treasury_address_fails() {
    let mut deps = mock_dependencies();
    let info = mock_info(OSMO3, &[]);
    let mut msg = mock_init_msg();

    msg.protocol_fee_config.treasury_address = Some(CELESTIA1.to_string());
    let res = crate::contract::instantiate(
        deps.as_mut(),
        cosmwasm_std::testing::mock_env(),
        info.clone(),
        msg,
    );
    assert!(res.is_err());
}

#[test]
fn invalid_monitors_fails() {
    let mut deps = mock_dependencies();
    let info = mock_info(OSMO3, &[]);
    let mut msg = mock_init_msg();

    msg.monitors[1] = CELESTIA1.to_string();
    let res = crate::contract::instantiate(
        deps.as_mut(),
        cosmwasm_std::testing::mock_env(),
        info.clone(),
        msg,
    );
    assert!(res.is_err());
}

#[test]
fn invalid_unbonding_period_fails() {
    let mut deps = mock_dependencies();
    let info = mock_info(OSMO3, &[]);
    let mut msg = mock_init_msg();

    msg.native_chain_config.unbonding_period = MAX_UNBONDING_PERIOD + 1;
    let res = crate::contract::instantiate(
        deps.as_mut(),
        cosmwasm_std::testing::mock_env(),
        info.clone(),
        msg,
    );
    assert!(res.is_err());
}

#[test]
fn invalid_batch_period_fails() {
    let mut deps = mock_dependencies();
    let info = mock_info(OSMO3, &[]);
    let mut msg = mock_init_msg();

    msg.batch_period = msg.native_chain_config.unbonding_period + 1;
    let res = crate::contract::instantiate(
        deps.as_mut(),
        cosmwasm_std::testing::mock_env(),
        info.clone(),
        msg,
    );
    assert!(res.is_err());
}

#[test]
fn init_properly() {
    let mut deps = mock_dependencies();
    let info = mock_info(OSMO3, &[]);
    let msg = mock_init_msg();

    let res = crate::contract::instantiate(
        deps.as_mut(),
        cosmwasm_std::testing::mock_env(),
        info.clone(),
        msg.clone(),
    );
    assert!(res.is_ok());

    let pending_batch = BATCHES
        .range(&deps.storage, None, None, Order::Descending)
        .find(|r| r.is_ok() && r.as_ref().unwrap().1.status == BatchStatus::Pending)
        .unwrap()
        .unwrap()
        .1;
    assert!(pending_batch.id == 1);

    let config = CONFIG.load(&deps.storage).unwrap();
    assert_eq!(
        NativeChainConfig {
            token_denom: "utia".to_string(),
            account_address_prefix: "celestia".to_string(),
            validator_address_prefix: "celestiavaloper".to_string(),
            validators: vec![Addr::unchecked(CELESTIAVAL1), Addr::unchecked(CELESTIAVAL2)],
            unbonding_period: 1209600,
            staker_address: Addr::unchecked(STAKER_ADDRESS),
            reward_collector_address: Addr::unchecked(CELESTIA2),
        },
        config.native_chain_config
    );
    assert_eq!(
        ProtocolChainConfig {
            account_address_prefix: "osmo".to_string(),
            ibc_token_denom: NATIVE_TOKEN.to_string(),
            ibc_channel_id: CHANNEL_ID.to_string(),
            oracle_address: Some(Addr::unchecked(OSMO4)),
            minimum_liquid_stake_amount: Uint128::from(100u128),
        },
        config.protocol_chain_config
    );
    assert_eq!(
        ProtocolFeeConfig {
            dao_treasury_fee: Uint128::from(10000u128),
            treasury_address: Some(Addr::unchecked(OSMO1)),
        },
        config.protocol_fee_config
    );
    assert_eq!(
        vec![Addr::unchecked(OSMO2), Addr::unchecked(OSMO3)],
        config.monitors
    );
    assert_eq!(86400, config.batch_period);
}
