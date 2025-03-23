use cosmwasm_schema::cw_serde;
use cw_storage_plus::Map;

use crate::state::ibc::PacketLifecycleStatus;

#[cw_serde]
pub struct IbcWaitingForReply {
    pub amount: u128,
}

/// A transfer packet sent by this contract that is expected to be received but
/// needs to be tracked in case the receive fails or times-out
#[cw_serde]
pub struct IBCTransfer {
    pub sequence: u64,
    pub amount: u128,
    pub status: PacketLifecycleStatus,
}

/// In-Flight packets by (source_channel_id, sequence)
pub const INFLIGHT_PACKETS: Map<u64, IBCTransfer> = Map::new("inflight");
pub const IBC_WAITING_FOR_REPLY: Map<u64, IbcWaitingForReply> = Map::new("ibc_waiting_for_reply");
