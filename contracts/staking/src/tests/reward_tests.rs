use crate::contract::{execute, IBC_TIMEOUT};
use crate::helpers::derive_intermediate_sender;
use crate::msg::ExecuteMsg;
use crate::state::{CONFIG, STATE};
use crate::tests::test_helper::{init, CHANNEL_ID, NATIVE_TOKEN, STAKER_ADDRESS};

use cosmwasm_std::testing::{mock_env, mock_info};
use cosmwasm_std::{Addr, CosmosMsg, ReplyOn, Uint128};
use osmosis_std::types::ibc::applications::transfer::v1::MsgTransfer;

#[test]
fn receive_rewards() {
    let mut deps = init();
    let env = mock_env();

    let mut state = STATE.load(&deps.storage).unwrap();
    let mut config = CONFIG.load(&deps.storage).unwrap();

    state.total_liquid_stake_token = Uint128::from(100_000u128);
    state.total_native_token = Uint128::from(100_000u128);
    state.total_reward_amount = Uint128::from(0u128);
    STATE.save(&mut deps.storage, &state).unwrap();

    let msg = ExecuteMsg::ReceiveRewards {};

    let contract = env.contract.address.clone().to_string();

    let sender = derive_intermediate_sender(
        &config.protocol_chain_config.ibc_channel_id,
        config.native_chain_config.reward_collector_address.as_str(),
        config.protocol_chain_config.account_address_prefix.as_str(),
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
            denom: config.protocol_chain_config.ibc_token_denom.clone(),
        }],
    );
    let res = execute(deps.as_mut(), env.clone(), info, msg.clone());

    assert!(res.is_err()); // wrong sender

    // Test without send fees to treasury
    config.protocol_fee_config.treasury_address = None;
    CONFIG.save(&mut deps.storage, &config).unwrap();

    let info = mock_info(
        &sender,
        &[cosmwasm_std::Coin {
            amount: Uint128::from(100u128),
            denom: config.protocol_chain_config.ibc_token_denom.clone(),
        }],
    );
    let res = execute(deps.as_mut(), env.clone(), info, msg.clone()).unwrap();

    assert_eq!(res.messages.len(), 2); // transfer message and redemption/purchase rate update
    assert_eq!(res.messages[1].reply_on, ReplyOn::Always);
    assert_eq!(
        res.messages[1].msg,
        CosmosMsg::from(MsgTransfer {
            source_channel: CHANNEL_ID.to_string(),
            source_port: "transfer".to_string(),
            sender: env.contract.address.to_string(),
            receiver: Addr::unchecked(STAKER_ADDRESS).to_string(),
            token: Some(osmosis_std::types::cosmos::base::v1beta1::Coin {
                denom: NATIVE_TOKEN.to_string(),
                amount: "90".to_string(),
            }),
            timeout_height: None,
            timeout_timestamp: env.block.time.nanos() + IBC_TIMEOUT.nanos(),
            memo: format!("{{\"ibc_callback\":\"{}\"}}", env.contract.address),
        })
    );

    let state = STATE.load(&deps.storage).unwrap();

    assert_eq!(state.total_reward_amount, Uint128::from(100u128));
    assert_eq!(state.total_native_token, Uint128::from(100_090u128));
    assert_eq!(state.total_fees, Uint128::from(10u128));
}

#[test]
fn receive_rewards_and_send_fees_to_treasury() {
    let mut deps = init();
    let env = mock_env();

    let mut state = STATE.load(&deps.storage).unwrap();
    let config = CONFIG.load(&deps.storage).unwrap();

    state.total_liquid_stake_token = Uint128::from(100_000u128);
    state.total_native_token = Uint128::from(100_000u128);
    state.total_reward_amount = Uint128::from(0u128);
    STATE.save(&mut deps.storage, &state).unwrap();

    let msg = ExecuteMsg::ReceiveRewards {};

    let sender = derive_intermediate_sender(
        &config.protocol_chain_config.ibc_channel_id,
        config.native_chain_config.reward_collector_address.as_str(),
        config.protocol_chain_config.account_address_prefix.as_str(),
    )
    .unwrap();

    let info = mock_info(
        &sender,
        &[cosmwasm_std::Coin {
            amount: Uint128::from(100u128),
            denom: config.protocol_chain_config.ibc_token_denom.clone(),
        }],
    );
    let res = execute(deps.as_mut(), env.clone(), info, msg.clone()).unwrap();
    assert_eq!(res.messages.len(), 3); // transfer message, redemption/purchase rate update and
                                       // send message to treasury
    assert_eq!(res.messages[1].reply_on, ReplyOn::Always);
    assert_eq!(
        res.messages[1].msg,
        CosmosMsg::from(MsgTransfer {
            source_channel: CHANNEL_ID.to_string(),
            source_port: "transfer".to_string(),
            sender: env.contract.address.to_string(),
            receiver: Addr::unchecked(STAKER_ADDRESS).to_string(),
            token: Some(osmosis_std::types::cosmos::base::v1beta1::Coin {
                denom: NATIVE_TOKEN.to_string(),
                amount: "90".to_string(),
            }),
            timeout_height: None,
            timeout_timestamp: env.block.time.nanos() + IBC_TIMEOUT.nanos(),
            memo: format!("{{\"ibc_callback\":\"{}\"}}", env.contract.address),
        })
    );
    assert_eq!(
        res.messages[2].msg,
        CosmosMsg::from(cosmwasm_std::BankMsg::Send {
            to_address: config
                .protocol_fee_config
                .treasury_address
                .unwrap()
                .to_string(),
            amount: vec![cosmwasm_std::Coin::new(10u128, NATIVE_TOKEN)],
        })
    );

    let state = STATE.load(&deps.storage).unwrap();

    assert_eq!(state.total_reward_amount, Uint128::from(100u128));
    assert_eq!(state.total_native_token, Uint128::from(100_090u128));
    assert_eq!(state.total_fees, Uint128::from(0u128));
}
