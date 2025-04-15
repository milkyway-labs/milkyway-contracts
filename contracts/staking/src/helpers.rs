use cosmwasm_std::{Decimal, Deps, Order, StdError, StdResult, Uint128};
use cw_storage_plus::{Bound, Bounder, KeyDeserialize, Map};
use sha2::{Digest, Sha256};

use crate::state::State;

pub fn compute_mint_amount(
    total_native_token: Uint128,
    total_liquid_stake_token: Uint128,
    native_to_stake: Uint128,
) -> Uint128 {
    // TODO: Review integer math
    // Possible truncation issues when quantities are small
    // Initial very large total_native_token would cause round to 0 and block minting
    // Mint at a 1:1 ratio if there is no total native token
    // Amount = Total stTIA * (Amount of native token / Total native token)
    if total_native_token.is_zero() {
        native_to_stake
    } else {
        total_liquid_stake_token.multiply_ratio(native_to_stake, total_native_token)
    }
}

pub fn compute_unbond_amount(
    total_native_token: Uint128,
    total_liquid_stake_token: Uint128,
    batch_liquid_stake_token: Uint128,
) -> Uint128 {
    if batch_liquid_stake_token.is_zero() {
        Uint128::zero()
    } else {
        // unbond amount is calculated at the batch level
        // total_native_token - total TIA delegated by MilkyWay
        // batch_liquid_stake_token - total stTIA in submitted batch
        // total_liquid_stake_token - total stTIA minted by MilkyWay

        total_native_token.multiply_ratio(batch_liquid_stake_token, total_liquid_stake_token)
    }
}

// Hash creates a new address from address type and key.
// The functions should only be used by new types defining their own address function
// (eg public keys).
/// https://github.com/cosmos/cosmos-sdk/blob/main/types/address/hash.go
pub fn addess_hash(typ: &str, key: &[u8]) -> [u8; 32] {
    let mut hasher = Sha256::default();
    hasher.update(typ.as_bytes());
    let th = hasher.finalize();
    let mut hasher = Sha256::default();
    hasher.update(th);
    hasher.update(key);
    hasher.finalize().into()
}

// derives the sender address to be used when calling wasm hooks
// https://github.com/osmosis-labs/osmosis/blob/master/x/ibc-hooks/keeper/keeper.go#L170 ```
pub const SENDER_PREFIX: &str = "ibc-wasm-hook-intermediary";

pub fn derive_intermediate_sender(
    channel_id: &str,
    original_sender: &str,
    bech32_prefix: &str,
) -> StdResult<String> {
    let sender_str = format!("{channel_id}/{original_sender}");
    let sender_hash_32 = addess_hash(SENDER_PREFIX, sender_str.as_bytes());
    let hrp =
        bech32::Hrp::parse(bech32_prefix).map_err(|e| StdError::generic_err(e.to_string()))?;
    bech32::encode::<bech32::Bech32>(hrp, &sender_hash_32)
        .map_err(|e| StdError::generic_err(e.to_string()))
}

/// Generic function for paginating a list of (K, V) pairs in a
/// CosmWasm Map.
#[allow(clippy::type_complexity)]
pub fn paginate_map<'a, 'b, K, V, R: 'static>(
    deps: Deps,
    map: &Map<'a, K, V>,
    start_after: Option<K>,
    limit: Option<u32>,
    order: Order,
    filter: Option<Box<dyn Fn(&V) -> bool>>,
) -> StdResult<Vec<V>>
where
    K: Bounder<'a> + KeyDeserialize<Output = R> + 'b,
    V: serde::de::DeserializeOwned + serde::Serialize,
{
    let (range_min, range_max) = match order {
        Order::Ascending => (start_after.map(Bound::exclusive), None),
        Order::Descending => (None, start_after.map(Bound::exclusive)),
    };

    let mut items = map.range(deps.storage, range_min, range_max, order);
    let mut taken = 0;
    let mut result = Vec::new();
    let limit = limit.unwrap_or(u32::MAX);
    while taken < limit {
        let item = items.next();
        match item {
            None => break,
            Some(r) => {
                if r.is_err() {
                    continue;
                }
                let (_, v) = r.unwrap();
                if let Some(filter) = &filter {
                    if filter(&v) {
                        taken += 1;
                        result.push(v);
                    } else {
                        continue;
                    }
                } else {
                    taken += 1;
                    result.push(v);
                }
            }
        }
    }
    Ok(result)
}

pub fn get_rates(state: &State) -> (Decimal, Decimal) {
    let total_native_token = state.total_native_token;
    let total_liquid_stake_token = state.total_liquid_stake_token;
    if total_liquid_stake_token.is_zero() || total_native_token.is_zero() {
        (Decimal::one(), Decimal::one())
    } else {
        // return redemption_rate, purchase_rate
        (
            Decimal::from_ratio(total_native_token, total_liquid_stake_token),
            Decimal::from_ratio(total_liquid_stake_token, total_native_token),
        )
    }
}

/// Checks if the provided denom is valid or not.
pub fn validate_denom(denom: impl Into<String>) -> StdResult<String> {
    let denom: String = denom.into();

    if denom.len() <= 3 {
        return Err(StdError::generic_err("denom len is less than 3"));
    }
    if !denom.chars().all(|c| c.is_ascii_alphabetic()) {
        return Err(StdError::generic_err("denom must be alphabetic"));
    }

    Ok(denom)
}

/// Checks the provided denom is a valid ibc denom or not.
pub fn validate_ibc_denom(ibc_denom: impl Into<String>) -> StdResult<String> {
    let ibc_denom: String = ibc_denom.into();

    if ibc_denom.starts_with("ibc/") && ibc_denom.strip_prefix("ibc/").unwrap().len() == 64 {
        Ok(ibc_denom)
    } else {
        Err(StdError::generic_err("ibc denom is invalid"))
    }
}
/// Note: The order of elements in the input vector is not preserved.
pub fn dedup_vec<T>(mut vec: Vec<T>) -> Vec<T>
where
    T: std::cmp::Ord,
{
    if vec.is_empty() {
        return vec;
    }

    vec.sort();
    vec.dedup();
    vec
}
