#![cfg_attr(docsrs, feature(doc_cfg))]
#![allow(rustdoc::bare_urls, rustdoc::broken_intra_doc_links)]
#![forbid(unsafe_code)]
#![warn(trivial_casts, trivial_numeric_casts, unused_import_braces)]

pub mod traits;
mod type_urls;

pub use prost;
pub use prost_types::Any;

// we use tendermint.*.rs in prost instead of tendermint-rs
pub use tendermint_proto as tendermint;

/// Cosmos protobuf definitions.
pub mod cosmos {

    pub mod app {
        pub mod runtime {
            pub mod v1alpha1 {
                include!("proto/cosmos.app.runtime.v1alpha1.rs");
            }
        }
        pub mod v1alpha1 {
            include!("proto/cosmos.app.v1alpha1.rs");
        }
    }

    /// Authentication of accounts and transactions.
    pub mod auth {
        pub mod module {
            pub mod v1 {
                include!("proto/cosmos.auth.module.v1.rs");
            }
        }
        pub mod v1beta1 {
            include!("proto/cosmos.auth.v1beta1.rs");
        }
    }

    /// Granting of arbitrary privileges from one account to another.
    pub mod authz {
        pub mod module {
            pub mod v1 {
                include!("proto/cosmos.authz.module.v1.rs");
            }
        }
        pub mod v1beta1 {
            include!("proto/cosmos.authz.v1beta1.rs");
        }
    }

    pub mod autocli {
        pub mod v1 {
            include!("proto/cosmos.autocli.v1.rs");
        }
    }

    /// Balances.
    pub mod bank {
        pub mod module {
            pub mod v1 {
                include!("proto/cosmos.bank.module.v1.rs");
            }
        }
        pub mod v1beta1 {
            include!("proto/cosmos.bank.v1beta1.rs");
        }
    }

    /// Base functionality.
    pub mod base {
        /// Application BlockChain Interface (ABCI).
        ///
        /// Interface that defines the boundary between the replication engine
        /// (the blockchain), and the state machine (the application).
        pub mod abci {
            pub mod v1beta1 {
                include!("proto/cosmos.base.abci.v1beta1.rs");
            }
        }

        pub mod node {
            pub mod v1beta1 {
                include!("proto/cosmos.base.node.v1beta1.rs");
            }
        }

        /// Query support.
        pub mod query {
            pub mod v1beta1 {
                include!("proto/cosmos.base.query.v1beta1.rs");
            }
        }

        /// Reflection support.
        pub mod reflection {
            pub mod v1beta1 {
                include!("proto/cosmos.base.reflection.v1beta1.rs");
            }

            pub mod v2alpha1 {
                include!("proto/cosmos.base.reflection.v2alpha1.rs");
            }
        }

        pub mod tendermint {
            pub mod v1beta1 {
                include!("proto/cosmos.base.tendermint.v1beta1.rs");
            }
        }

        pub mod v1beta1 {
            include!("proto/cosmos.base.v1beta1.rs");
        }
    }

    pub mod circuit {
        pub mod module {
            pub mod v1 {
                include!("proto/cosmos.circuit.module.v1.rs");
            }
        }
        pub mod v1 {
            include!("proto/cosmos.circuit.v1.rs");
        }
    }

    pub mod consensus {
        pub mod module {
            pub mod v1 {
                include!("proto/cosmos.consensus.module.v1.rs");
            }
        }
        pub mod v1 {
            include!("proto/cosmos.consensus.v1.rs");
        }
    }

    /// Crisis handling
    pub mod crisis {
        pub mod module {
            pub mod v1 {
                include!("proto/cosmos.crisis.module.v1.rs");
            }
        }
        pub mod v1beta1 {
            include!("proto/cosmos.crisis.v1beta1.rs");
        }
    }

    /// Cryptographic primitives.
    pub mod crypto {
        pub mod hd {
            pub mod v1 {
                include!("proto/cosmos.crypto.hd.v1.rs");
            }
        }
        pub mod keyring {
            pub mod v1 {
                include!("proto/cosmos.crypto.keyring.v1.rs");
            }
        }

