use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    TransferOwnership {
        new_owner: String,
    },
    AcceptOwnership {},
    RevokeOwnershipTransfer {},
    SpendFunds {
        amount: cosmwasm_std::Coin,
        receiver: String,
        channel_id: Option<String>,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {}
