use crate::contract::instantiate;
use crate::msg::InstantiateMsg;
use crate::state::{Config, CONFIG};
use crate::types::{UnsafeNativeChainConfig, UnsafeProtocolChainConfig, UnsafeProtocolFeeConfig};

use cosmwasm_std::testing::{
    mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage,
};
use cosmwasm_std::{coins, OwnedDeps, Uint128};

pub static OSMO1: &str = "osmo12z558dm3ew6avgjdj07mfslx80rp9sh8nt7q3w";
pub static OSMO2: &str = "osmo13ftwm6z4dq6ugjvus2hf2vx3045ahfn3dq7dms";
pub static OSMO3: &str = "osmo1sfhy3emrgp26wnzuu64p06kpkxd9phel8ym0ge";
pub static OSMO4: &str = "osmo17x4zm0m0mxc428ykll3agmehfrxpr5hqpmsatd";
pub static STAKER_ADDRESS: &str = "celestia1sfhy3emrgp26wnzuu64p06kpkxd9phel74e0yx";
pub static CELESTIA1: &str = "celestia1fc25htmfvg28ygjckkhrxr7t73ek6zly8dshju";
pub static CELESTIA2: &str = "celestia1ztrhpdznu2xlwakd4yp3hg9lwyr3d46ayd30u2";
pub static CELESTIAVAL1: &str = "celestiavaloper1463wx5xkus5hyugyecvlhv9qpxklz62kyhwcts";
pub static CELESTIAVAL2: &str = "celestiavaloper1amxp3ah9anq4pmpnsknls7sql3kras9hs8pu0g";
pub static CELESTIAVAL3: &str = "celestiavaloper1t345w0vxnyyrf4eh43lpd3jl7z378rtsdn9tz3";
pub static CHANNEL_ID: &str = "channel-123";
pub static NATIVE_TOKEN: &str =
    "ibc/C3E53D20BC7A4CC993B17C7971F8ECD06A433C10B6A96F4C4C3714F0624C56DA";
pub static LIQUID_STAKE_TOKEN_DENOM: &str = "umilkTIA";

pub fn mock_init_msg() -> InstantiateMsg {
    InstantiateMsg {
        native_chain_config: UnsafeNativeChainConfig {
            token_denom: "utia".to_string(),
            account_address_prefix: "celestia".to_string(),
            validator_address_prefix: "celestiavaloper".to_string(),
            validators: vec![CELESTIAVAL1.to_string(), CELESTIAVAL2.to_string()],
            unbonding_period: 1209600,
            staker_address: STAKER_ADDRESS.to_string(),
            reward_collector_address: CELESTIA2.to_string(),
        },
        protocol_chain_config: UnsafeProtocolChainConfig {
            account_address_prefix: "osmo".to_string(),
            ibc_token_denom: NATIVE_TOKEN.to_string(),
            ibc_channel_id: CHANNEL_ID.to_string(),
            oracle_address: Some(OSMO4.to_string()),
            minimum_liquid_stake_amount: Uint128::from(100u128),
        },
        liquid_stake_token_denom: LIQUID_STAKE_TOKEN_DENOM.to_string(),
        monitors: vec![OSMO2.to_string(), OSMO3.to_string()],
        batch_period: 86400,
        protocol_fee_config: UnsafeProtocolFeeConfig {
            dao_treasury_fee: Uint128::from(10_000u128),
            treasury_address: Some(OSMO1.to_string()),
        },
    }
}

pub fn init() -> OwnedDeps<MockStorage, MockApi, MockQuerier> {
    let mut deps = mock_dependencies();
    let msg = mock_init_msg();
    let info = mock_info(OSMO3, &coins(1000, "uosmo"));

    let res = instantiate(deps.as_mut(), mock_env(), info, msg);
    if res.is_err() {
        panic!("error: {:?}", res);
    }
    assert!(res.is_ok());

    let mut config: Config = CONFIG.load(&deps.storage).unwrap();
    config.stopped = false;
    CONFIG.save(&mut deps.storage, &config).unwrap();

    deps
}