        /// Multi-signature support.
        pub mod multisig {
            include!("proto/cosmos.crypto.multisig.rs");
            pub mod v1beta1 {
                include!("proto/cosmos.crypto.multisig.v1beta1.rs");
            }
        }
        pub mod ed25519 {
            include!("proto/cosmos.crypto.ed25519.rs");
        }
        pub mod secp256k1 {
            include!("proto/cosmos.crypto.secp256k1.rs");
        }
        pub mod secp256r1 {
            include!("proto/cosmos.crypto.secp256r1.rs");
        }
    }

    /// Messages and services handling token distribution
    pub mod distribution {
        pub mod module {
            pub mod v1 {
                include!("proto/cosmos.distribution.module.v1.rs");
            }
        }
        pub mod v1beta1 {
            include!("proto/cosmos.distribution.v1beta1.rs");
        }
    }

    /// Messages and services handling evidence
    pub mod evidence {
        pub mod module {
            pub mod v1 {
                include!("proto/cosmos.evidence.module.v1.rs");
            }
        }
        pub mod v1beta1 {
            include!("proto/cosmos.evidence.v1beta1.rs");
        }
    }

    /// Allows accounts to grant fee allowances and to use fees from their accounts.
    pub mod feegrant {
        pub mod module {
            pub mod v1 {
                include!("proto/cosmos.feegrant.module.v1.rs");
            }
        }
        pub mod v1beta1 {
            include!("proto/cosmos.feegrant.v1beta1.rs");
        }
    }

    /// Messages and services handling gentx's
    pub mod genutil {
        pub mod module {
            pub mod v1 {
                include!("proto/cosmos.genutil.module.v1.rs");
            }
        }
        pub mod v1beta1 {
            include!("proto/cosmos.genutil.v1beta1.rs");
        }
    }

    /// Messages and services handling governance
    pub mod gov {
        pub mod module {
            pub mod v1 {
                include!("proto/cosmos.gov.module.v1.rs");
            }
        }
        pub mod v1 {
            include!("proto/cosmos.gov.v1.rs");
        }
        pub mod v1beta1 {
            include!("proto/cosmos.gov.v1beta1.rs");
        }
    }

    pub mod group {
        pub mod module {
            pub mod v1 {
                include!("proto/cosmos.group.module.v1.rs");
            }
        }
        pub mod v1 {
            include!("proto/cosmos.group.v1.rs");
        }
    }

    /// ICS23 protobuf definitions.
    pub mod ics23 {
        pub mod v1 {
            include!("proto/cosmos.ics23.v1.rs");
        }
    }

    /// Messages and services handling minting
    pub mod mint {
        pub mod module {
            pub mod v1 {
                include!("proto/cosmos.mint.module.v1.rs");
            }
        }
        pub mod v1beta1 {
            include!("proto/cosmos.mint.v1beta1.rs");
        }
    }

    pub mod nft {
        pub mod module {
            pub mod v1 {
                include!("proto/cosmos.nft.module.v1.rs");
            }
        }
        pub mod v1beta1 {
            include!("proto/cosmos.nft.v1beta1.rs");
        }
    }

    pub mod orm {
        pub mod module {
            pub mod v1alpha1 {
                include!("proto/cosmos.orm.module.v1alpha1.rs");
            }
        }
        pub mod query {
            pub mod v1alpha1 {
                include!("proto/cosmos.orm.query.v1alpha1.rs");
            }
        }
        pub mod v1 {
            include!("proto/cosmos.orm.v1.rs");
        }
        pub mod v1alpha1 {
            include!("proto/cosmos.orm.v1alpha1.rs");
        }
    }

    /// Messages and services handling chain parameters
    pub mod params {
        pub mod module {
            pub mod v1 {
                include!("proto/cosmos.params.module.v1.rs");
            }
        }
        pub mod v1beta1 {
            include!("proto/cosmos.params.v1beta1.rs");
        }
    }

