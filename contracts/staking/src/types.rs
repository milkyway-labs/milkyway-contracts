use cosmwasm_schema::cw_serde;
use cosmwasm_std::Uint128;

use crate::{
    error::{ContractError, ContractResult},
    helpers::{
        validate_address, validate_address_prefix, validate_addresses, validate_denom,
        validate_ibc_denom,
    },
    state::{NativeChainConfig, ProtocolChainConfig, ProtocolFeeConfig},
};

const MAX_TREASURY_FEE: Uint128 = Uint128::new(100_000);
/// The maximum allowed unbonding period is 42 days,
/// which is twice the typical staking period of a Cosmos SDK-based chain.
pub const MAX_UNBONDING_PERIOD: u64 = 3_628_800;

/// Config related to the fees collected by the contract to
/// operate the liquid staking protocol.
#[cw_serde]
pub struct UnsafeProtocolFeeConfig {
    pub dao_treasury_fee: Uint128, // not using a fraction, fee percentage=x/100000

    /// Address where the collected fees are sent.
    /// If this value is None, the fees are sent to the treasury.
    pub treasury_address: Option<String>,
}

impl UnsafeProtocolFeeConfig {
    pub fn validate(
        &self,
        config: &ProtocolChainConfig,
    ) -> Result<ProtocolFeeConfig, ContractError> {
        if self.dao_treasury_fee > MAX_TREASURY_FEE {
            return Err(ContractError::InvalidDaoTreasuryFee {});
        }

        Ok(ProtocolFeeConfig {
            dao_treasury_fee: self.dao_treasury_fee,
            treasury_address: self
                .treasury_address
                .as_ref()
                .map(|a| validate_address(a, &config.account_address_prefix))
                .transpose()?,
        })
    }
}

/// Config related to the chain for which we are creating
/// the LST token.
/// For example Celestia is the native chain of milkTIA LST token.
#[cw_serde]
pub struct UnsafeNativeChainConfig {
    /// Bech32 prefix for accounts (e.g. "celestia", "initia", etc)
    pub account_address_prefix: String,

    /// Bech32 prefix for validator accounts (e.g. "celestiavaloper", "initavaloper", etc)
    pub validator_address_prefix: String,

    /// Denomination of underlying token (e.g. "utia", "uinit", etc)
    pub token_denom: String,

    /// Set of validators who will receive the delegations.
    pub validators: Vec<String>,

    /// The staking module's unbonding period in seconds.
    pub unbonding_period: u64,

    /// Address of the account that delegates the tokens
    /// toward the validators.
    pub staker_address: String,

    /// Address where the staking rewards are withdrawn.
    pub reward_collector_address: String,
}

impl UnsafeNativeChainConfig {
    pub fn validate(&self) -> ContractResult<NativeChainConfig> {
        if self.unbonding_period > MAX_UNBONDING_PERIOD {
            return Err(ContractError::ValueTooBig {
                field_name: "unbonding_period".to_string(),
                value: Uint128::from(self.unbonding_period),
                max: Uint128::from(MAX_UNBONDING_PERIOD),
            });
        }

        Ok(NativeChainConfig {
            account_address_prefix: validate_address_prefix(&self.account_address_prefix)?,
            validator_address_prefix: validate_address_prefix(&self.validator_address_prefix)?,
            token_denom: validate_denom(&self.token_denom)?,
            validators: validate_addresses(&self.validators, &self.validator_address_prefix)?,
            unbonding_period: self.unbonding_period,
            staker_address: validate_address(&self.staker_address, &self.account_address_prefix)?,
            reward_collector_address: validate_address(
                &self.reward_collector_address,
                &self.account_address_prefix,
            )?,
        })
    }
}
/// Config related to the chain where the smart contract is deployed.
#[cw_serde]
pub struct UnsafeProtocolChainConfig {
    /// Bech32 prefix for accounts (e.g. "osmosis", "milkyway", etc)
    pub account_address_prefix: String,

    /// IBC denom of the supported token (e.g. IBC denom of TIA, INIT, etc)
    pub ibc_token_denom: String,

    /// IBC channel id from the Protocol chain to the base chain (e.g. Osmosis -> Celestia)
    pub ibc_channel_id: String,

    /// Minimum amount of token that can be liquid staked.
    pub minimum_liquid_stake_amount: Uint128,

    /// The redemption / purchase rate oracle address
    pub oracle_address: Option<String>,
}

impl UnsafeProtocolChainConfig {
    pub fn validate(&self) -> Result<ProtocolChainConfig, ContractError> {
        let channel_id_correct = self.ibc_channel_id.starts_with("channel-")
            && self
                .ibc_channel_id
                .strip_prefix("channel-")
                .unwrap()
                .parse::<u64>()
                .is_ok();
        if !channel_id_correct {
            return Err(ContractError::IbcChannelConfigWrong {});
        }

        Ok(ProtocolChainConfig {
            account_address_prefix: validate_address_prefix(&self.account_address_prefix)?,
            ibc_token_denom: validate_ibc_denom(&self.ibc_token_denom)?,
            ibc_channel_id: self.ibc_channel_id.clone(),
            minimum_liquid_stake_amount: self.minimum_liquid_stake_amount,
            oracle_address: self
                .oracle_address
                .as_ref()
                .map(|a| validate_address(a, &self.account_address_prefix))
                .transpose()?,
        })
    }
}
