#[cfg(test)]
mod tests {
    use crate::msg::InstantiateMsg;
    use crate::state::{Config, MultisigAddressConfig, ProtocolFeeConfig, BATCHES, CONFIG};
    use crate::tests::test_helper::{
        init, CELESTIA1, CELESTIA2, CHANNEL_ID, NATIVE_TOKEN, OSMO1, OSMO2, OSMO3,
    };

    use cosmwasm_std::{Addr, Order, Uint128};
    use milky_way::staking::BatchStatus;

    #[test]
    fn proper_instantiation() {
        let deps = init();

        let pending_batch = BATCHES
            .range(&deps.storage, None, None, Order::Descending)
            .find(|r| r.is_ok() && r.as_ref().unwrap().1.status == BatchStatus::Pending)
            .unwrap()
            .unwrap()
            .1;

        assert!(pending_batch.id == 1);
    }

    #[test]
    fn config_validation() {
        let mut deps = init();

        fn get_msg() -> InstantiateMsg {
            InstantiateMsg {
                native_token_denom: NATIVE_TOKEN.to_string(),
                liquid_stake_token_denom: "stTIA".to_string(),
                treasury_address: OSMO1.to_string(),
                monitors: vec![OSMO2.to_string(), OSMO3.to_string()],
                validators: vec![CELESTIA1.to_string(), CELESTIA2.to_string()],
                batch_period: 86400,
                unbonding_period: 1209600,
                protocol_fee_config: ProtocolFeeConfig {
                    dao_treasury_fee: Uint128::from(10u128),
                },
                multisig_address_config: MultisigAddressConfig {
                    staker_address: Addr::unchecked(CELESTIA1),
                    reward_collector_address: Addr::unchecked(CELESTIA2),
                },
                minimum_liquid_stake_amount: Uint128::from(100u128),
                ibc_channel_id: CHANNEL_ID.to_string(),
                oracle_address: None,
            }
        }

        let info = cosmwasm_std::testing::mock_info(OSMO3, &[]);

        let mut msg = get_msg();
        msg.native_token_denom = "".to_string();
        let res = crate::contract::instantiate(
            deps.as_mut(),
            cosmwasm_std::testing::mock_env(),
            info.clone(),
            msg,
        );
        assert!(res.is_err());

        let mut msg = get_msg();
        msg.ibc_channel_id = "".to_string();
        let res = crate::contract::instantiate(
            deps.as_mut(),
            cosmwasm_std::testing::mock_env(),
            info.clone(),
            msg,
        );
        assert!(res.is_err());

        let mut msg = get_msg();
        msg.treasury_address = "".to_string();
        let res = crate::contract::instantiate(
            deps.as_mut(),
            cosmwasm_std::testing::mock_env(),
            info.clone(),
            msg,
        );
        assert!(res.is_err());

        let mut msg = get_msg();
        msg.monitors[1] = "".to_string();
        let res = crate::contract::instantiate(
            deps.as_mut(),
            cosmwasm_std::testing::mock_env(),
            info.clone(),
            msg,
        );
        assert!(res.is_err());

        let mut msg = get_msg();
        msg.monitors[1] = CELESTIA1.to_string();
        let res = crate::contract::instantiate(
            deps.as_mut(),
            cosmwasm_std::testing::mock_env(),
            info.clone(),
            msg,
        );
        assert!(res.is_err());

        let mut msg = get_msg();
        msg.validators[1] = OSMO1.to_string();
        let res = crate::contract::instantiate(
            deps.as_mut(),
            cosmwasm_std::testing::mock_env(),
            info.clone(),
            msg,
        );
        assert!(res.is_err());

        let mut msg = get_msg();
        msg.multisig_address_config.staker_address = Addr::unchecked(OSMO1.to_string());
        let res = crate::contract::instantiate(
            deps.as_mut(),
            cosmwasm_std::testing::mock_env(),
            info.clone(),
            msg,
        );
        assert!(res.is_err());

        let mut msg = get_msg();
        msg.multisig_address_config.reward_collector_address = Addr::unchecked(OSMO1.to_string());
        let res = crate::contract::instantiate(
            deps.as_mut(),
            cosmwasm_std::testing::mock_env(),
            info.clone(),
            msg,
        );
        assert!(res.is_err());
    }