    pub mod reflection {
        pub mod v1 {
            include!("proto/cosmos.reflection.v1.rs");
        }
    }

    /// Handling slashing parameters and unjailing
    pub mod slashing {
        pub mod module {
            pub mod v1 {
                include!("proto/cosmos.slashing.module.v1.rs");
            }
        }
        pub mod v1beta1 {
            include!("proto/cosmos.slashing.v1beta1.rs");
        }
    }

    /// Proof-of-Stake layer for public blockchains.
    pub mod staking {
        pub mod module {
            pub mod v1 {
                include!("proto/cosmos.staking.module.v1.rs");
            }
        }
        pub mod v1beta1 {
            include!("proto/cosmos.staking.v1beta1.rs");
        }
    }

    pub mod store {
        pub mod internal {
            pub mod kv {
                pub mod v1beta1 {
                    include!("proto/cosmos.store.internal.kv.v1beta1.rs");
                }
            }
        }

        pub mod snapshots {
            pub mod v1 {
                include!("proto/cosmos.store.snapshots.v1.rs");
            }
        }

        pub mod streaming {
            pub mod abci {
                include!("proto/cosmos.store.streaming.abci.rs");
            }
        }

        pub mod v1beta1 {
            include!("proto/cosmos.store.v1beta1.rs");
        }
    }

    /// Transactions.
    pub mod tx {
        pub mod config {
            pub mod v1 {
                include!("proto/cosmos.tx.config.v1.rs");
            }
        }

        /// Transaction signing support.
        pub mod signing {
            pub mod v1beta1 {
                include!("proto/cosmos.tx.signing.v1beta1.rs");
            }
        }

        pub mod v1beta1 {
            include!("proto/cosmos.tx.v1beta1.rs");
        }
    }

    /// Services for the upgrade module.
    pub mod upgrade {
        pub mod module {
            pub mod v1 {
                include!("proto/cosmos.upgrade.module.v1.rs");
            }
        }
        pub mod v1beta1 {
            include!("proto/cosmos.upgrade.v1beta1.rs");
        }
    }

    /// Services and tx's for the vesting module.
    pub mod vesting {
        pub mod module {
            pub mod v1 {
                include!("proto/cosmos.vesting.module.v1.rs");
            }
        }
        pub mod v1beta1 {
            include!("proto/cosmos.vesting.v1beta1.rs");
        }
    }
}

/// IBC protobuf definitions.
pub mod ibc {

    /// IBC applications.
    pub mod applications {
        pub mod fee {
            pub mod v1 {
                include!("proto/ibc.applications.fee.v1.rs");
            }
        }

        /// Interchain accounts support.
        pub mod interchain_accounts {
            pub mod controller {
                pub mod v1 {
                    include!("proto/ibc.applications.interchain_accounts.controller.v1.rs");
                }
            }

            pub mod genesis {
                pub mod v1 {
                    include!("proto/ibc.applications.interchain_accounts.genesis.v1.rs");
                }
            }

            pub mod host {
                pub mod v1 {
                    include!("proto/ibc.applications.interchain_accounts.host.v1.rs");
                }
            }

            pub mod v1 {
                include!("proto/ibc.applications.interchain_accounts.v1.rs");
            }
        }

        pub mod nft_transfer {
            pub mod v1 {
                include!("proto/ibc.applications.nft_transfer.v1.rs");
            }
        }

        /// Transfer support.
        pub mod transfer {
            pub mod v1 {
                include!("proto/ibc.applications.transfer.v1.rs");
            }

            pub mod v2 {
                include!("proto/ibc.applications.transfer.v2.rs");
            }
        }
    }

    /// IBC core.
    pub mod core {
        /// IBC channels.
        pub mod channel {
            pub mod v1 {
                include!("proto/ibc.core.channel.v1.rs");
            }
        }

        /// IBC client.
        pub mod client {
            pub mod v1 {
                include!("proto/ibc.core.client.v1.rs");
            }
        }

