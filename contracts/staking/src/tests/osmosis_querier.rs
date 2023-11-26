use std::collections::HashMap;

use cosmwasm_std::{to_binary, Binary, ContractResult, QuerierResult};
use osmosis_std::types::osmosis::{
    downtimedetector::v1beta1::{
        RecoveredSinceDowntimeOfLengthRequest, RecoveredSinceDowntimeOfLengthResponse,
    },
    poolmanager::v1beta1::{PoolResponse, SpotPriceResponse},
    twap::v1beta1::{
        ArithmeticTwapToNowRequest, ArithmeticTwapToNowResponse,
        GeometricTwapToNowResponse,
    },
};
use prost::{DecodeError};



use super::test_helper::RESERVE_TOKEN;

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
        if path == "/osmosis.twap.v1beta1.Query/ArithmeticTwapToNow" {
            // let parse_osmosis_query: Result<ArithmeticTwapToNowRequest, DecodeError> =
            //     Message::decode(data.as_slice());
            let parse_osmosis_query: Result<ArithmeticTwapToNowRequest, DecodeError> =
                Result::Ok(ArithmeticTwapToNowRequest {
                    pool_id: 1,
                    base_asset: "uosmo".to_string(),
                    quote_asset: RESERVE_TOKEN.to_string(),
                    start_time: Some(osmosis_std::shim::Timestamp {
                        seconds: 1_571_797_419 - 3600,
                        nanos: 879_305_533,
                    }),
                });
            if let Ok(osmosis_query) = parse_osmosis_query {
                return Ok(self.handle_query_arithmetic_twap_request(osmosis_query));
            }
        }

        if path == "/osmosis.downtimedetector.v1beta1.Query/RecoveredSinceDowntimeOfLength" {
            let parse_osmosis_query: Result<RecoveredSinceDowntimeOfLengthRequest, DecodeError> =
                Result::Ok(RecoveredSinceDowntimeOfLengthRequest {
                    downtime: 0,
                    recovery: None,
                });
            if let Ok(osmosis_query) = parse_osmosis_query {
                return Ok(self.handle_recovered_since_downtime_of_length(osmosis_query));
            }
        }

        Err(())
    }

    fn handle_query_arithmetic_twap_request(
        &self,
        _request: ArithmeticTwapToNowRequest,
    ) -> QuerierResult {
        let query_response = ArithmeticTwapToNowResponse {
            arithmetic_twap: "0.1".to_string(),
        };
        let res: ContractResult<Binary> = to_binary(&query_response).into();
        Ok(res).into()
    }

    fn handle_recovered_since_downtime_of_length(
        &self,
        _request: RecoveredSinceDowntimeOfLengthRequest,
    ) -> QuerierResult {
        let query_response = RecoveredSinceDowntimeOfLengthResponse {
            succesfully_recovered: true,
        };
        let res: ContractResult<Binary> = to_binary(&query_response).into();
        Ok(res).into()
    }
}
