use crate::contract::{instantiate, IBC_TIMEOUT};
use crate::msg::InstantiateMsg;
use crate::state::{FeatureFlags, IbcConfig, MultisigAddressConfig, ProtocolFeeConfig, IBC_CONFIG};

use cosmwasm_std::testing::{
    mock_env, mock_info, MockApi, MockQuerier, MockStorage, MOCK_CONTRACT_ADDR,
};
use cosmwasm_std::{coins, Addr, Coin, OwnedDeps, Uint128};

use super::milkyway_mock_querier::MilkywayMockQuerier;

pub static OSMO1: &str = "osmo12z558dm3ew6avgjdj07mfslx80rp9sh8nt7q3w";
pub static OSMO2: &str = "osmo13ftwm6z4dq6ugjvus2hf2vx3045ahfn3dq7dms";
pub static OSMO3: &str = "osmo1sfhy3emrgp26wnzuu64p06kpkxd9phel8ym0ge";
pub static CELESTIA1: &str = "celestia1sfhy3emrgp26wnzuu64p06kpkxd9phel74e0yx";
pub static CELESTIA2: &str = "celestia1ztrhpdznu2xlwakd4yp3hg9lwyr3d46ayd30u2";
pub static CELESTIAVAL1: &str = "celestiavaloper1463wx5xkus5hyugyecvlhv9qpxklz62kyhwcts";
pub static CELESTIAVAL2: &str = "celestiavaloper1amxp3ah9anq4pmpnsknls7sql3kras9hs8pu0g";
pub static CELESTIAVAL3: &str = "celestiavaloper1t345w0vxnyyrf4eh43lpd3jl7z378rtsdn9tz3";
pub static CHANNEL_ID: &str = "channel-123";
pub static NATIVE_TOKEN: &str = "osmoTIA";

/// mock_dependencies replacement for cosmwasm_std::testing::mock_dependencies
pub fn mock_dependencies(
    contract_balance: &[Coin],
) -> OwnedDeps<MockStorage, MockApi, MilkywayMockQuerier> {
    let contract_addr = Addr::unchecked(MOCK_CONTRACT_ADDR);
    let custom_querier: MilkywayMockQuerier = MilkywayMockQuerier::new(MockQuerier::new(&[(
        contract_addr.as_ref(),
        contract_balance,
    )]));

    OwnedDeps {
        storage: MockStorage::default(),
        api: MockApi::default(),
        querier: custom_querier,
        custom_query_type: Default::default(),
    }
}

pub fn init() -> OwnedDeps<MockStorage, MockApi, MilkywayMockQuerier> {
    let mut deps = mock_dependencies(&vec![]);
    let msg = InstantiateMsg {
        native_token_denom: NATIVE_TOKEN.to_string(),
        liquid_stake_token_denom: "stTIA".to_string(),
        treasury_address: OSMO1.to_string(),
        operators: vec![OSMO2.to_string(), OSMO3.to_string()],
        validators: vec![CELESTIAVAL1.to_string(), CELESTIAVAL2.to_string()],
        batch_period: 86400,
        unbonding_period: 1209600,
        protocol_fee_config: ProtocolFeeConfig {
            dao_treasury_fee: Uint128::from(10000u128),
        },
        multisig_address_config: MultisigAddressConfig {
            staker_address: Addr::unchecked(CELESTIA1),
            reward_collector_address: Addr::unchecked(CELESTIA2),
        },
        minimum_liquid_stake_amount: Uint128::from(100u128),
        ibc_channel_id: CHANNEL_ID.to_string(),
        pool_id: 1,
        feature_flags: FeatureFlags {
            enable_auto_claim: true,
        },
    };
    let info = mock_info("creator", &coins(1000, "uosmo"));

    let res = instantiate(deps.as_mut(), mock_env(), info, msg);

    assert!(res.is_ok());

    let ibc_config = IbcConfig {
        channel_id: "channel-123".to_string(),
        default_timeout: IBC_TIMEOUT,
    };
    IBC_CONFIG.save(&mut deps.storage, &ibc_config).unwrap();

    deps
}
