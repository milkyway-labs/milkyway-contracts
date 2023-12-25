// Types for the Oracle contract
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Binary, Decimal};

pub const ORACLE_KEY: &str = "milkTIA_redemption_rate";

#[cw_serde]
pub enum Oracle {
    /// Uploads and stores a new metric
    PostMetric {
        /// Key identifying the metric (e.g. `stuatom_redemption_rate`)
        key: String,
        /// Value for the metric (e.g. `1.1`)
        value: String,
        /// Category for the metric(e.g. `redemption_rate`)
        /// Helps determine handling of additional context
        metric_type: MetricType,
        /// Unix timestamp with which the metric was updated on the source chain
        update_time: u64,
        /// Block height with which the metric was updated on the source chain
        block_height: u64,
        /// Additional metric-specific attributes
        attributes: Option<Binary>,
    },
}

/// The RedemptionRate struct represents the redemption rate of an stToken
#[cw_serde]
pub struct RedemptionRate {
    /// stToken denom as an IBC hash, as it appears on the oracle chain
    pub denom: String,
    /// The redemption rate of the stToken
    pub redemption_rate: Decimal,
    /// The unix timestamp representing when the redemption rate was last updated
    pub update_time: u64,
}

/// This contract represents a generic key value store
/// A "metric" is the term for a piece of information stored
/// Each metric has a higher level category that helps inform if any other,
/// metric-specific logic needs to be run
/// i.e. For redemption rates, there is an expected format for the attributes
/// field with additional metadata
#[cw_serde]
pub enum MetricType {
    RedemptionRate,
    Other(String),
}

/// For use in price oracles, the RedemptionRate metric requires the stToken denom
/// as it appears on the controller chain (e.g. `stuosmo`)
#[cw_serde]
pub struct RedemptionRateAttributes {
    pub sttoken_denom: String,
}
