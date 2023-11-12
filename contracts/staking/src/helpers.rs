use cosmwasm_std::{Addr, Api, StdError, StdResult, Uint128};
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

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::mock_dependencies;

    #[test]
    fn validate_addresses_success() {
        let addresses = vec![
            "osmo12z558dm3ew6avgjdj07mfslx80rp9sh8nt7q3w".to_string(),
            "osmo13ftwm6z4dq6ugjvus2hf2vx3045ahfn3dq7dms".to_string(),
        ];

        let result = validate_addresses(
            mock_dependencies().as_ref().api,
            addresses,
            "osmo".to_string(),
        )
        .unwrap();

        assert_eq!(2, result.len());
    }

    #[test]
    fn validate_addresses_duplicate() {
        let addresses = vec![
            "osmo12z558dm3ew6avgjdj07mfslx80rp9sh8nt7q3w".to_string(),
            "osmo12z558dm3ew6avgjdj07mfslx80rp9sh8nt7q3w".to_string(),
        ];

        let result = validate_addresses(
            mock_dependencies().as_ref().api,
            addresses,
            "osmo".to_string(),
        );

        assert!(result.is_err());
    }
    #[test]
    fn validate_addresses_invalid() {
        let addresses = vec![
            "osmo12z558dm3ew6avgjdj07mfslx80rp9sh8nt7q3w".to_string(),
            "osmo12z558dm3ew6avgjdj07mfslx80rp9sh8nt7q3w".to_string(),
        ];

        let result = validate_addresses(
            mock_dependencies().as_ref().api,
            addresses,
            "osmo".to_string(),
        );

        assert!(result.is_err());
    }
    // TODO: Review this test - currently passing but I think mock_deps has weird deps.api.addr_validate behavior?
    #[test]
    fn validate_addresses_invalid_prefix() {
        let addresses = vec![
            "a".to_string(),
            "osmo12z558dm3ew6avgjdj07mfslx80rp9sh8nt7q3w".to_string(),
        ];

        let result = validate_addresses(
            mock_dependencies().as_ref().api,
            addresses,
            "celestia".to_string(),
        );

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
