use std::collections::HashMap;

use cosmwasm_std::{to_binary, Binary, ContractResult, QuerierResult};
use osmosis_std::types::osmosis::{
    downtimedetector::v1beta1::{
        RecoveredSinceDowntimeOfLengthRequest, RecoveredSinceDowntimeOfLengthResponse,
    },
    poolmanager::v1beta1::{EstimateSwapExactAmountOutResponse, PoolResponse, SpotPriceResponse},
    twap::v1beta1::{ArithmeticTwapToNowResponse, GeometricTwapToNowResponse},
};

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
pub struct PriceKey {
    pub pool_id: u64,
    pub denom_in: String,
    pub denom_out: String,
}

#[derive(Clone, Default)]
pub struct OsmosisQuerier {
    pub pools: HashMap<u64, PoolResponse>,

    pub spot_prices: HashMap<PriceKey, SpotPriceResponse>,
    pub arithmetic_twap_prices: HashMap<PriceKey, ArithmeticTwapToNowResponse>,
    pub geometric_twap_prices: HashMap<PriceKey, GeometricTwapToNowResponse>,

    pub downtime_detector: HashMap<(i32, u64), RecoveredSinceDowntimeOfLengthResponse>,
}

impl OsmosisQuerier {
    pub fn handle_stargate_query(&self, path: &str, _data: &Binary) -> Result<QuerierResult, ()> {
        if path == "/osmosis.poolmanager.v1beta1.Query/EstimateSwapExactAmountOut" {
            let query_response = EstimateSwapExactAmountOutResponse {
                token_in_amount: "1000".to_string(),
            };
            let res: ContractResult<Binary> = to_binary(&query_response).into();
            let querier_res: QuerierResult = Ok(res).into();
            return Ok(querier_res);
        }

        if path == "/osmosis.downtimedetector.v1beta1.Query/RecoveredSinceDowntimeOfLength" {
            let query_response = RecoveredSinceDowntimeOfLengthRequest {
                downtime: 0,
                recovery: None,
            };
            let res: ContractResult<Binary> = to_binary(&query_response).into();
            let querier_res: QuerierResult = Ok(res).into();
            return Ok(querier_res);
        }

        Err(())
    }
}