    #[test]
    fn update_config() {
        let mut deps = init();

        let info = cosmwasm_std::testing::mock_info(OSMO3, &[]);

        let config_update_msg = crate::msg::ExecuteMsg::UpdateConfig {
            batch_period: Some(86400),
            unbonding_period: Some(1209600),
            protocol_fee_config: Some(ProtocolFeeConfig {
                dao_treasury_fee: Uint128::from(10u128),
            }),
            multisig_address_config: Some(MultisigAddressConfig {
                staker_address: Addr::unchecked(CELESTIA1),
                reward_collector_address: Addr::unchecked(CELESTIA2),
            }),
            minimum_liquid_stake_amount: Some(Uint128::from(100u128)),
            native_token_denom: Some(
                "ibc/C3E53D20BC7A4CC993B17C7971F8ECD06A433C10B6A96F4C4C3714F0624C56DA".to_string(),
            ),
            channel_id: Some("channel-0".to_string()),
            monitors: Some(vec![OSMO3.to_string()]),
            treasury_address: Some(OSMO3.to_string()),
            oracle_address: None,
        };

        let res = crate::contract::execute(
            deps.as_mut(),
            cosmwasm_std::testing::mock_env(),
            info.clone(),
            config_update_msg,
        );
        assert!(res.is_ok());
        let config: Config = CONFIG.load(&deps.storage).unwrap();
        assert!(config.clone().monitors.unwrap().len() == 1);
        assert!(config.clone().monitors.unwrap().first().unwrap().to_string() == *OSMO3);
        assert!(config.treasury_address == OSMO3);

        let config_update_msg = crate::msg::ExecuteMsg::UpdateConfig {
            batch_period: Some(86400),
            unbonding_period: Some(1209600),
            protocol_fee_config: Some(ProtocolFeeConfig {
                dao_treasury_fee: Uint128::from(10u128),
            }),
            multisig_address_config: Some(MultisigAddressConfig {
                staker_address: Addr::unchecked(CELESTIA1),
                reward_collector_address: Addr::unchecked(CELESTIA2),
            }),
            minimum_liquid_stake_amount: Some(Uint128::from(100u128)),
            native_token_denom: Some("".to_string()),
            channel_id: Some("channel-0".to_string()),
            monitors: None,
            treasury_address: None,
            oracle_address: None,
        };
        let res = crate::contract::execute(
            deps.as_mut(),
            cosmwasm_std::testing::mock_env(),
            info.clone(),
            config_update_msg,
        );
        assert!(res.is_err());

        let config_update_msg = crate::msg::ExecuteMsg::UpdateConfig {
            batch_period: Some(86400),
            unbonding_period: Some(1209600),
            protocol_fee_config: Some(ProtocolFeeConfig {
                dao_treasury_fee: Uint128::from(10u128),
            }),
            multisig_address_config: Some(MultisigAddressConfig {
                staker_address: Addr::unchecked(CELESTIA1),
                reward_collector_address: Addr::unchecked(CELESTIA2),
            }),
            minimum_liquid_stake_amount: Some(Uint128::from(100u128)),
            native_token_denom: Some("ibc/abc".to_string()),
            channel_id: Some("".to_string()),
            monitors: None,
            treasury_address: None,
            oracle_address: None,
        };
        let res = crate::contract::execute(
            deps.as_mut(),
            cosmwasm_std::testing::mock_env(),
            info.clone(),
            config_update_msg,
        );
        assert!(res.is_err());

        let config_update_msg = crate::msg::ExecuteMsg::UpdateConfig {
            batch_period: Some(86400),
            unbonding_period: Some(1209600),
            protocol_fee_config: Some(ProtocolFeeConfig {
                dao_treasury_fee: Uint128::from(10u128),
            }),
            multisig_address_config: Some(MultisigAddressConfig {
                staker_address: Addr::unchecked(CELESTIA1),
                reward_collector_address: Addr::unchecked(CELESTIA2),
            }),
            minimum_liquid_stake_amount: Some(Uint128::from(100u128)),
            native_token_denom: Some("".to_string()),
            channel_id: Some("".to_string()),
            monitors: None,
            treasury_address: None,
            oracle_address: None,
        };
        let res = crate::contract::execute(
            deps.as_mut(),
            cosmwasm_std::testing::mock_env(),
            info.clone(),
            config_update_msg,
        );
        assert!(res.is_err());
    }
}
