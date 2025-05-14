// Types for the Oracle contract
use cosmwasm_schema::cw_serde;

#[cw_serde]
pub enum Oracle {
    PostRates {
        denom: String,
        purchase_rate: String,
        redemption_rate: String,
    },
}

#[cw_serde]
pub struct OracleInstantiateMsg {
    pub admin_address: String,
}
