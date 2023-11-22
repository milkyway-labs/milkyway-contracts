use cosmwasm_schema::cw_serde;
use cosmwasm_std::{to_binary, Addr, Binary};
use enum_repr::EnumRepr;
/// IBC ACK. See:
/// https://github.com/cosmos/cosmos-sdk/blob/f999b1ff05a4db4a338a855713864497bedd4396/proto/ibc/core/channel/v1/channel.proto#L141-L147
#[cw_serde]
pub enum Ack {
    Result(Binary),
    Error(String),
}

pub fn make_ack_success() -> Binary {
    let res = Ack::Result(b"1".into());
    to_binary(&res).unwrap()
}

pub fn make_ack_fail(err: String) -> Binary {
    let res = Ack::Error(err);
    to_binary(&res).unwrap()
}

// reply id
#[EnumRepr(type = "u64")]
pub enum ReplyId {
    IbcTransfer = 1,
}

// We define the response as a prost message to be able to decode the protobuf data.
#[derive(Clone, PartialEq, Eq, ::prost::Message)]
pub struct MsgTransferResponse {
    #[prost(uint64, tag = "1")]
    pub sequence: u64,
}

#[cw_serde]
pub enum FailedDeliveryAction {
    DoNothing,
    LocalRecoveryAddr(Addr),
}
