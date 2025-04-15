use cosmwasm_std::{
    from_json, testing::MockQuerier, to_json_binary, CustomQuery, Empty, Querier, QuerierResult,
    QueryRequest, SystemError, SystemResult,
};
use osmosis_std::types::osmosis::poolmanager::v1beta1::{
    EstimateSwapExactAmountInRequest, EstimateSwapExactAmountInResponse,
};
use serde::de::DeserializeOwned;

use crate::error::ContractResult;

pub const INVALID_POOL_DENOM: &str = "INVALID";

pub struct OsmosisQuerierMock<C: DeserializeOwned = Empty> {
    pub querier: MockQuerier<C>,
}

impl Default for OsmosisQuerierMock {
    fn default() -> Self {
        OsmosisQuerierMock {
            querier: MockQuerier::default(),
        }
    }
}

impl<C: CustomQuery + DeserializeOwned> Querier for OsmosisQuerierMock<C> {
    fn raw_query(&self, bin_request: &[u8]) -> QuerierResult {
        let request: QueryRequest<C> = match from_json(bin_request) {
            Ok(v) => v,
            Err(e) => {
                return SystemResult::Err(SystemError::InvalidRequest {
                    error: format!("Parsing query request: {e}"),
                    request: bin_request.into(),
                })
            }
        };
        self.handle_query(&request)
    }
}

impl<C: CustomQuery + DeserializeOwned> OsmosisQuerierMock<C> {
    pub fn handle_query(&self, request: &QueryRequest<C>) -> QuerierResult {
        match &request {
            QueryRequest::Stargate { path, data } => match path.as_str() {
                "/osmosis.poolmanager.v1beta1.Query/EstimateSwapExactAmountIn" => {
                    let request = EstimateSwapExactAmountInRequest::try_from(data.clone()).unwrap();
                    if request.token_in == INVALID_POOL_DENOM {
                        return SystemResult::Err(SystemError::Unknown {});
                    }

                    let response = EstimateSwapExactAmountInResponse {
                        token_out_amount: "1000".to_string(),
                    };
                    SystemResult::Ok(ContractResult::Ok(to_json_binary(&response).unwrap()).into())
                }
                _ => SystemResult::Err(SystemError::UnsupportedRequest {
                    kind: path.to_string(),
                }),
            },
            _ => self.querier.handle_query(request),
        }
    }
}
