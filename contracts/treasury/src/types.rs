use cosmwasm_schema::cw_serde;
use cosmwasm_std::StdResult;
use milky_way::utils::validate_address_prefix;

use crate::{
    error::ContractError,
    state::{NativeChainConfig, ProtocolChainConfig},
};

/// Config related to the chain for which we are creating
/// the LST token.
/// For example Celestia is the native chain of milkTIA LST token.
#[cw_serde]
pub struct UnsafeNativeChainConfig {
    /// Bech32 prefix for accounts (e.g. "celestia", "initia", etc)
    pub account_address_prefix: String,
}

impl UnsafeNativeChainConfig {
    pub fn validate(&self) -> StdResult<NativeChainConfig> {
        Ok(NativeChainConfig {
            account_address_prefix: validate_address_prefix(&self.account_address_prefix)?,
        })
    }
}
/// Config related to the chain where the smart contract is deployed.
#[cw_serde]
pub struct UnsafeProtocolChainConfig {
    /// Bech32 prefix for accounts (e.g. "osmosis", "milkyway", etc)
    pub account_address_prefix: String,
}

impl UnsafeProtocolChainConfig {
    pub fn validate(&self) -> Result<ProtocolChainConfig, ContractError> {
        Ok(ProtocolChainConfig {
            account_address_prefix: validate_address_prefix(&self.account_address_prefix)?,
        })
    }
}
