use cosmwasm_std::testing::{mock_dependencies, mock_env};
use cosmwasm_std::{Addr, Coin, Uint128};
use cw2::set_contract_version;

use crate::contract::{migrate, CONTRACT_NAME};
use crate::migrations::states::v1_0_0;
use crate::msg::MigrateMsg;
use crate::state::ibc::{IBCTransfer, PacketLifecycleStatus};
use crate::state::{
    Config, IbcWaitingForReply, NativeChainConfig, ProtocolChainConfig, ProtocolFeeConfig, CONFIG,
    IBC_WAITING_FOR_REPLY, INFLIGHT_PACKETS, MIGRATING,
};
use crate::tests::test_helper::{
    CELESTIA2, CHANNEL_ID, LIQUID_STAKE_TOKEN_DENOM, NATIVE_TOKEN, OSMO1, OSMO4, STAKER_ADDRESS,
};

#[test]
fn migrate_from_wrong_version_fails() {
    let mut deps = mock_dependencies();

    // Set wrong contract version
    set_contract_version(deps.as_mut().storage, CONTRACT_NAME, "0.4.20").unwrap();

    // Perform the migration
    let result = migrate(
        deps.as_mut(),
        mock_env(),
        MigrateMsg::V1_0_0ToV1_1_0 { limit: None },
    );
    assert!(result.is_err());
}

#[test]
fn paginated_migration() {
    let mut deps = mock_dependencies();

    // Set the contract version to 1.0.0
    set_contract_version(deps.as_mut().storage, CONTRACT_NAME, "1.0.0").unwrap();
    CONFIG
        .save(
            deps.as_mut().storage,
            &Config {
                native_chain_config: NativeChainConfig {
                    token_denom: "utia".to_string(),
                    account_address_prefix: "celestia".to_string(),
                    validator_address_prefix: "celestiavaloper".to_string(),
                    validators: vec![],
                    unbonding_period: 1209600,
                    staker_address: Addr::unchecked(STAKER_ADDRESS),
                    reward_collector_address: Addr::unchecked(CELESTIA2),
                },
                protocol_chain_config: ProtocolChainConfig {
                    account_address_prefix: "osmo".to_string(),
                    ibc_token_denom: NATIVE_TOKEN.to_string(),
                    ibc_channel_id: CHANNEL_ID.to_string(),
                    oracle_address: Some(Addr::unchecked(OSMO4)),
                    minimum_liquid_stake_amount: Uint128::from(100u128),
                },
                liquid_stake_token_denom: LIQUID_STAKE_TOKEN_DENOM.to_string(),
                monitors: vec![],
                batch_period: 86400,
                protocol_fee_config: ProtocolFeeConfig {
                    dao_treasury_fee: Uint128::from(10000u128),
                    treasury_address: Some(Addr::unchecked(OSMO1)),
                },
                stopped: false,
            },
        )
        .unwrap();

    // Store the data that will be subject to the migration
    for i in 1..=6 {
        v1_0_0::IBC_WAITING_FOR_REPLY
            .save(
                deps.as_mut().storage,
                i,
                &v1_0_0::IbcWaitingForReply {
                    amount: 1000 + u128::from(i),
                },
            )
            .unwrap();
    }

    for i in 1..=6 {
        v1_0_0::INFLIGHT_PACKETS
            .save(
                deps.as_mut().storage,
                i,
                &v1_0_0::IBCTransfer {
                    sequence: i,
                    amount: 1000 + u128::from(i),
                    status: PacketLifecycleStatus::Sent,
                },
            )
            .unwrap();
    }

    // Perform the migration in multiple steps
    migrate(
        deps.as_mut(),
        mock_env(),
        MigrateMsg::V1_0_0ToV1_1_0 { limit: Some(3) },
    )
    .unwrap();
    assert_eq!(true, MIGRATING.load(&mut deps.storage).unwrap());

    migrate(
        deps.as_mut(),
        mock_env(),
        MigrateMsg::V1_0_0ToV1_1_0 { limit: Some(6) },
    )
    .unwrap();
    assert_eq!(true, MIGRATING.load(&mut deps.storage).unwrap());

    migrate(
        deps.as_mut(),
        mock_env(),
        MigrateMsg::V1_0_0ToV1_1_0 { limit: Some(3) },
    )
    .unwrap();
    assert_eq!(false, MIGRATING.load(&mut deps.storage).unwrap());

    // Ensure the data have been migrated correctly
    for i in 1..=6 {
        let packet = IBC_WAITING_FOR_REPLY
            .load(deps.as_mut().storage, i)
            .unwrap();
        assert_eq!(
            IbcWaitingForReply {
                receiver: STAKER_ADDRESS.to_string(),
                amount: Coin::new(1000 + u128::from(i), NATIVE_TOKEN),
            },
            packet,
        );
    }

    for i in 1..=6 {
        let packet = INFLIGHT_PACKETS.load(deps.as_mut().storage, i).unwrap();
        assert_eq!(
            IBCTransfer {
                sequence: i,
                receiver: STAKER_ADDRESS.to_string(),
                amount: Coin::new(1000 + u128::from(i), NATIVE_TOKEN),
                status: PacketLifecycleStatus::Sent,
            },
            packet,
        );
    }
}

