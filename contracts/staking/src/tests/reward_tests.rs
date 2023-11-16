#[cfg(test)]
mod reward_tests {
    use crate::contract::execute;
    use crate::helpers::derive_intermediate_sender;
    use crate::msg::ExecuteMsg;
    use crate::state::{CONFIG, STATE};
    use crate::tests::test_helper::init;

    use cosmwasm_std::testing::{mock_env, mock_info};
    use cosmwasm_std::{Coin, Uint128};

    #[test]
    fn receive_rewards() {
        let mut deps = init();
        let env = mock_env();

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
}
