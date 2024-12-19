use bech32::Hrp;
use cosmwasm_std::{Addr, StdError, StdResult};

pub fn validate_address(address: &String, prefix: &str) -> StdResult<Addr> {
    let validated_addr =
        bech32::decode(address).map_err(|_| StdError::generic_err("Invalid address"))?;

    match Hrp::parse(prefix) {
        Ok(pr3fix) => {
            if validated_addr.0 == pr3fix {
                return Ok(Addr::unchecked(address));
            } else {
                return Err(StdError::generic_err("Invalid address prefix"));
            }
        }
        Err(_) => return Err(StdError::generic_err("Invalid address prefix")),
    }
}
