use crate::helpers::{get_redemption_rate, paginate_map};
use crate::msg::{
    BatchResponse, BatchesResponse, ConfigResponse, IBCQueueResponse, IBCReplyQueueResponse,
    StateResponse,
};
use crate::state::ibc::IBCTransfer;
use crate::state::{
    unstake_requests, UnstakeRequest, BATCHES, CONFIG, IBC_WAITING_FOR_REPLY, INFLIGHT_PACKETS,
    PENDING_BATCH_ID, STATE,
};
use cosmwasm_std::{Deps, StdResult, Timestamp, Uint128};
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
        oracle_contract_address_v2: config
            .oracle_contract_address_v2
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

fn batch_to_response(batch: Batch) -> BatchResponse {
    BatchResponse {
        id: batch.id,
        batch_total_liquid_stake: batch.batch_total_liquid_stake,
        expected_native_unstaked: batch.expected_native_unstaked.unwrap_or(Uint128::zero()),
        received_native_unstaked: batch.received_native_unstaked.unwrap_or(Uint128::zero()),
        next_batch_action_time: Timestamp::from_seconds(
            batch.next_batch_action_time.unwrap_or(0u64),
        ),
        status: batch.status.as_str().to_string(),
        unstake_request_count: batch.unstake_requests_count.unwrap_or(0), // Fallback. Only is none if migration failed. Would be set in updates for new batches though
    }
}

pub fn query_batch(deps: Deps, id: u64) -> StdResult<BatchResponse> {
    let batch: Batch = BATCHES.load(deps.storage, id)?;
    Ok(batch_to_response(batch))
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
        batches: batches.into_iter().map(|v| batch_to_response(v)).collect(),
    };
    Ok(res)
}

pub fn query_batches_by_ids(deps: Deps, ids: Vec<u64>) -> StdResult<BatchesResponse> {
    let batches: Vec<Batch> = ids
        .into_iter()
        .map(|id| BATCHES.load(deps.storage, id))
        .filter_map(|r| {
            if r.is_ok() {
                let batch = r.unwrap();
                return Some(batch);
            }
            None
        })
        .collect();

    let res = BatchesResponse {
        batches: batches.into_iter().map(|v| batch_to_response(v)).collect(),
    };
    Ok(res)
}

pub fn query_pending_batch(deps: Deps) -> StdResult<BatchResponse> {
    let pending_batch_id = PENDING_BATCH_ID.load(deps.storage)?;
    let pending_batch = BATCHES.load(deps.storage, pending_batch_id)?;

    Ok(batch_to_response(pending_batch))
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

pub fn query_unstake_requests(deps: Deps, user: String) -> StdResult<Vec<UnstakeRequest>> {
    let unstaking_requests = unstake_requests()
        .idx
        .by_user
        .prefix(user.to_string())
        .range(deps.storage, None, None, cosmwasm_std::Order::Ascending)
        .filter_map(|r| {
            if r.is_ok() {
                let request = r.unwrap().1;
                return Some(request);
            }
            None
        })
        .collect();

    Ok(unstaking_requests)
}

// DEPR
pub fn query_all_unstake_requests(
    deps: Deps,
    start_after: Option<u64>,
    limit: Option<u32>,
) -> StdResult<Vec<UnstakeRequest>> {
    let unstaking_requests = unstake_requests()
        .idx
        .by_user
        .range(
            deps.storage,
            start_after.map(|s| Bound::exclusive(("".to_string(), s))),
            None,
            cosmwasm_std::Order::Ascending,
        )
        .take(limit.unwrap_or(u32::MAX) as usize)
        .filter_map(|r| {
            if r.is_ok() {
                let request = r.unwrap().1;
                return Some(request);
            }
            None
        })
        .collect();

    Ok(unstaking_requests)
}

pub fn query_all_unstake_requests_v2(
    deps: Deps,
    start_after: Option<u64>,
    limit: Option<u32>,
) -> StdResult<Vec<(String, u64, Uint128)>> {
    let unstaking_requests = unstake_requests()
        .idx
        .by_user
        .range(
            deps.storage,
            start_after.map(|s| Bound::exclusive(("".to_string(), s))),
            None,
            cosmwasm_std::Order::Ascending,
        )
        .take(limit.unwrap_or(u32::MAX) as usize)
        .filter_map(|r| {
            if r.is_ok() {
                let request = r.unwrap().1;
                return Some((request.user, request.batch_id, request.amount));
            }
            None
        })
        .collect();

    Ok(unstaking_requests)
}
