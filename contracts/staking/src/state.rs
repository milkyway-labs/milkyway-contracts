use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, DepsMut, StdError, Timestamp, Uint128};
use cw_controllers::Admin;
use cw_storage_plus::{Index, IndexList, IndexedMap, Item, Map, UniqueIndex};
use milky_way::staking::Batch;

#[cw_serde]
pub struct Config {
    pub native_token_denom: String,
    pub liquid_stake_token_denom: String,
    pub treasury_address: Addr,
    pub monitors: Option<Vec<Addr>>,
    pub validators: Vec<Addr>,
    pub batch_period: u64,
    pub unbonding_period: u64,
    pub protocol_fee_config: ProtocolFeeConfig,
    pub multisig_address_config: MultisigAddressConfig,
    pub minimum_liquid_stake_amount: Uint128,
    pub ibc_channel_id: String,
    pub stopped: bool,
    pub oracle_address: Option<Addr>,
}
// TODO: PENDING - DOCS DEFINE THESE AS MAPS?
// Discuss: Do we want to add or remove any state?
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

#[cw_serde]
#[derive(Default)]
pub struct ProtocolFeeConfig {
    pub dao_treasury_fee: Uint128, // not using a fraction, fee percentage=x/100000
}

#[cw_serde]
pub struct MultisigAddressConfig {
    pub staker_address: Addr,
    pub reward_collector_address: Addr,
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

    // depr version
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
    pub amount: u128,
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
        pub amount: u128,
        pub status: PacketLifecycleStatus,
    }
}

/// In-Flight packets by (source_channel_id, sequence)
pub const INFLIGHT_PACKETS: Map<u64, ibc::IBCTransfer> = Map::new("inflight");
pub const IBC_WAITING_FOR_REPLY: Map<u64, IbcWaitingForReply> = Map::new("ibc_waiting_for_reply");
