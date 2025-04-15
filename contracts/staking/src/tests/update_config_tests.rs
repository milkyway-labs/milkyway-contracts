use cosmwasm_std::{Addr, Uint128};

use crate::{
    state::CONFIG,
    tests::test_helper::{
        init, CELESTIA1, CELESTIA2, CELESTIAVAL1, CHANNEL_ID, NATIVE_TOKEN, OSMO1, OSMO3, OSMO4,
    },
    types::{
        UnsafeNativeChainConfig, UnsafeProtocolChainConfig, UnsafeProtocolFeeConfig,
        MAX_UNBONDING_PERIOD,
    },
};

#[test]
fn update_native_chain_config_with_invalid_staker_fails() {
    let mut deps = init();
    let info = cosmwasm_std::testing::mock_info(OSMO3, &[]);
    let config_update_msg = crate::msg::ExecuteMsg::UpdateConfig {
        native_chain_config: Some(UnsafeNativeChainConfig {
            account_address_prefix: "celestia".to_string(),
            validator_address_prefix: "celestiavaloper".to_string(),
            staker_address: OSMO1.to_string(),
            reward_collector_address: CELESTIA2.to_string(),
            token_denom: "utia".to_string(),
            unbonding_period: 1209600,
            validators: vec![],
        }),
        protocol_chain_config: None,
        protocol_fee_config: None,
        batch_period: None,
        monitors: None,
    };

    let res = crate::contract::execute(
        deps.as_mut(),
        cosmwasm_std::testing::mock_env(),
        info.clone(),
        config_update_msg,
    );
    assert!(res.is_err());
}

#[test]
fn update_native_chain_config_with_invalid_reward_collector_fails() {
    let mut deps = init();
    let info = cosmwasm_std::testing::mock_info(OSMO3, &[]);
    let config_update_msg = crate::msg::ExecuteMsg::UpdateConfig {
        native_chain_config: Some(UnsafeNativeChainConfig {
            account_address_prefix: "celestia".to_string(),
            validator_address_prefix: "celestiavaloper".to_string(),
            staker_address: CELESTIA1.to_string(),
            reward_collector_address: OSMO1.to_string(),
            token_denom: "utia".to_string(),
            unbonding_period: 1209600,
            validators: vec![],
        }),
        protocol_chain_config: None,
        protocol_fee_config: None,
        batch_period: None,
        monitors: None,
    };

    let res = crate::contract::execute(
        deps.as_mut(),
        cosmwasm_std::testing::mock_env(),
        info.clone(),
        config_update_msg,
    );
    assert!(res.is_err());
}

#[test]
fn update_native_chain_config_with_invalid_validator_fails() {
    let mut deps = init();
    let info = cosmwasm_std::testing::mock_info(OSMO3, &[]);
    let config_update_msg = crate::msg::ExecuteMsg::UpdateConfig {
        native_chain_config: Some(UnsafeNativeChainConfig {
            account_address_prefix: "celestia".to_string(),
            validator_address_prefix: "celestiavaloper".to_string(),
            staker_address: CELESTIA1.to_string(),
            reward_collector_address: CELESTIA1.to_string(),
            token_denom: "utia".to_string(),
            unbonding_period: 1209600,
            validators: vec!["osmovaloper1clpqr4nrk4khgkxj78fcwwh6dl3uw4ep88n0y4".to_string()],
        }),
        protocol_chain_config: None,
        protocol_fee_config: None,
        batch_period: None,
        monitors: None,
    };

    let res = crate::contract::execute(
        deps.as_mut(),
        cosmwasm_std::testing::mock_env(),
        info.clone(),
        config_update_msg,
    );
    assert!(res.is_err());
}

#[test]
fn update_native_chain_config_with_invalid_unbonding_period_fails() {
    let mut deps = init();
    let info = cosmwasm_std::testing::mock_info(OSMO3, &[]);
    let config_update_msg = crate::msg::ExecuteMsg::UpdateConfig {
        native_chain_config: Some(UnsafeNativeChainConfig {
            account_address_prefix: "celestia".to_string(),
            validator_address_prefix: "celestiavaloper".to_string(),
            staker_address: CELESTIA1.to_string(),
            reward_collector_address: CELESTIA1.to_string(),
            token_denom: "utia".to_string(),
            unbonding_period: MAX_UNBONDING_PERIOD + 1,
            validators: vec!["osmovaloper1clpqr4nrk4khgkxj78fcwwh6dl3uw4ep88n0y4".to_string()],
        }),
        protocol_chain_config: None,
        protocol_fee_config: None,
        batch_period: None,
        monitors: None,
    };

    let res = crate::contract::execute(
        deps.as_mut(),
        cosmwasm_std::testing::mock_env(),
        info.clone(),
        config_update_msg,
    );
    assert!(res.is_err());
}

