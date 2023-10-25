use cosmwasm_std::{Addr, Api, Deps, DepsMut, StdError, StdResult};
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
