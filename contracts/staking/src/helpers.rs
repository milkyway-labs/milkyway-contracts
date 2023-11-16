use cosmwasm_std::{Addr, Api, StdError, StdResult, Uint128};
use sha2::{Digest, Sha256};
use std::collections::HashSet;

// Validates addresses are valid and unique and returns a vector of validated addresses
pub fn validate_addresses(
    _api: &dyn Api,
    addresses: Vec<String>,
    prefix: String,
) -> StdResult<Vec<Addr>> {
    let mut validated = Vec::new();
    let mut seen = HashSet::new();

    for address in addresses {
        let validated_addr = bech32::decode(&address);

        if validated_addr.is_err() {
            return Err(StdError::generic_err("Invalid address"));
        }

        if validated_addr.unwrap().0 != prefix {
            return Err(StdError::generic_err("Invalid address prefix"));
        }

        if seen.contains(&address) {
            return Err(StdError::generic_err("Duplicate address"));
        }

        validated.push(Addr::unchecked(&address));
        seen.insert(address.clone());
    }

    Ok(validated)
}

pub fn compute_mint_amount(
    total_native_token: Uint128,
    total_liquid_stake_token: Uint128,
    native_to_stake: Uint128,
) -> Uint128 {
    //TODO: Review integer math
    // Possible truncation issues when quantities are small
    // Initial very large total_native_token would cause round to 0 and block minting
    // Mint at a 1:1 ratio if there is no total native token or total liquid stake token
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
) -> Result<String, bech32_no_std::Error> {
    use bech32_no_std::ToBase32;
    let sender_str = format!("{channel_id}/{original_sender}");
    let sender_hash_32 = addess_hash(SENDER_PREFIX, sender_str.as_bytes());
    let sender = sender_hash_32.to_base32();
    bech32_no_std::encode(bech32_prefix, sender)
}
