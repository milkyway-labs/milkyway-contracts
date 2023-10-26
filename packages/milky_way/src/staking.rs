use cosmwasm_std::{Addr, Uint128};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct Batch {
    /// ID of this batch
    pub id: u64,
    /// Total amount of `stTIA` to be burned in this batch
    pub batch_total: Uint128,
    /// Estimated time when this batch will be submitted for unbonding
    pub est_unbond_start_time: u64,

    pub liquid_unstake_requests: Vec<LiquidUnstakeRequest>,

    /// Estimated time when this batch will finish unbonding
    pub est_unbond_end_time: Option<Uint128>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct LiquidUnstakeRequest {
    /// ID of the batch this request added to
    pub id: u64,
    /// The user's address
    pub user: Addr,
    /// The user's share in the batch
    pub shares: Uint128,
}
