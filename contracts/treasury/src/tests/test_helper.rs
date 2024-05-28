use crate::contract::instantiate;
use crate::msg::InstantiateMsg;
use crate::state::SwapRoute;

use cosmwasm_std::testing::{
    mock_dependencies, mock_env, mock_info, MockApi, MockQuerier, MockStorage,
};
use cosmwasm_std::{coins, OwnedDeps};

pub static ADMIN: &str = "osmo1sfhy3emrgp26wnzuu64p06kpkxd9phel8ym0ge";
pub static TRADER: &str = "osmo12z558dm3ew6avgjdj07mfslx80rp9sh8nt7q3w";
pub static TIA_USDC_POOL: u64 = 1;
pub static TIA_OSMO_POOL: u64 = 2;
pub static OSMO_USDC_POOL: u64 = 3;
pub static TIA_DENOM: &str = "utia";
pub static USDC_DENOM: &str = "uusdc";
pub static OSMO_DENOM: &str = "uosmo";

pub fn init() -> OwnedDeps<MockStorage, MockApi, MockQuerier> {
    let mut deps = mock_dependencies();
    let msg = InstantiateMsg {
        admin: Some(ADMIN.to_string()),
        trader: Some(TRADER.to_string()),
        allowed_swap_routes: vec![
            // TIA -> USDC
            vec![SwapRoute {
                pool_id: TIA_USDC_POOL,
                token_in_denom: TIA_DENOM.to_string(),
                token_out_denom: USDC_DENOM.to_string(),
            }],
            // TIA -> OSMO -> USDC
            vec![
                SwapRoute {
                    pool_id: TIA_OSMO_POOL,
                    token_in_denom: TIA_DENOM.to_string(),
                    token_out_denom: OSMO_DENOM.to_string(),
                },
                SwapRoute {
                    pool_id: OSMO_USDC_POOL,
                    token_in_denom: OSMO_DENOM.to_string(),
                    token_out_denom: USDC_DENOM.to_string(),
                },
            ],
        ],
    };
    let info = mock_info(ADMIN, &coins(1000, "uosmo"));

    let res = instantiate(deps.as_mut(), mock_env(), info, msg);
    if res.is_err() {
        panic!("error: {:?}", res);
    }
    assert!(res.is_ok());

    deps
}
