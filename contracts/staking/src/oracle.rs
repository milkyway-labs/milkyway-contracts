// Types for the Oracle contract
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Binary, Decimal};

pub const ORACLE_REDEMPTION_RATE_KEY: &str = "milkTIA_redemption_rate";
pub const ORACLE_PURCHASE_RATE_KEY: &str = "milkTIA_purchase_rate";

#[cw_serde]
pub enum Oracle {
    PostMetric {
        key: String,
        value: String,
        metric_type: MetricType,
        update_time: u64,
        block_height: u64,
        attributes: Option<Binary>,
    },
    PostRates {
        denom: String,
        purchase_rate: String,
        redemption_rate: String,
    },
}

#[cw_serde]
pub enum MetricType {
    RedemptionRate,
    PurchaseRate,
    Other(String),
}

#[cw_serde]
pub struct RedemptionRateAttributes {
    pub sttoken_denom: String,
}

#[cw_serde]
pub struct PurchaseRateAttributes {
    pub sttoken_denom: String,
}
