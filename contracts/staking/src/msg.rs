use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Coin;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    LiquidStake { coin: Coin },
    LiquidUnstake { coin: Coin },
    Claim {},
    AddValidator { new_validator: String },
    RemoveValidator { validator: String },
    TransferOwnership { new_owner: String },
    AcceptOwnership {},
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {}
