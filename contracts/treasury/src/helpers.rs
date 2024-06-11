use cosmwasm_std::{Addr, StdError, StdResult};

pub fn validate_address(address: &String, prefix: &str) -> StdResult<Addr> {
    let validated_addr =
        bech32::decode(address).map_err(|_| StdError::generic_err("Invalid address"))?;

    if validated_addr.0 != prefix {
        return Err(StdError::generic_err("Invalid address prefix"));
    }

    Ok(Addr::unchecked(address))
}
