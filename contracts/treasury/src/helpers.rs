use cosmwasm_std::{Addr, StdError, StdResult};

pub fn validate_address(address: &String, prefix: &str) -> StdResult<Addr> {
    let validated_addr = bech32::decode(address);

    if validated_addr.is_err() {
        return Err(StdError::generic_err("Invalid address"));
    }

    if validated_addr.unwrap().0 != prefix {
        return Err(StdError::generic_err("Invalid address prefix"));
    }

    Ok(Addr::unchecked(address))
}