#[test]
fn migrate_all_packets() {
    let mut deps = mock_dependencies();

    // Set the contract version to 1.0.0
    set_contract_version(deps.as_mut().storage, CONTRACT_NAME, "1.0.0").unwrap();
    CONFIG
        .save(
            deps.as_mut().storage,
            &Config {
                native_chain_config: NativeChainConfig {
                    token_denom: "utia".to_string(),
                    account_address_prefix: "celestia".to_string(),
                    validator_address_prefix: "celestiavaloper".to_string(),
                    validators: vec![],
                    unbonding_period: 1209600,
                    staker_address: Addr::unchecked(STAKER_ADDRESS),
                    reward_collector_address: Addr::unchecked(CELESTIA2),
                },
                protocol_chain_config: ProtocolChainConfig {
                    account_address_prefix: "osmo".to_string(),
                    ibc_token_denom: NATIVE_TOKEN.to_string(),
                    ibc_channel_id: CHANNEL_ID.to_string(),
                    oracle_address: Some(Addr::unchecked(OSMO4)),
                    minimum_liquid_stake_amount: Uint128::from(100u128),
                },
                liquid_stake_token_denom: LIQUID_STAKE_TOKEN_DENOM.to_string(),
                monitors: vec![],
                batch_period: 86400,
                protocol_fee_config: ProtocolFeeConfig {
                    dao_treasury_fee: Uint128::from(10000u128),
                    treasury_address: Some(Addr::unchecked(OSMO1)),
                },
                stopped: false,
            },
        )
        .unwrap();

    // Store the data that will be subject to the migration
    for i in 1..=6 {
        v1_0_0::IBC_WAITING_FOR_REPLY
            .save(
                deps.as_mut().storage,
                i,
                &v1_0_0::IbcWaitingForReply {
                    amount: 1000 + u128::from(i),
                },
            )
            .unwrap();
    }

    for i in 1..=6 {
        v1_0_0::INFLIGHT_PACKETS
            .save(
                deps.as_mut().storage,
                i,
                &v1_0_0::IBCTransfer {
                    sequence: i,
                    amount: 1000 + u128::from(i),
                    status: PacketLifecycleStatus::Sent,
                },
            )
            .unwrap();
    }

    // Perform the migration
    migrate(
        deps.as_mut(),
        mock_env(),
        MigrateMsg::V1_0_0ToV1_1_0 { limit: None },
    )
    .unwrap();

    // Ensure the data have been migrated correctly
    for i in 1..=6 {
        let packet = IBC_WAITING_FOR_REPLY
            .load(deps.as_mut().storage, i)
            .unwrap();
        assert_eq!(
            IbcWaitingForReply {
                receiver: STAKER_ADDRESS.to_string(),
                amount: Coin::new(1000 + u128::from(i), NATIVE_TOKEN),
            },
            packet,
        );
    }

    for i in 1..=6 {
        let packet = INFLIGHT_PACKETS.load(deps.as_mut().storage, i).unwrap();
        assert_eq!(
            IBCTransfer {
                sequence: i,
                receiver: STAKER_ADDRESS.to_string(),
                amount: Coin::new(1000 + u128::from(i), NATIVE_TOKEN),
                status: PacketLifecycleStatus::Sent,
            },
            packet,
        );
    }

    assert_eq!(false, MIGRATING.load(&mut deps.storage).unwrap())
}
