use crate::{
    contract::{CONTRACT_NAME, CONTRACT_VERSION},
    error::ContractResult,
    migrations::states::v1_0_0,
    state::{
        ibc::IBCTransfer, IbcWaitingForReply, CONFIG, IBC_WAITING_FOR_REPLY, INFLIGHT_PACKETS,
        MIGRATING,
    },
};
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Coin, DepsMut, Env, Response};
use cw2::{assert_contract_version, set_contract_version};
use cw_storage_plus::Item;

#[cw_serde]
enum MigrationStatus {
    InProgress { migrated: usize },
    Completed {},
}

#[cw_serde]
struct PacketsMigration {
    pub inflight: MigrationStatus,
    pub waiting_for_reply: MigrationStatus,
}

impl PacketsMigration {
    pub fn new() -> Self {
        PacketsMigration {
            inflight: MigrationStatus::InProgress { migrated: 0 },
            waiting_for_reply: MigrationStatus::InProgress { migrated: 0 },
        }
    }

    pub fn all_migrated(&self) -> bool {
        if let MigrationStatus::Completed {} = self.inflight {
            if let MigrationStatus::Completed {} = self.waiting_for_reply {
                return true;
            }
        }
        false
    }
}

const FROM_VERSION: &str = "1.0.0";
const TO_VERSION: &str = "1.1.0";
const PACKETS_MIGRATION_STATUS: Item<PacketsMigration> = Item::new("packets_migration_status");

pub fn migrate(deps: DepsMut, _env: Env, limit: Option<usize>) -> ContractResult<Response> {
    // Ensure that we are migrating from the correct version.
    assert_contract_version(deps.storage, CONTRACT_NAME, FROM_VERSION)?;
    let config = CONFIG.load(deps.storage)?;

    let mut limit = limit.unwrap_or(usize::MAX);
    let mut packets_migration_status = PACKETS_MIGRATION_STATUS
        .may_load(deps.storage)?
        .unwrap_or_else(PacketsMigration::new);

    if let MigrationStatus::InProgress { migrated } = packets_migration_status.inflight {
        // Get the packets to migrate
        let inflight_packets = v1_0_0::INFLIGHT_PACKETS
            .range(deps.storage, None, None, cosmwasm_std::Order::Ascending)
            .skip(migrated)
            .take(limit)
            .collect::<Result<Vec<(u64, v1_0_0::IBCTransfer)>, _>>()?;
        // Get the len since the inflight_packets Vec will be consumed
        // by the for loop
        let migrated_packets = inflight_packets.len();

        for packet in inflight_packets {
            let (key, packet) = packet;
            INFLIGHT_PACKETS.save(
                deps.storage,
                key,
                &IBCTransfer {
                    sequence: packet.sequence,
                    amount: Coin::new(packet.amount, &config.protocol_chain_config.ibc_token_denom),
                    receiver: config.native_chain_config.staker_address.to_string(),
                    status: packet.status,
                },
            )?;
        }

        // Check if we have migrated all the elements
        let migration_completed = v1_0_0::INFLIGHT_PACKETS
            .range(deps.storage, None, None, cosmwasm_std::Order::Ascending)
            .nth(migrated + limit)
            .is_none();
        if migration_completed {
            packets_migration_status.inflight = MigrationStatus::Completed {}
        } else {
            packets_migration_status.inflight = MigrationStatus::InProgress {
                migrated: migrated + migrated_packets,
            }
        }

        // Update the limit
        limit -= migrated_packets;
    }

    if let MigrationStatus::InProgress { migrated } = packets_migration_status.waiting_for_reply {
        // Migrate the ibc messages waiting for reply
        let waiting_for_reply_packets = v1_0_0::IBC_WAITING_FOR_REPLY
            .range(deps.storage, None, None, cosmwasm_std::Order::Ascending)
            .skip(migrated)
            .take(limit)
            .collect::<Result<Vec<(u64, v1_0_0::IbcWaitingForReply)>, _>>()?;
        // Get the len since the waiting_for_reply_packets Vec will be consumed
        // by the for loop
        let migrated_packets = waiting_for_reply_packets.len();

        for packet in waiting_for_reply_packets {
            let (key, waiting_for_reply) = packet;
            IBC_WAITING_FOR_REPLY.save(
                deps.storage,
                key,
                &IbcWaitingForReply {
                    amount: Coin::new(
                        waiting_for_reply.amount,
                        &config.protocol_chain_config.ibc_token_denom,
                    ),
                    receiver: config.native_chain_config.staker_address.to_string(),
                },
            )?;
        }

        // Check if we have migrated all the elements
        let migration_completed = v1_0_0::IBC_WAITING_FOR_REPLY
            .range(deps.storage, None, None, cosmwasm_std::Order::Ascending)
            .nth(migrated + limit)
            .is_none();
        if migration_completed {
            packets_migration_status.waiting_for_reply = MigrationStatus::Completed {}
        } else {
            packets_migration_status.waiting_for_reply = MigrationStatus::InProgress {
                migrated: migrated + migrated_packets,
            }
        }
    }

    // Check if we have completed the migration
    if packets_migration_status.all_migrated() {
        PACKETS_MIGRATION_STATUS.remove(deps.storage);
        MIGRATING.save(deps.storage, &false)?;
        set_contract_version(deps.storage, CONTRACT_NAME, TO_VERSION)?;
    } else {
        PACKETS_MIGRATION_STATUS.save(deps.storage, &packets_migration_status)?;
        MIGRATING.save(deps.storage, &true)?;
    }

    Ok(Response::new()
        .add_attribute("action", "migrate")
        .add_attribute("from_version", FROM_VERSION)
        .add_attribute("to_version", CONTRACT_VERSION)
        .add_attribute(
            "completed",
            packets_migration_status.all_migrated().to_string(),
        ))
}
