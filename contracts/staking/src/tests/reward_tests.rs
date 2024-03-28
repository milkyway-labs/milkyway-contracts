#[cfg(test)]
mod reward_tests {
    use crate::contract::{execute, IBC_TIMEOUT};
    use crate::helpers::derive_intermediate_sender;
    use crate::msg::ExecuteMsg;
    use crate::state::{CONFIG, STATE};
    use crate::tests::test_helper::{init, CELESTIA1, CHANNEL_ID, NATIVE_TOKEN};

    use cosmwasm_std::testing::{mock_env, mock_info};
    use cosmwasm_std::{Addr, CosmosMsg, ReplyOn, Uint128};
    use osmosis_std::types::ibc::applications::transfer::v1::MsgTransfer;

    #[test]
    fn receive_rewards() {
        let mut deps = init();
        let env = mock_env();

        let mut state = STATE.load(&deps.storage).unwrap();
        let config = CONFIG.load(&deps.storage).unwrap();

        state.total_liquid_stake_token = Uint128::from(100_000u128);
        state.total_native_token = Uint128::from(100_000u128);
        state.total_reward_amount = Uint128::from(0u128);
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
            &[cosmwasm_std::Coin {
                amount: Uint128::from(100u128),
                denom: "uosmo".to_string(),
            }],
        );
        let res = execute(deps.as_mut(), env.clone(), info, msg.clone());

        assert!(res.is_err()); // wrong denom

        let info = mock_info(
            &contract,
            &[cosmwasm_std::Coin {
                amount: Uint128::from(100u128),
                denom: config.native_token_denom.clone(),
            }],
        );
        let res = execute(deps.as_mut(), env.clone(), info, msg.clone());

        assert!(res.is_err()); // wrong sender

        let info = mock_info(
            &sender,
            &[cosmwasm_std::Coin {
                amount: Uint128::from(100u128),
                denom: config.native_token_denom.clone(),
            }],
        );
        let res = execute(deps.as_mut(), env.clone(), info, msg.clone());

        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(res.messages.len(), 4); // transfer message and redemption/purchase rate update
        assert_eq!(res.messages[3].reply_on, ReplyOn::Always);
        assert_eq!(
            res.messages[3].msg,
            CosmosMsg::from(MsgTransfer {
                source_channel: CHANNEL_ID.to_string(),
                source_port: "transfer".to_string(),
                sender: env.contract.address.to_string(),
                receiver: Addr::unchecked(CELESTIA1).to_string(),
                token: Some(osmosis_std::types::cosmos::base::v1beta1::Coin {
                    denom: NATIVE_TOKEN.to_string(),
                    amount: "90".to_string(),
                }),
                timeout_height: None,
                timeout_timestamp: env.block.time.nanos() + IBC_TIMEOUT.nanos(),
                memo: format!(
                    "{{\"ibc_callback\":\"{}\"}}",
                    env.contract.address.to_string()
                ),
            })
        );

        let state = STATE.load(&deps.storage).unwrap();

        assert_eq!(state.total_reward_amount, Uint128::from(100u128));
        assert_eq!(state.total_native_token, Uint128::from(100_090u128));
        assert_eq!(state.total_fees, Uint128::from(10u128));
    }
}