#[test]
fn update_native_chain_config_properly() {
    let mut deps = init();
    let info = cosmwasm_std::testing::mock_info(OSMO3, &[]);

    let new_config = UnsafeNativeChainConfig {
        account_address_prefix: "celestia".to_string(),
        validator_address_prefix: "celestiavaloper".to_string(),
        staker_address: CELESTIA1.to_string(),
        reward_collector_address: CELESTIA2.to_string(),
        token_denom: "utia".to_string(),
        unbonding_period: 1209600,
        validators: vec![CELESTIAVAL1.to_string()],
    };
    let config_update_msg = crate::msg::ExecuteMsg::UpdateConfig {
        native_chain_config: Some(new_config.clone()),
        protocol_chain_config: None,
        protocol_fee_config: None,
        batch_period: None,
        monitors: None,
    };

    crate::contract::execute(
        deps.as_mut(),
        cosmwasm_std::testing::mock_env(),
        info.clone(),
        config_update_msg,
    )
    .unwrap();

    let config = CONFIG.load(&deps.storage).unwrap();

    assert_eq!(
        new_config.account_address_prefix,
        config.native_chain_config.account_address_prefix
    );
    assert_eq!(
        new_config.validator_address_prefix,
        config.native_chain_config.validator_address_prefix
    );
    assert_eq!(
        Addr::unchecked(new_config.staker_address),
        config.native_chain_config.staker_address
    );
    assert_eq!(
        Addr::unchecked(new_config.reward_collector_address),
        config.native_chain_config.reward_collector_address
    );
    assert_eq!(
        new_config.token_denom,
        config.native_chain_config.token_denom
    );
    assert_eq!(
        new_config.unbonding_period,
        config.native_chain_config.unbonding_period
    );
    assert_eq!(
        vec![Addr::unchecked(CELESTIAVAL1)],
        config.native_chain_config.validators
    );
}

#[test]
fn update_protocol_chain_config_with_invalid_account_address_prefix_fails() {
    let mut deps = init();
    let info = cosmwasm_std::testing::mock_info(OSMO3, &[]);
    let config_update_msg = crate::msg::ExecuteMsg::UpdateConfig {
        native_chain_config: None,
        protocol_chain_config: Some(UnsafeProtocolChainConfig {
            account_address_prefix: "".to_string(),
            ibc_token_denom: NATIVE_TOKEN.to_string(),
            ibc_channel_id: CHANNEL_ID.to_string(),
            oracle_address: Some(OSMO4.to_string()),
            minimum_liquid_stake_amount: Uint128::from(100u128),
        }),
        protocol_fee_config: None,
        batch_period: None,
        monitors: None,
    };

    let res = crate::contract::execute(
        deps.as_mut(),
        cosmwasm_std::testing::mock_env(),
        info.clone(),
        config_update_msg,
    );
    assert!(res.is_err());
}

#[test]
fn update_protocol_chain_config_with_invalid_ibc_token_fails() {
    let mut deps = init();
    let info = cosmwasm_std::testing::mock_info(OSMO3, &[]);
    let config_update_msg = crate::msg::ExecuteMsg::UpdateConfig {
        native_chain_config: None,
        protocol_chain_config: Some(UnsafeProtocolChainConfig {
            account_address_prefix: "osmo".to_string(),
            ibc_token_denom: "utia".to_string(),
            ibc_channel_id: CHANNEL_ID.to_string(),
            oracle_address: Some(OSMO4.to_string()),
            minimum_liquid_stake_amount: Uint128::from(100u128),
        }),
        protocol_fee_config: None,
        batch_period: None,
        monitors: None,
    };

    let res = crate::contract::execute(
        deps.as_mut(),
        cosmwasm_std::testing::mock_env(),
        info.clone(),
        config_update_msg,
    );
    assert!(res.is_err());
}

