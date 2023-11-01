use crate::ack::{make_ack_fail, make_ack_success};
use crate::error::{ContractError, ContractResult};
use crate::msg::IbcExecuteMsg;
use crate::state::{BATCHES, IBC_CONFIG, STATE};
#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    from_binary, DepsMut, Env, IbcBasicResponse, IbcChannel, IbcChannelCloseMsg,
    IbcChannelConnectMsg, IbcChannelOpenMsg, IbcChannelOpenResponse, IbcOrder, IbcPacketAckMsg,
    IbcPacketReceiveMsg, IbcPacketTimeoutMsg, IbcReceiveResponse, StdResult, Uint128,
};
use milky_way::staking::BatchStatus;
use osmosis_std::types::ibc;
// TODO: implement
pub const IBC_VERSION: &str = "mw-1";

/// Handles the `OpenInit` and `OpenTry` parts of the IBC handshake.
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn ibc_channel_open(
    _deps: DepsMut,
    _env: Env,
    msg: IbcChannelOpenMsg,
) -> Result<IbcChannelOpenResponse, ContractError> {
    validate_order_and_version(msg.channel(), msg.counterparty_version())?;
    Ok(())
}
/// Handles the OpenAck and OpenConfirm handshake steps.
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn ibc_channel_connect(
    deps: DepsMut,
    _env: Env,
    msg: IbcChannelConnectMsg,
) -> Result<IbcBasicResponse, ContractError> {
    validate_order_and_version(msg.channel(), msg.counterparty_version())?;
    let mut ibc_config = IBC_CONFIG.load(deps.storage)?;

    ibc_config.channel = Some(msg.channel().clone());
    IBC_CONFIG.save(deps.storage, &ibc_config)?;

    Ok(IbcBasicResponse::new()
        .add_attribute("method", "ibc_channel_connect")
        .add_attribute("channel_id", msg.channel().connection_id.clone()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn ibc_channel_close(
    deps: DepsMut,
    _env: Env,
    msg: IbcChannelCloseMsg,
) -> Result<IbcBasicResponse, ContractError> {
    let channel = msg.channel().endpoint.channel_id.clone();
    // TODO: Discuss if we need to do anything here.
    // Currently unsetting the channel in the config
    let mut ibc_config = IBC_CONFIG.load(deps.storage)?;
    ibc_config.channel = None;
    IBC_CONFIG.save(deps.storage, &ibc_config)?;
    Ok(IbcBasicResponse::new()
        .add_attribute("method", "ibc_channel_close")
        .add_attribute("channel", channel))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn ibc_packet_receive(
    deps: DepsMut,
    env: Env,
    msg: IbcPacketReceiveMsg,
) -> Result<IbcReceiveResponse, ContractError> {
    // Regardless of if our processing of this packet works we need to
    // commit an ACK to the chain. As such, we wrap all handling logic
    // in a seprate function and on error write out an error ack.
    // Commit ack
    // TODO: Implement this.
    match do_ibc_packet_receive(deps, env, msg) {
        Ok(response) => Ok(response),
        Err(error) => Ok(IbcReceiveResponse::new()
            .add_attribute("method", "ibc_packet_receive")
            .add_attribute("error", error.to_string())
            .set_ack(make_ack_fail(error.to_string()))),
    }
}

pub fn do_ibc_packet_receive(
    deps: DepsMut,
    _env: Env,
    msg: IbcPacketReceiveMsg,
) -> Result<IbcReceiveResponse, ContractError> {
    // TODO:
    // validate batch number is provided
    // pass to "handle_batch" function
    // !!! NOT CHECKING SENDER YET
    // RESEARCH: https://ideas.skip.money/t/how-to-make-smart-contracts-modules-more-accessible-to-cross-chain-users/86

    let msg: IbcExecuteMsg = from_binary(&msg.packet.data)?;

    match msg {
        IbcExecuteMsg::ReceiveBatch {
            batch_id,
            batch_amount,
        } => execute_receive_batch(deps, batch_id, batch_amount),
        IbcExecuteMsg::ReceiveRewards { reward_amount } => {
            execute_receive_batch_rewards(deps, reward_amount)
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn ibc_packet_ack(
    _deps: DepsMut,
    _env: Env,
    _ack: IbcPacketAckMsg,
) -> Result<IbcBasicResponse, ContractError> {
    // is there a way we can pass back any interesting info here?
    Ok(IbcBasicResponse::new().add_attribute("method", "ibc_packet_ack"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn ibc_packet_timeout(
    deps: DepsMut,
    _env: Env,
    msg: IbcPacketTimeoutMsg,
) -> Result<IbcBasicResponse, ContractError> {
    // TODO: Implement this.
    Ok(IbcBasicResponse::new().add_attribute("method", "ibc_packet_timeout"))
}

pub fn validate_order_and_version(
    channel: &IbcChannel,
    counterparty_version: Option<&str>,
) -> Result<(), ContractError> {
    // TODO: any reason to considere ordered channels?

    // We expect an unordered channel here. Ordered channels have the
    // property that if a message is lost the entire channel will stop
    // working until you start it again.
    if channel.order != IbcOrder::Unordered {
        return Err(ContractError::OrderedChannel {});
    }

    if channel.version != IBC_VERSION {
        return Err(ContractError::InvalidVersion {
            actual: channel.version.to_string(),
            expected: IBC_VERSION.to_string(),
        });
    }

    // Make sure that we're talking with a counterparty who speaks the
    // same "protocol" as us.
    //
    // For a connection between chain A and chain B being established
    // by chain A, chain B knows counterparty information during
    // `OpenTry` and chain A knows counterparty information during
    // `OpenAck`. We verify it when we have it but when we don't it's
    // alright.
    if let Some(counterparty_version) = counterparty_version {
        if counterparty_version != IBC_VERSION {
            return Err(ContractError::InvalidVersion {
                actual: counterparty_version.to_string(),
                expected: IBC_VERSION.to_string(),
            });
        }
    }

    Ok(())
}
// TODO: implement this.
fn execute_receive_batch(
    deps: DepsMut,
    batch_id: u64,
    batch_amount: Uint128,
) -> Result<IbcReceiveResponse, ContractError> {
    // TODO:
    // check if batch exists -X
    // check if batch is already executed - X
    // execute batch
    // update batch status -X

    // Batch must be submitted
    if let Ok(mut batch) = BATCHES.load(deps.storage, batch_id) {
        if batch.status != BatchStatus::Submitted {
            return Err(ContractError::UnexpecedBatchStatus {
                actual: batch.status,
                expected: BatchStatus::Submitted,
            });
        }
        batch.update_status(BatchStatus::Received, None)
    }
    // palceholder logic
    let mut state = STATE.load(deps.storage)?;

    state.total_native_token += batch_amount;
    STATE.save(deps.storage, &state)?;

    // TODO: Process Batch and distribute withdrawals

    Ok(IbcReceiveResponse::new()
        .add_attribute("method", "execute_receive_batch")
        .add_attribute("reward_amount", batch_amount.to_string())
        .add_attribute("batch_id", batch_id.to_string())
        .set_ack(make_ack_success()))
}
// TODO: implement this.
fn execute_receive_batch_rewards(
    deps: DepsMut,
    reward_amount: Uint128,
) -> Result<IbcReceiveResponse, ContractError> {
    let mut state = STATE.load(deps.storage)?;

    // Rewards are now in SC balance
    state.total_reward_amount += reward_amount;
    STATE.save(deps.storage, &state)?;

    Ok(IbcReceiveResponse::new()
        .add_attribute("method", "execute_receive_batch_rewards")
        .add_attribute("reward_amount", reward_amount.to_string())
        .set_ack(make_ack_success()))
}