        /// IBC commitments.
        pub mod commitment {
            pub mod v1 {
                include!("proto/ibc.core.commitment.v1.rs");
            }
        }

        /// IBC connections.
        pub mod connection {
            pub mod v1 {
                include!("proto/ibc.core.connection.v1.rs");
            }
        }

        /// IBC types.
        pub mod types {
            pub mod v1 {
                include!("proto/ibc.core.types.v1.rs");
            }
        }
    }

    /// IBC light clients.
    pub mod lightclients {
        pub mod localhost {
            pub mod v1 {
                include!("proto/ibc.lightclients.localhost.v2.rs");
            }
        }
        pub mod solomachine {
            pub mod v1 {
                include!("proto/ibc.lightclients.solomachine.v2.rs");
            }

            pub mod v2 {
                include!("proto/ibc.lightclients.solomachine.v3.rs");
            }
        }
        pub mod tendermint {
            pub mod v1 {
                include!("proto/ibc.lightclients.tendermint.v1.rs");
            }
        }
        pub mod wasm {
            pub mod v1 {
                include!("proto/ibc.lightclients.wasm.v1.rs");
            }
        }
    }
}

pub mod initia {

    pub mod bank {
        pub mod v1 {
            include!("proto/initia.bank.v1.rs");
        }
    }

    pub mod distribution {
        pub mod module {
            pub mod v1 {
                include!("proto/initia.distribution.module.v1.rs");
            }
        }
        pub mod v1 {
            include!("proto/initia.distribution.v1.rs");
        }
    }

    pub mod gov {
        pub mod v1 {
            include!("proto/initia.gov.v1.rs");
        }
    }

    pub mod ibchooks {
        pub mod module {
            pub mod v1 {
                include!("proto/initia.ibchooks.module.v1.rs");
            }
        }
        pub mod v1 {
            include!("proto/initia.ibchooks.v1.rs");
        }
    }

    pub mod intertx {
        pub mod module {
            pub mod v1 {
                include!("proto/initia.intertx.module.v1.rs");
            }
        }
        pub mod v1 {
            include!("proto/initia.intertx.v1.rs");
        }
    }

    pub mod r#move {
        pub mod module {
            pub mod v1 {
                include!("proto/initia.move.module.v1.rs");
            }
        }
        pub mod v1 {
            include!("proto/initia.move.v1.rs");
        }
    }

    pub mod mstaking {
        pub mod module {
            pub mod v1 {
                include!("proto/initia.mstaking.module.v1.rs");
            }
        }
        pub mod v1 {
            include!("proto/initia.mstaking.v1.rs");
        }
    }

    pub mod reward {
        pub mod module {
            pub mod v1 {
                include!("proto/initia.reward.module.v1.rs");
            }
        }
        pub mod v1 {
            include!("proto/initia.reward.v1.rs");
        }
    }
}

pub mod celestia {
    pub mod blob {
        pub mod v1 {
            include!("proto/celestia.blob.v1.rs");
        }
    }

    pub mod core {
        pub mod v1 {
            pub mod blob {
                include!("proto/celestia.core.v1.blob.rs");
            }
        }
    }
}

pub mod sdk {
    pub mod auction {
        pub mod module {
            pub mod v1 {
                include!("proto/sdk.auction.module.v1.rs");
            }
        }
        pub mod v1 {
            include!("proto/sdk.auction.v1.rs");
        }
    }

    pub mod mempool {
        pub mod v1 {
            include!("proto/sdk.mempool.v1.rs");
        }
    }

    pub mod proposals {
        pub mod v1 {
            include!("proto/sdk.proposals.v1.rs");
        }
    }
}

pub mod capability {
    pub mod v1 {
        include!("proto/capability.v1.rs");
    }
}

pub mod cosmos_proto {
    include!("proto/cosmos_proto.rs");
}

pub mod miniwasm {
    pub mod tokenfactory {
        pub mod v1 {
            include!("proto/miniwasm.tokenfactory.v1.rs");
        }
    }
}
