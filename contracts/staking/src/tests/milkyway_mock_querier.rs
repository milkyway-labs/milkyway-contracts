use cosmwasm_std::{
    from_slice, testing::MockQuerier, Addr, Empty, Querier, QuerierResult, QueryRequest,
    SystemError, SystemResult, WasmQuery,
};

use super::osmosis_querier::OsmosisQuerier;

pub struct MilkywayMockQuerier {
    base: MockQuerier<Empty>,
    osmosis_querier: OsmosisQuerier,
}

impl Querier for MilkywayMockQuerier {
    fn raw_query(&self, bin_request: &[u8]) -> QuerierResult {
        let request: QueryRequest<Empty> = match from_slice(bin_request) {
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

impl MilkywayMockQuerier {
    pub fn new(base: MockQuerier<Empty>) -> Self {
        MilkywayMockQuerier {
            base,
            osmosis_querier: OsmosisQuerier::default(),
        }
    }

    pub fn handle_query(&self, request: &QueryRequest<Empty>) -> QuerierResult {
        match &request {
            QueryRequest::Wasm(WasmQuery::Smart { contract_addr, msg }) => {
                let _contract_addr = Addr::unchecked(contract_addr);

                panic!("[mock]: Unsupported wasm query: {msg:?}");
            }

            QueryRequest::Stargate { path, data } => {
                if let Ok(querier_res) = self.osmosis_querier.handle_stargate_query(path, data) {
                    return querier_res;
                }

                panic!("[mock]: Unsupported stargate query, path: {path:?}");
            }

            _ => self.base.handle_query(request),
        }
    }
}
