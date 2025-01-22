use crate::{
    error::ContractError,
    helpers::{validate_address, validate_addresses, validate_denom, validate_ibc_denom},
    state::{
        ibc::IBCTransfer, IbcWaitingForReply, NativeChainConfig, ProtocolChainConfig,
        ProtocolFeeConfig, UnstakeRequest,
    },
};
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Decimal, StdResult, Timestamp, Uint128};
use milky_way::staking::BatchStatus;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

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
    pub fn validate(&self, config: &ProtocolChainConfig) -> StdResult<ProtocolFeeConfig> {
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
    pub fn validate(&self) -> StdResult<NativeChainConfig> {
        let validators = validate_addresses(&self.validators, &self.validator_address_prefix)?;
        let staker_address = validate_address(&self.staker_address, &self.account_address_prefix)?;
        let reward_collector_address =
            validate_address(&self.reward_collector_address, &self.account_address_prefix)?;

        Ok(NativeChainConfig {
            account_address_prefix: self.account_address_prefix.clone(),
            validator_address_prefix: self.validator_address_prefix.clone(),
            token_denom: validate_denom(&self.token_denom)?,
            validators,
            unbonding_period: self.unbonding_period,
            staker_address,
            reward_collector_address,
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
            account_address_prefix: self.account_address_prefix.clone(),
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

#[cw_serde]
pub struct InstantiateMsg {
    pub native_chain_config: UnsafeNativeChainConfig,
    pub protocol_chain_config: UnsafeProtocolChainConfig,
    pub protocol_fee_config: UnsafeProtocolFeeConfig,

    /// Denomination of the liquid staking token (stTIA)
    pub liquid_stake_token_denom: String,

    pub batch_period: u64,
    pub monitors: Vec<String>,
}

#[cw_serde]
#[allow(clippy::large_enum_variant)]
pub enum ExecuteMsg {
    LiquidStake {
        mint_to: Option<String>,
        expected_mint_amount: Option<Uint128>,
    },
    LiquidUnstake {},
    SubmitBatch {},
    Withdraw {
        batch_id: u64,
    },
    AddValidator {
        new_validator: String,
    },
    RemoveValidator {
        validator: String,
    },
    TransferOwnership {
        new_owner: String,
    },
    AcceptOwnership {},
    RevokeOwnershipTransfer {},
    UpdateConfig {
        native_chain_config: Option<UnsafeNativeChainConfig>,
        protocol_chain_config: Option<UnsafeProtocolChainConfig>,
        protocol_fee_config: Option<UnsafeProtocolFeeConfig>,
        monitors: Option<Vec<String>>,
        batch_period: Option<u64>,
    },
    ReceiveRewards {},
    ReceiveUnstakedTokens {
        batch_id: u64,
    },
    CircuitBreaker {},
    ResumeContract {
        total_native_token: Uint128,
        total_liquid_stake_token: Uint128,
        total_reward_amount: Uint128,
    },
    RecoverPendingIbcTransfers {
        paginated: Option<bool>,
        selected_packets: Option<Vec<u64>>,
    },
    FeeWithdraw {
        amount: Uint128,
    },
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct ConfigResponse {
    pub native_chain_config: NativeChainConfig,
    pub protocol_chain_config: ProtocolChainConfig,
    pub protocol_fee_config: ProtocolFeeConfig,
    pub monitors: Vec<Addr>,
    pub liquid_stake_token_denom: String,
    pub batch_period: u64,
    pub stopped: bool,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug, Default)]
pub struct StateResponse {
    pub total_native_token: Uint128,
    pub total_liquid_stake_token: Uint128,
    pub rate: Decimal,
    pub pending_owner: String,
    pub total_reward_amount: Uint128,
    pub total_fees: Uint128,
}
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug, Default)]
pub struct BatchResponse {
    pub id: u64,
    pub batch_total_liquid_stake: Uint128,
    pub expected_native_unstaked: Uint128,
    pub received_native_unstaked: Uint128,
    pub unstake_request_count: u64,
    pub next_batch_action_time: Timestamp,
    pub status: String,
}
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug, Default)]
pub struct BatchesResponse {
    pub batches: Vec<BatchResponse>,
}
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug, Default)]
pub struct UnstakeRequestResponse {
    pub batch_id: u64,
    pub batch_total_liquid_stake: Uint128,
    pub expected_native_unstaked: Uint128,
    pub received_native_unstaked: Uint128,
    pub status: String,
    pub unstake_amount: Uint128,
    pub user: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug, Default)]
pub struct IBCQueueResponse {
    pub ibc_queue: Vec<IBCTransfer>,
}
#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug, Default)]
pub struct IBCReplyQueueResponse {
    pub ibc_queue: Vec<IbcWaitingForReply>,
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(ConfigResponse)]
    Config {},
    #[returns(StateResponse)]
    State {},
    #[returns(BatchResponse)]
    Batch { id: u64 },
    #[returns(BatchesResponse)]
    Batches {
        start_after: Option<u64>,
        limit: Option<u32>,
        status: Option<BatchStatus>,
    },
    #[returns(BatchesResponse)]
    BatchesByIds { ids: Vec<u64> },
    #[returns(BatchResponse)]
    PendingBatch {},
    #[returns(Vec<UnstakeRequest>)]
    UnstakeRequests { user: Addr },
    #[returns(Vec<UnstakeRequestResponse>)]
    AllUnstakeRequests {
        start_after: Option<u64>,
        limit: Option<u32>,
    },
    #[returns(Vec<(String, u64, Uint128)>)]
    AllUnstakeRequestsV2 {
        start_after: Option<u64>,
        limit: Option<u32>,
    },
    #[returns(IBCQueueResponse)]
    IbcQueue {
        start_after: Option<u64>,
        limit: Option<u32>,
    },
    #[returns(IBCReplyQueueResponse)]
    IbcReplyQueue {
        start_after: Option<u64>,
        limit: Option<u32>,
    },
}

#[cw_serde]
pub enum MigrateMsg {
    V0_4_18ToV0_4_20 {
        send_fees_to_treasury: bool,
    },
    V0_4_20ToV1_0_0 {
        native_account_address_prefix: String,
        native_validator_address_prefix: String,
        native_token_denom: String,
        protocol_account_address_prefix: String,
    },
}

#[cw_serde]
pub enum IBCLifecycleComplete {
    #[serde(rename = "ibc_ack")]
    IBCAck {
        /// The source channel (osmosis side) of the IBC packet
        channel: String,
        /// The sequence number that the packet was sent with
        sequence: u64,
        /// String encoded version of the ack as seen by OnAcknowledgementPacket(..)
        ack: String,
        /// Weather an ack is a success of failure according to the transfer spec
        success: bool,
    },
    #[serde(rename = "ibc_timeout")]
    IBCTimeout {
        /// The source channel (osmosis side) of the IBC packet
        channel: String,
        /// The sequence number that the packet was sent with
        sequence: u64,
    },
}

/// Message type for `sudo` entry_point
#[cw_serde]
pub enum SudoMsg {
    #[serde(rename = "ibc_lifecycle_complete")]
    IBCLifecycleComplete(IBCLifecycleComplete),
}