#[test]
fn update_protocol_chain_config_with_invalid_ibc_channel_id_fails() {
    let mut deps = init();
    let info = cosmwasm_std::testing::mock_info(OSMO3, &[]);
    let config_update_msg = crate::msg::ExecuteMsg::UpdateConfig {
        native_chain_config: None,
        protocol_chain_config: Some(UnsafeProtocolChainConfig {
            account_address_prefix: "osmo".to_string(),
            ibc_token_denom: NATIVE_TOKEN.to_string(),
            ibc_channel_id: "".to_string(),
            oracle_address: Some(OSMO4.to_string()),
            minimum_liquid_stake_amount: Uint128::from(100u128),
        }),
        protocol_fee_config: None,
        batch_period: None,
        monitors: None,
    };

    let res = crate::contract::execute(
        deps.as_mut(),
        cosmwasm_std::testing::mock_env(),
        info.clone(),
        config_update_msg,
    );
    assert!(res.is_err());
}

#[test]
fn update_protocol_chain_config_with_invalid_oracle_address_fails() {
    let mut deps = init();
    let info = cosmwasm_std::testing::mock_info(OSMO3, &[]);
    let config_update_msg = crate::msg::ExecuteMsg::UpdateConfig {
        native_chain_config: None,
        protocol_chain_config: Some(UnsafeProtocolChainConfig {
            account_address_prefix: "osmo".to_string(),
            ibc_token_denom: NATIVE_TOKEN.to_string(),
            ibc_channel_id: CHANNEL_ID.to_string(),
            oracle_address: Some(CELESTIA1.to_string()),
            minimum_liquid_stake_amount: Uint128::from(100u128),
        }),
        protocol_fee_config: None,
        batch_period: None,
        monitors: None,
    };

    let res = crate::contract::execute(
        deps.as_mut(),
        cosmwasm_std::testing::mock_env(),
        info.clone(),
        config_update_msg,
    );
    assert!(res.is_err());
}

#[test]
fn update_protocol_chain_config_properly() {
    let mut deps = init();
    let info = cosmwasm_std::testing::mock_info(OSMO3, &[]);

    let new_config = UnsafeProtocolChainConfig {
        account_address_prefix: "celestia".to_string(),
        ibc_token_denom: "ibc/C3E53D20BC7A4CC993B17C7971F8ECD06A433C10B6A96F4C4C3714F0624C56AA"
            .to_string(),
        ibc_channel_id: "channel-234".to_string(),
        oracle_address: Some(CELESTIA1.to_string()),
        minimum_liquid_stake_amount: Uint128::from(1000u128),
    };
    let config_update_msg = crate::msg::ExecuteMsg::UpdateConfig {
        native_chain_config: None,
        protocol_chain_config: Some(new_config.clone()),
        protocol_fee_config: None,
        batch_period: None,
        monitors: None,
    };

    crate::contract::execute(
        deps.as_mut(),
        cosmwasm_std::testing::mock_env(),
        info.clone(),
        config_update_msg.clone(),
    )
    .unwrap();

    let config = CONFIG.load(&deps.storage).unwrap();
    assert_eq!(
        new_config.account_address_prefix,
        config.protocol_chain_config.account_address_prefix
    );
    assert_eq!(
        new_config.ibc_token_denom,
        config.protocol_chain_config.ibc_token_denom
    );
    assert_eq!(
        new_config.ibc_channel_id,
        config.protocol_chain_config.ibc_channel_id
    );
    assert_eq!(
        new_config.oracle_address.unwrap(),
        config
            .protocol_chain_config
            .oracle_address
            .unwrap()
            .to_string()
    );
    assert_eq!(
        new_config.minimum_liquid_stake_amount,
        config.protocol_chain_config.minimum_liquid_stake_amount
    );
}

#[test]
fn update_protocol_fee_config_with_invalid_treasury_address_fails() {
    let mut deps = init();
    let info = cosmwasm_std::testing::mock_info(OSMO3, &[]);
    let config_update_msg = crate::msg::ExecuteMsg::UpdateConfig {
        native_chain_config: None,
        protocol_chain_config: None,
        protocol_fee_config: Some(UnsafeProtocolFeeConfig {
            dao_treasury_fee: Uint128::from(10000u128),
            treasury_address: Some(CELESTIA1.to_string()),
        }),
        batch_period: None,
        monitors: None,
    };

    let res = crate::contract::execute(
        deps.as_mut(),
        cosmwasm_std::testing::mock_env(),
        info.clone(),
        config_update_msg,
    );
    assert!(res.is_err());
}

