use cosmwasm_std::{DepsMut, Response};

use crate::state::{self, Config, CONFIG};
use crate::{error::ContractError, state::INFLIGHT_PACKETS};

/// Called by the chain when the ack for a packet that has configured this contract as its
/// callback has been received.
///
/// The chain needs to verify that the ack is valid ack for the packet with  the matching
/// source_channel and sequence before calling this function.
///
/// If the contract didn't send the IBC packet with (source_channel, sequence), we return a
/// success and no other changes are made.
///
/// If this contract sent the IBC packet, its data will be stored in
/// INFLIGHT_PACKETS. At this point the ack can be a success or a failure.
///
/// If it's a success, we remove the inflight packet and return. The packet will
/// no longer be tracked.
///
/// If it's a failure, the sent funds will have been returned to this contract.
/// We then store the amount and original sender on RECOVERY_STATES so that the
/// sender can recover the funds by calling execute::Recover{}.
pub fn receive_ack(
    deps: DepsMut,
    source_channel: String,
    sequence: u64,
    _ack: String,
    success: bool,
) -> Result<Response, ContractError> {
    // deps.api.debug(&format!(
    //     "received ack for packet {source_channel:?} {sequence:?}: {ack:?}, {success:?}"
    // ));

    let config: Config = CONFIG.load(deps.storage)?;
    if source_channel != config.ibc_channel_id {
        // If the ack is not for this contract, return a success
        return Ok(Response::new()
            .add_attribute("action", "receive_ack")
            .add_attribute("error", "received ack for different channel"));
    }

    let response = Response::new().add_attribute("action", "receive_ack");

    // Check if there is an inflight packet for the received (sequence)
    let sent_packet = INFLIGHT_PACKETS.may_load(deps.storage, sequence)?;
    let Some(mut inflight_packet) = sent_packet else {
        // If there isn't, continue
        return Ok(response.add_attribute("msg", "received unexpected ack"));
    };

    if success {
        // Remove the in-flight packet
        INFLIGHT_PACKETS.remove(deps.storage, sequence);

        // If the acc is successful, there is nothing else to do and the crosschain swap has been completed
        return Ok(response.add_attribute("msg", "success"));
    }

    inflight_packet.status = state::ibc::PacketLifecycleStatus::AckFailure;
    INFLIGHT_PACKETS.save(deps.storage, sequence, &inflight_packet)?;

    Ok(response.add_attribute("error", "ibc acknowledgement failed"))
}

// This is very similar to the handling of acks, but it always creates a
// recovery since there is no concept of a "successful timeout"
pub fn receive_timeout(
    deps: DepsMut,
    source_channel: String,
    sequence: u64,
) -> Result<Response, ContractError> {
    let config: Config = CONFIG.load(deps.storage)?;
    if source_channel != config.ibc_channel_id {
        // If the ack is not for this contract, return a success
        return Ok(Response::new()
            .add_attribute("action", "receive_ack")
            .add_attribute("error", "received ack for different channel"));
    }

    let response = Response::new().add_attribute("action", "receive_timeout");

    // Check if there is an inflight packet for the received (sequence)
    let sent_packet = INFLIGHT_PACKETS.may_load(deps.storage, sequence)?;
    let Some(mut inflight_packet) = sent_packet else {
        // If there isn't, continue
        return Ok(response.add_attribute("error", "received unexpected timeout"));
    };

    inflight_packet.status = state::ibc::PacketLifecycleStatus::TimedOut;
    INFLIGHT_PACKETS.save(deps.storage, sequence, &inflight_packet)?;

    Ok(response.add_attribute("error", "ibc packet timed out"))
}
