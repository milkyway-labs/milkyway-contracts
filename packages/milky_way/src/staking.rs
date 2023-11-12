use cosmwasm_std::{Addr, Uint128};
use schemars::{JsonSchema, Map};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub enum BatchStatus {
    Pending,
    Submitted,
    Received,
    Failed,
}
impl BatchStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            BatchStatus::Pending => "pending",
            BatchStatus::Submitted => "submitted",
            BatchStatus::Received => "received",
            BatchStatus::Failed => "failed",
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct Batch {
    /// ID of this batch
    pub id: u64,
    /// Total amount of `stTIA` to be burned in this batch
    pub batch_total_liquid_stake: Uint128,
    // The amount of native tokens that should be received after unbonding
    pub expected_native_unstaked: Option<Uint128>,

    pub liquid_unstake_requests: Map<String, LiquidUnstakeRequest>,

    /// Estimated time when next batch action occurs
    pub next_batch_action_time: Option<u64>,

    pub status: BatchStatus,
}
// Batch should always be constructed with a pending status
// Contract: Caller determines batch data
impl Batch {
    pub fn new(id: u64, batch_total: Uint128, est_next_batch_action: u64) -> Self {
        Self {
            id,
            batch_total_liquid_stake: batch_total,
            liquid_unstake_requests: Map::new(),
            next_batch_action_time: Some(est_next_batch_action),
            status: BatchStatus::Pending,
            expected_native_unstaked: None,
        }
    }
    pub fn update_status(&mut self, new_status: BatchStatus, next_action: Option<u64>) {
        // Defined by caller - env.block.time + batch period
        match new_status {
            BatchStatus::Pending => {
                self.status = new_status;
                self.next_batch_action_time = next_action;
            }
            // Defined by caller - env.block.time + unbonding period
            BatchStatus::Submitted => {
                self.status = new_status;
                self.next_batch_action_time = next_action;
            }
            BatchStatus::Received => {
                self.status = new_status;
                self.next_batch_action_time = None;
            }
            BatchStatus::Failed => {
                self.status = new_status;
                self.next_batch_action_time = None;
            }
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Eq, PartialEq, JsonSchema)]
pub struct LiquidUnstakeRequest {
    /// The user's address
    pub user: Addr,
    /// The user's share in the batch
    pub shares: Uint128,
    pub redeemed: bool,
}
impl LiquidUnstakeRequest {
    pub fn new(user: Addr, shares: Uint128) -> Self {
        Self {
            user,
            shares,
            redeemed: false,
        }
    }

    // Add to existing request for user
    pub fn update_request(&mut self, new_shares: Uint128) {
        self.shares += new_shares;
    }
}