#[test]
fn update_protocol_fee_config_with_invalid_dao_treasury_fee_fails() {
    let mut deps = init();
    let info = cosmwasm_std::testing::mock_info(OSMO3, &[]);
    let config_update_msg = crate::msg::ExecuteMsg::UpdateConfig {
        native_chain_config: None,
        protocol_chain_config: None,
        protocol_fee_config: Some(UnsafeProtocolFeeConfig {
            dao_treasury_fee: Uint128::new(100_001),
            treasury_address: Some(OSMO3.to_string()),
        }),
        batch_period: None,
        monitors: None,
    };

    let res = crate::contract::execute(
        deps.as_mut(),
        cosmwasm_std::testing::mock_env(),
        info.clone(),
        config_update_msg,
    );
    assert!(res.is_err());
}

#[test]
fn update_protocol_fee_config_properly() {
    let mut deps = init();
    let info = cosmwasm_std::testing::mock_info(OSMO3, &[]);

    let new_config = UnsafeProtocolFeeConfig {
        dao_treasury_fee: Uint128::from(100000u128),
        treasury_address: Some(OSMO3.to_string()),
    };
    let config_update_msg = crate::msg::ExecuteMsg::UpdateConfig {
        native_chain_config: None,
        protocol_chain_config: None,
        protocol_fee_config: Some(new_config.clone()),
        batch_period: None,
        monitors: None,
    };

    crate::contract::execute(
        deps.as_mut(),
        cosmwasm_std::testing::mock_env(),
        info.clone(),
        config_update_msg,
    )
    .unwrap();

    let config = CONFIG.load(&deps.storage).unwrap();
    assert_eq!(
        new_config.dao_treasury_fee,
        config.protocol_fee_config.dao_treasury_fee
    );
    assert_eq!(
        new_config.treasury_address.unwrap(),
        config
            .protocol_fee_config
            .treasury_address
            .unwrap()
            .to_string()
    );
}

#[test]
fn update_batch_period_with_value_bigger_then_unbonding_period_fails() {
    let mut deps = init();
    let info = cosmwasm_std::testing::mock_info(OSMO3, &[]);
    let unbonding_period = CONFIG
        .load(&deps.storage)
        .unwrap()
        .native_chain_config
        .unbonding_period;

    let config_update_msg = crate::msg::ExecuteMsg::UpdateConfig {
        native_chain_config: None,
        protocol_chain_config: None,
        protocol_fee_config: None,
        batch_period: Some(unbonding_period + 1),
        monitors: None,
    };

    let result = crate::contract::execute(
        deps.as_mut(),
        cosmwasm_std::testing::mock_env(),
        info.clone(),
        config_update_msg,
    );

    assert!(result.is_err());
}

#[test]
fn update_monitors_with_invalid_address_fails() {
    let mut deps = init();
    let info = cosmwasm_std::testing::mock_info(OSMO3, &[]);
    let config_update_msg = crate::msg::ExecuteMsg::UpdateConfig {
        native_chain_config: None,
        protocol_chain_config: None,
        protocol_fee_config: None,
        batch_period: None,
        monitors: Some(vec![CELESTIA1.to_string()]),
    };

    let res = crate::contract::execute(
        deps.as_mut(),
        cosmwasm_std::testing::mock_env(),
        info.clone(),
        config_update_msg,
    );
    assert!(res.is_err());
}

#[test]
fn update_monitors_properly() {
    let mut deps = init();
    let info = cosmwasm_std::testing::mock_info(OSMO3, &[]);
    let config_update_msg = crate::msg::ExecuteMsg::UpdateConfig {
        native_chain_config: None,
        protocol_chain_config: None,
        protocol_fee_config: None,
        batch_period: None,
        monitors: Some(vec![OSMO1.to_string()]),
    };

    crate::contract::execute(
        deps.as_mut(),
        cosmwasm_std::testing::mock_env(),
        info.clone(),
        config_update_msg,
    )
    .unwrap();

    let config = CONFIG.load(&deps.storage).unwrap();
    assert_eq!(config.monitors, vec![Addr::unchecked(OSMO1)]);
}
