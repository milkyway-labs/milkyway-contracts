use crate::helpers::{get_redemption_rate, paginate_map};
use crate::msg::{
    BatchResponse, BatchesResponse, ConfigResponse, IBCQueueResponse, IBCReplyQueueResponse,
    LiquidUnstakeRequestResponse, StateResponse,
};
use crate::state::ibc::IBCTransfer;
use crate::state::{
    BATCHES, CONFIG, IBC_WAITING_FOR_REPLY, INFLIGHT_PACKETS, PENDING_BATCH_ID, STATE,
    UNSTAKE_REQUESTS, UNSTAKE_REQUESTS_BY_USER,
};
use cosmwasm_std::{Addr, Deps, StdResult, Timestamp, Uint128};
use cw_storage_plus::Bound;
use milky_way::staking::{Batch, BatchStatus};

pub fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
    let config = CONFIG.load(deps.storage)?;

    let res = ConfigResponse {
        native_token_denom: config.native_token_denom,
        liquid_stake_token_denom: config.liquid_stake_token_denom,
        treasury_address: config.treasury_address.to_string(),
        monitors: config
            .monitors
            .unwrap()
            .into_iter()
            .map(|v| v.to_string())
            .collect(),
        validators: config
            .validators
            .into_iter()
            .map(|v| v.to_string())
            .collect(),
        batch_period: config.batch_period,
        unbonding_period: config.unbonding_period,
        minimum_liquid_stake_amount: config.minimum_liquid_stake_amount,
        staker_address: config.multisig_address_config.staker_address.to_string(),
        reward_collector_address: config
            .multisig_address_config
            .reward_collector_address
            .to_string(),
        protocol_fee_config: config.protocol_fee_config,
        ibc_channel_id: config.ibc_channel_id,
        stopped: config.stopped,
        oracle_contract_address: config
            .oracle_contract_address
            .map(|v| v.to_string())
            .unwrap_or_default(),
    };
    Ok(res)
}

pub fn query_state(deps: Deps) -> StdResult<StateResponse> {
    let state = STATE.load(deps.storage)?;

    let res = StateResponse {
        total_native_token: state.total_native_token,
        total_liquid_stake_token: state.total_liquid_stake_token,
        rate: get_redemption_rate(&deps),
        pending_owner: state
            .pending_owner
            .map(|v| v.to_string())
            .unwrap_or_default(),
        total_reward_amount: state.total_reward_amount,
        total_fees: state.total_fees,
    };
    Ok(res)
}

fn batch_to_response(deps: Deps, batch: Batch) -> BatchResponse {
    let unstake_requests = UNSTAKE_REQUESTS
        .prefix(batch.id)
        .range(deps.storage, None, None, cosmwasm_std::Order::Ascending)
        .map(|v| v.unwrap())
        .collect::<Vec<_>>();
    BatchResponse {
        id: batch.id,
        batch_total_liquid_stake: batch.batch_total_liquid_stake,
        expected_native_unstaked: batch.expected_native_unstaked.unwrap_or(Uint128::zero()),
        received_native_unstaked: batch.received_native_unstaked.unwrap_or(Uint128::zero()),
        next_batch_action_time: Timestamp::from_seconds(
            batch.next_batch_action_time.unwrap_or(0u64),
        ),
        status: batch.status.as_str().to_string(),
        requests: unstake_requests
            .into_iter()
            .map(|v| LiquidUnstakeRequestResponse {
                user: v.0,
                amount: v.1,
            })
            .collect(),
    }
}

pub fn query_batch(deps: Deps, id: u64) -> StdResult<BatchResponse> {
    let batch: Batch = BATCHES.load(deps.storage, id)?;
    Ok(batch_to_response(deps, batch))
}

pub fn query_batches(
    deps: Deps,
    start_after: Option<u64>,
    limit: Option<u32>,
    status: Option<BatchStatus>,
) -> StdResult<BatchesResponse> {
    let filter_closure =
        status.map(|s| Box::new(move |v: &Batch| v.status == s) as Box<dyn Fn(&Batch) -> bool>);

    let batches = paginate_map(
        deps,
        &BATCHES,
        start_after,
        limit,
        cosmwasm_std::Order::Ascending,
        filter_closure,
    )?;

    let res = BatchesResponse {
        batches: batches
            .into_iter()
            .map(|v| batch_to_response(deps, v))
            .collect(),
    };
    Ok(res)
}

pub fn query_pending_batch(deps: Deps) -> StdResult<BatchResponse> {
    let pending_batch_id = PENDING_BATCH_ID.load(deps.storage)?;
    let pending_batch = BATCHES.load(deps.storage, pending_batch_id)?;

    Ok(batch_to_response(deps, pending_batch))
}

pub fn query_ibc_queue(
    deps: Deps,
    start_after: Option<u64>,
    limit: Option<u32>,
) -> StdResult<IBCQueueResponse> {
    let inflight_packets: Vec<IBCTransfer> = paginate_map(
        deps,
        &INFLIGHT_PACKETS,
        start_after,
        limit,
        cosmwasm_std::Order::Ascending,
        None,
    )?;
    let res = IBCQueueResponse {
        ibc_queue: inflight_packets,
    };

    Ok(res)
}

pub fn query_reply_queue(
    deps: Deps,
    start_after: Option<u64>,
    limit: Option<u32>,
) -> StdResult<IBCReplyQueueResponse> {
    let ibc_messages_waiting = paginate_map(
        deps,
        &IBC_WAITING_FOR_REPLY,
        start_after,
        limit,
        cosmwasm_std::Order::Ascending,
        None,
    )?;
    let res = IBCReplyQueueResponse {
        ibc_queue: ibc_messages_waiting,
    };
    Ok(res)
}

pub fn query_claimable(
    deps: Deps,
    user: Addr,
    start_after: Option<u64>,
    limit: Option<u32>,
) -> StdResult<BatchesResponse> {
    deps.api.addr_validate(&user.to_string())?;

    let unstaking_requests = UNSTAKE_REQUESTS_BY_USER
        .prefix(user.to_string())
        .range(
            deps.storage,
            start_after.map(Bound::exclusive),
            None,
            cosmwasm_std::Order::Ascending,
        )
        .take(limit.unwrap_or(u32::MAX as u32) as usize)
        .map(|v| v.unwrap())
        .collect::<Vec<_>>();

    let batches = unstaking_requests
        .into_iter()
        .filter_map(|v| {
            let batch_id = v.0;
            let batch = BATCHES.load(deps.storage, batch_id).ok()?;
            if batch.status == BatchStatus::Received {
                Some(batch)
            } else {
                None
            }
        })
        .map(|v| batch_to_response(deps, v))
        .collect();

    let res = BatchesResponse { batches };
    Ok(res)
}
