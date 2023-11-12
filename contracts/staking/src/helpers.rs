use cosmwasm_std::{Addr, Api, StdError, StdResult, Uint128};
use sha2::{Digest, Sha256};
use std::collections::HashSet;

// Validates addresses are valid and unique and returns a vector of validated addresses
pub fn validate_addresses(api: &dyn Api, addresses: Vec<String>) -> StdResult<Vec<Addr>> {
    let mut validated = Vec::new();
    let mut seen = HashSet::new();

    for address in addresses {
        let validated_addr = api.addr_validate(&address)?;

        if seen.contains(&validated_addr) {
            return Err(StdError::generic_err("Duplicate address"));
        }

        validated.push(validated_addr.clone());
        seen.insert(validated_addr);
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
    let mint_amount;
    // Mint at a 1:1 ratio if there is no total native token or total liquid stake token
    // Amount = Total stTIA * (Amount of native token / Total native token)
    if total_native_token.is_zero() {
        mint_amount = native_to_stake
    } else {
        mint_amount = total_liquid_stake_token.multiply_ratio(native_to_stake, total_native_token)
    }

    mint_amount
}

pub fn compute_unbond_amount(
    total_native_token: Uint128,
    total_liquid_stake_token: Uint128,
    batch_liquid_stake_token: Uint128,
) -> Uint128 {
    let unbond_amount: Uint128;

    if batch_liquid_stake_token.is_zero() {
        unbond_amount = Uint128::zero()
    } else {
        // unbond amount is calculated at the batch level
        // total_native_token - total TIA delegated by MilkyWay
        // batch_liquid_stake_token - total stTIA in submitted batch
        // total_liquid_stake_token - total stTIA minted by MilkyWay
        unbond_amount =
            total_native_token.multiply_ratio(batch_liquid_stake_token, total_liquid_stake_token)
    }

    unbond_amount
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

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::mock_dependencies;

    #[test]
    fn validate_addresses_success() {
        let addresses = vec![
            "osmo1pkptre7fdkl6gfrzlesjjvhxhlc3r4gmmk8rs6".to_string(),
            "osmo10dyr9899g6t0pelew4nvf4j5c3jcgv0r73qga5".to_string(),
        ];

        let result = validate_addresses(mock_dependencies().as_ref().api, addresses).unwrap();

        assert_eq!(2, result.len());
    }

    #[test]
    fn validate_addresses_duplicate() {
        let addresses = vec![
            "osmo1pkptre7fdkl6gfrzlesjjvhxhlc3r4gmmk8rs6".to_string(),
            "osmo1pkptre7fdkl6gfrzlesjjvhxhlc3r4gmmk8rs6".to_string(),
        ];

        let result = validate_addresses(mock_dependencies().as_ref().api, addresses);

        assert!(result.is_err());
    }
    // TODO: Review this test - currently passing but I think mock_deps has weird deps.api.addr_validate behavior?
    #[test]
    fn validate_addresses_invalid() {
        let addresses = vec![
            "a".to_string(),
            "osmo1pkptre7fdkl6gfrzlesjjvhxhlc3r4gmmk8rs6".to_string(),
        ];

        let result = validate_addresses(mock_dependencies().as_ref().api, addresses);

        assert!(result.is_err());
    }
    // Basic test - based on figures from excalidraw
    #[test]
    fn test_compute_mint_amount() {
        let total_native_token = Uint128::from(2_000_000_000u128);
        let total_liquid_stake_token = Uint128::from(1_800_000_000u128);
        let native_to_stake = Uint128::from(100_000_000u128);
        let mint_amount = compute_mint_amount(
            total_native_token,
            total_liquid_stake_token,
            native_to_stake,
        );

        assert_eq!(mint_amount, Uint128::from(90_000_000u128));
    }
    // Basic test - based on figures from excalidraw
    #[test]
    fn test_compute_unbond_amount() {
        let total_native_token = Uint128::from(2_000_000_000u128);
        let total_liquid_stake_token = Uint128::from(1_800_000_000u128);
        let batch_unstake = Uint128::from(90_000_000u128);
        let unbond_amount =
            compute_unbond_amount(total_native_token, total_liquid_stake_token, batch_unstake);

        assert_eq!(unbond_amount, Uint128::from(100_000_000u128));
    }
}
