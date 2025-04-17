use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Coin, Deps, DepsMut, StdError, Timestamp, Uint128};
use cw_controllers::Admin;
use cw_storage_plus::{Index, IndexList, IndexedMap, Item, Map, UniqueIndex};
use milky_way::staking::Batch;

use crate::error::{ContractError, ContractResult};

#[cw_serde]
pub struct Config {
    /// Config related to the chain for which we are creating
    /// the LST token.
    /// For example Celestia is the native chain of milkTIA LST token.
    pub native_chain_config: NativeChainConfig,

    /// Config related to the chain where the smart contract is deployed.
    pub protocol_chain_config: ProtocolChainConfig,

    /// Config related to the fees collected by the contract to
    /// operate the liquid staking protocol.
    pub protocol_fee_config: ProtocolFeeConfig,

    /// Denomination of the liquid staking token that have been
    /// minted through the tokenfactory module.
    pub liquid_stake_token_denom: String,

    /// Accounts that can execute the [crate::msg::ExecuteMsg::CircuitBreaker].
    pub monitors: Vec<Addr>,

    /// Time in seconds between each batch.
    pub batch_period: u64,

    /// If true, the contract is stopped and no actions are allowed.
    pub stopped: bool,
}

/// Config related to the chain for which we are creating
/// the LST token.
/// For example Celestia is the native chain of milkTIA LST token.
#[cw_serde]
pub struct NativeChainConfig {
    /// Bech32 prefix for accounts (e.g. "celestia", "initia", etc)
    pub account_address_prefix: String,

    /// Bech32 prefix for validator accounts (e.g. "celestiavaloper", "initavaloper", etc)
    pub validator_address_prefix: String,

    /// Denomination of underlying token (e.g. "utia", "uinit", etc)
    pub token_denom: String,

    /// Set of validators who will receive the delegations.
    pub validators: Vec<Addr>,

    /// The staking module's unbonding period in seconds.
    pub unbonding_period: u64,

    /// Address of the account that is performing the delegation in the native
    /// chain.
    pub staker_address: Addr,

    /// Address where the staking rewards are withdrawn.
    pub reward_collector_address: Addr,
}

/// Config related to the chain where the smart contract is deployed.
#[cw_serde]
pub struct ProtocolChainConfig {
    /// Bech32 prefix for accounts (e.g. "osmosis", "milkyway", etc)
    pub account_address_prefix: String,

    /// IBC channel id from the Protocol chain to the base chain (e.g. Osmosis -> Celestia)
    pub ibc_channel_id: String,

    /// IBC denom of the token for which we are creating the LST once is
    /// received in the chain where this contract is deployed.
    pub ibc_token_denom: String,

    /// Minimum amount of token that can be liquid staked.
    pub minimum_liquid_stake_amount: Uint128,

    /// The redemption / purchase rate oracle address
    pub oracle_address: Option<Addr>,
}

/// Config related to the fees collected by the contract to
/// operate the liquid staking protocol.
#[cw_serde]
pub struct ProtocolFeeConfig {
    pub dao_treasury_fee: Uint128, // not using a fraction, fee percentage=x/100000

    /// Address where the collected fees are sent.
    /// If this value is None, the fees will be kept in the contract.
    pub treasury_address: Option<Addr>,
}

#[cw_serde]
pub struct State {
    pub total_native_token: Uint128,
    pub total_liquid_stake_token: Uint128,
    pub pending_owner: Option<Addr>,
    pub owner_transfer_min_time: Option<Timestamp>,
    pub total_reward_amount: Uint128,
    pub rate: Uint128,
    pub total_fees: Uint128,
    pub ibc_id_counter: u64,
}

pub const CONFIG: Item<Config> = Item::new("config");
pub const ADMIN: Admin = Admin::new("admin");
pub const STATE: Item<State> = Item::new("state");
pub const BATCHES: Map<u64, Batch> = Map::new("batches");
pub const PENDING_BATCH_ID: Item<u64> = Item::new("pending_batch_id");

#[cw_serde]
pub struct UnstakeRequest {
    pub batch_id: u64,
    pub user: String,
    pub amount: Uint128,
}

pub struct UnstakeRequestIndexes<'a> {
    pub by_user: UniqueIndex<'a, (String, u64), UnstakeRequest>,
}

impl<'a> IndexList<UnstakeRequest> for UnstakeRequestIndexes<'a> {
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<UnstakeRequest>> + '_> {
        let v: Vec<&dyn Index<UnstakeRequest>> = vec![&self.by_user];
        Box::new(v.into_iter())
    }
}

pub fn unstake_requests<'a>(
) -> IndexedMap<'a, (u64, String), UnstakeRequest, UnstakeRequestIndexes<'a>> {
    let indexes = UnstakeRequestIndexes {
        by_user: UniqueIndex::new(|r| (r.user.clone(), r.batch_id), "unstake_requests_by_user"),
    };

    IndexedMap::new("unstake_requests", indexes)
}

pub fn new_unstake_request(
    deps: &mut DepsMut,
    user: String,
    batch_id: u64,
    amount: Uint128,
) -> Result<(), StdError> {
    unstake_requests().save(
        deps.storage,
        (batch_id, user.clone()),
        &UnstakeRequest {
            batch_id,
            user,
            amount,
        },
    )?;
    Ok(())
}

pub fn remove_unstake_request(
    deps: &mut DepsMut,
    user: String,
    batch_id: u64,
) -> Result<(), StdError> {
    unstake_requests()
        .remove(deps.storage, (batch_id, user.clone()))
        .unwrap();
    Ok(())
}

#[cw_serde]
pub struct IbcWaitingForReply {
    pub amount: Coin,
    pub receiver: String,
}

pub mod ibc {
    use super::*;

    #[cw_serde]
    pub enum PacketLifecycleStatus {
        Sent,
        AckSuccess,
        AckFailure,
        TimedOut,
    }

    /// A transfer packet sent by this contract that is expected to be received but
    /// needs to be tracked in case the receive fails or times-out
    #[cw_serde]
    pub struct IBCTransfer {
        pub sequence: u64,
        pub amount: Coin,
        pub receiver: String,
        pub status: PacketLifecycleStatus,
    }
}

/// In-Flight packets by (source_channel_id, sequence)
pub const INFLIGHT_PACKETS: Map<u64, ibc::IBCTransfer> = Map::new("inflight");
pub const IBC_WAITING_FOR_REPLY: Map<u64, IbcWaitingForReply> = Map::new("ibc_waiting_for_reply");

pub const MIGRATING: Item<bool> = Item::new("migrating");

/// Checks if the contract is being migrated.
pub fn assert_not_migrating(deps: Deps) -> ContractResult<()> {
    if MIGRATING.may_load(deps.storage)?.unwrap_or(false) {
        Err(ContractError::Migrating {})
    } else {
        Ok(())
    }
}
