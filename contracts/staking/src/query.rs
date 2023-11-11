use crate::msg::{BatchResponse, ConfigResponse, StateResponse};
use crate::state::{Config, BATCHES, CONFIG, PENDING_BATCH, STATE};
use cosmwasm_std::{Deps, StdResult};
use std::fmt::DebugMap;

pub fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
    let config = CONFIG.load(deps.storage)?;

    let res = ConfigResponse {
        native_token_denom: config.native_token_denom,
        liquid_stake_token_denom: config.liquid_stake_token_denom,
        treasury_address: config.treasury_address.to_string(),
        node_operators: config
            .node_operators
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
        minimum_rewards_to_collect: config.minimum_rewards_to_collect,
    };
    Ok(res)
}

pub fn query_state(deps: Deps) -> StdResult<StateResponse> {
    let state = STATE.load(deps.storage)?;

    let res = StateResponse {
        total_native_token: state.total_native_token,
        total_liquid_stake_token: state.total_liquid_stake_token,
        pending_owner: state
            .pending_owner
            .map(|v| v.to_string())
            .unwrap_or_default(),
        total_reward_amount: state.total_reward_amount,
    };
    Ok(res)
}

pub fn query_batch(deps: Deps, id: u64) -> StdResult<BatchResponse> {
    let batch = BATCHES.load(deps.storage, id)?;

    let res = BatchResponse {
        batch_total_liquid_stake: batch.batch_total_liquid_stake,
        expected_native_unstaked: batch.expected_native_unstaked.unwrap_or_default(),
        next_batch_action_time: batch.next_batch_action_time.unwrap_or_default(),
        status: batch.status.as_str().to_string(),
    };
    Ok(res)
}
