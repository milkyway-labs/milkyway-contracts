use bech32::Hrp;
use std::collections::HashSet;

use cosmwasm_std::{Addr, StdError, StdResult};

pub const BECH32_HRP_MAX_LEN: usize = 83;
pub const IBC_CHANNEL_PREFIX: &str = "channel-";

/// Validate the HRP (human readable part).of a bech32 encoded address
/// as for [BIP-173](https://en.bitcoin.it/wiki/BIP_0173).
pub fn validate_address_prefix(hrp: &str) -> StdResult<String> {
    Hrp::parse(hrp).map_err(|e| StdError::generic_err(format!("invalid address prefix {e}")))?;
    Ok(hrp.to_string())
}

/// Ensure that the provided bech32 address have the provided prefix and its
/// valid.
pub fn validate_address(address: &str, prefix: &str) -> StdResult<Addr> {
    if let Ok((decoded_prefix, _)) = bech32::decode(address) {
        if decoded_prefix.as_str() == prefix {
            Ok(Addr::unchecked(address))
        } else {
            Err(StdError::generic_err(format!(
                "Invalid address prefix, address: {address}, prefix: {prefix}"
            )))
        }
    } else {
        Err(StdError::generic_err("Invalid address"))
    }
}

/// Validates addresses are valid and unique and returns a vector of validated addresses
pub fn validate_addresses(addresses: &Vec<String>, prefix: &str) -> StdResult<Vec<Addr>> {
    let mut validated = Vec::new();
    let mut seen = HashSet::new();

    for address in addresses {
        let validated_addr = validate_address(address, prefix)?;

        if seen.contains(address) {
            return Err(StdError::generic_err("Duplicate address"));
        }

        validated.push(validated_addr);
        seen.insert(address.clone());
    }

    Ok(validated)
}

/// Ensures that the provided IBC channel if valid.
pub fn validate_ibc_channel(ibc_channel_id: &str) -> StdResult<String> {
    let channel_id_correct = ibc_channel_id.starts_with(IBC_CHANNEL_PREFIX)
        && ibc_channel_id
            .strip_prefix(IBC_CHANNEL_PREFIX)
            .unwrap()
            .parse::<u64>()
            .is_ok();
    if !channel_id_correct {
        return Err(StdError::generic_err(format!(
            "invalid ibc channel {ibc_channel_id}"
        )));
    }

    Ok(ibc_channel_id.to_string())
}
