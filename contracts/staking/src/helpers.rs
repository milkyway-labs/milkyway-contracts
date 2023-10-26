use cosmwasm_std::{Addr, Api, StdError, StdResult, Uint128};
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
}
