use crate::{
    contract::CONTRACT_NAME,
    error::ContractResult,
    migrations::states::v1_0_0,
    state::{
        ibc::IBCTransfer, IbcWaitingForReply, CONFIG, IBC_WAITING_FOR_REPLY, INFLIGHT_PACKETS,
    },
};
use cosmwasm_std::{Coin, DepsMut, Env, Response};
use cw2::{assert_contract_version, set_contract_version};

const FROM_VERSION: &str = "1.0.0";
const TO_VERSION: &str = "1.1.0";

pub fn migrate(deps: DepsMut, _env: Env) -> ContractResult<Response> {
    // Ensure that we are migrating from the correct version.
    assert_contract_version(deps.storage, CONTRACT_NAME, FROM_VERSION)?;

    let config = CONFIG.load(deps.storage)?;

    // Migrate the inflight packets
    let inflight_packets = v1_0_0::INFLIGHT_PACKETS
        .range(deps.storage, None, None, cosmwasm_std::Order::Ascending)
        .collect::<Result<Vec<(u64, v1_0_0::IBCTransfer)>, _>>()?;
    for packet in inflight_packets {
        let (key, packet) = packet;
        INFLIGHT_PACKETS.save(
            deps.storage,
            key,
            &IBCTransfer {
                sequence: packet.sequence,
                amount: Coin::new(packet.amount, &config.protocol_chain_config.ibc_token_denom),
                receiver: config.native_chain_config.staker_address.to_string(),
                status: packet.status,
            },
        )?;
    }

    // Migrate the ibc messages waiting for replay
    let ibc_waiting_for_replay_packets = v1_0_0::IBC_WAITING_FOR_REPLY
        .range(deps.storage, None, None, cosmwasm_std::Order::Ascending)
        .collect::<Result<Vec<(u64, v1_0_0::IbcWaitingForReply)>, _>>()?;
    for packet in ibc_waiting_for_replay_packets {
        let (key, waiting_for_replay) = packet;
        IBC_WAITING_FOR_REPLY.save(
            deps.storage,
            key,
            &IbcWaitingForReply {
                amount: Coin::new(
                    waiting_for_replay.amount,
                    &config.protocol_chain_config.ibc_token_denom,
                ),
                receiver: config.native_chain_config.staker_address.to_string(),
            },
        )?;
    }

    // set new contract version
    set_contract_version(deps.storage, CONTRACT_NAME, TO_VERSION)?;

    Ok(Response::new()
        .add_attribute("action", "migrate")
        .add_attribute("from_version", FROM_VERSION)
        .add_attribute("to_version", TO_VERSION))
}
