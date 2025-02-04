/// ProposalInfo contains the metadata about a given proposal that was built by
/// the block-sdk. This is used to verify and consilidate proposal data across
/// the network.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProposalInfo {
    /// TxsByLane contains information about how each partial proposal
    /// was constructed by the block-sdk lanes.
    #[prost(map = "string, uint64", tag = "1")]
    pub txs_by_lane: ::std::collections::HashMap<::prost::alloc::string::String, u64>,
    /// MaxBlockSize corresponds to the upper bound on the size of the
    /// block that was used to construct this block proposal.
    #[prost(int64, tag = "2")]
    pub max_block_size: i64,
    /// MaxGasLimit corresponds to the upper bound on the gas limit of the
    /// block that was used to construct this block proposal.
    #[prost(uint64, tag = "3")]
    pub max_gas_limit: u64,
    /// BlockSize corresponds to the size of this block proposal.
    #[prost(int64, tag = "4")]
    pub block_size: i64,
    /// GasLimit corresponds to the gas limit of this block proposal.
    #[prost(uint64, tag = "5")]
    pub gas_limit: u64,
}
