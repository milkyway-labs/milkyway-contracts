use cosmwasm_std::{
    testing::{mock_dependencies, mock_env},
    Addr,
};
use cw2::set_contract_version;

use crate::{
    contract::{migrate, CONTRACT_NAME},
    migrations::states::v0_4_20,
    msg::MigrateMsg,
    state::CONFIG,
    tests::test_helper::TRADER,
    types::UnsafeProtocolChainConfig,
};

#[test]
fn invalid_protocol_chain_prefix_fails() {
    let mut deps = mock_dependencies();
    set_contract_version(
        &mut deps.storage,
        CONTRACT_NAME,
        crate::migrations::v1_0_0::FROM_VERSION,
    )
    .unwrap();

    // Save the configuration wit the old format
    v0_4_20::CONFIG
        .save(
            &mut deps.storage,
            &v0_4_20::Config {
                trader: Addr::unchecked(TRADER),
                allowed_swap_routes: vec![],
            },
        )
        .unwrap();

    migrate(
        deps.as_mut(),
        mock_env(),
        MigrateMsg::V0_4_20ToV1_0_0 {
            protocol_chain_config: UnsafeProtocolChainConfig {
                account_address_prefix: "osmosis".to_string(),
            },
            native_chain_config: crate::types::UnsafeNativeChainConfig {
                account_address_prefix: "celestia".to_string(),
            },
        },
    )
    .unwrap_err();
}

#[test]
fn migrate_successfully() {
    let mut deps = mock_dependencies();
    set_contract_version(
        &mut deps.storage,
        CONTRACT_NAME,
        crate::migrations::v1_0_0::FROM_VERSION,
    )
    .unwrap();

    // Save the configuration wit the old format
    v0_4_20::CONFIG
        .save(
            &mut deps.storage,
            &v0_4_20::Config {
                trader: Addr::unchecked(TRADER),
                allowed_swap_routes: vec![],
            },
        )
        .unwrap();

    migrate(
        deps.as_mut(),
        mock_env(),
        MigrateMsg::V0_4_20ToV1_0_0 {
            protocol_chain_config: UnsafeProtocolChainConfig {
                account_address_prefix: "osmo".to_string(),
            },
            native_chain_config: crate::types::UnsafeNativeChainConfig {
                account_address_prefix: "celestia".to_string(),
            },
        },
    )
    .unwrap();

    let config = CONFIG.load(&deps.storage).unwrap();
    assert_eq!("osmo", config.protocol_chain_config.account_address_prefix);
    assert_eq!(
        "celestia",
        config.native_chain_config.account_address_prefix
    );
}
