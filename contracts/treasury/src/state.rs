use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cosmwasm_std::Timestamp;
use cw_controllers::Admin;
use cw_storage_plus::Item;

#[cw_serde]
pub struct State {
    pub pending_owner: Option<Addr>,
    pub owner_transfer_min_time: Option<Timestamp>,
}

pub const ADMIN: Admin = Admin::new("admin");
pub const STATE: Item<State> = Item::new("state");
