/// Block is tendermint type Block, with the Header proposer address
/// field converted to bech32 string.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Block {
    #[prost(message, optional, tag = "1")]
    pub header: ::core::option::Option<Header>,
    #[prost(message, optional, tag = "2")]
    pub data: ::core::option::Option<tendermint_proto::types::Data>,
    #[prost(message, optional, tag = "3")]
    pub evidence: ::core::option::Option<tendermint_proto::types::EvidenceList>,
    #[prost(message, optional, tag = "4")]
    pub last_commit: ::core::option::Option<tendermint_proto::types::Commit>,
}
/// Header defines the structure of a Tendermint block header.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Header {
    /// basic block info
    #[prost(message, optional, tag = "1")]
    pub version: ::core::option::Option<tendermint_proto::version::Consensus>,
    #[prost(string, tag = "2")]
    pub chain_id: ::prost::alloc::string::String,
    #[prost(int64, tag = "3")]
    pub height: i64,
    #[prost(message, optional, tag = "4")]
    pub time: ::core::option::Option<::prost_types::Timestamp>,
    /// prev block info
    #[prost(message, optional, tag = "5")]
    pub last_block_id: ::core::option::Option<tendermint_proto::types::BlockId>,
    /// hashes of block data
    ///
    /// commit from validators from the last block
    #[prost(bytes = "vec", tag = "6")]
    pub last_commit_hash: ::prost::alloc::vec::Vec<u8>,
    /// transactions
    #[prost(bytes = "vec", tag = "7")]
    pub data_hash: ::prost::alloc::vec::Vec<u8>,
    /// hashes from the app output from the prev block
    ///
    /// validators for the current block
    #[prost(bytes = "vec", tag = "8")]
    pub validators_hash: ::prost::alloc::vec::Vec<u8>,
    /// validators for the next block
    #[prost(bytes = "vec", tag = "9")]
    pub next_validators_hash: ::prost::alloc::vec::Vec<u8>,
    /// consensus params for current block
    #[prost(bytes = "vec", tag = "10")]
    pub consensus_hash: ::prost::alloc::vec::Vec<u8>,
    /// state after txs from the previous block
    #[prost(bytes = "vec", tag = "11")]
    pub app_hash: ::prost::alloc::vec::Vec<u8>,
    /// root hash of all results from the txs from the previous block
    #[prost(bytes = "vec", tag = "12")]
    pub last_results_hash: ::prost::alloc::vec::Vec<u8>,
    /// consensus info
    ///
    /// evidence included in the block
    #[prost(bytes = "vec", tag = "13")]
    pub evidence_hash: ::prost::alloc::vec::Vec<u8>,
    /// proposer_address is the original block proposer address, formatted as a Bech32 string.
    /// In Tendermint, this type is `bytes`, but in the SDK, we convert it to a Bech32 string
    /// for better UX.
    ///
    /// original proposer of the block
    #[prost(string, tag = "14")]
    pub proposer_address: ::prost::alloc::string::String,
}
/// GetValidatorSetByHeightRequest is the request type for the Query/GetValidatorSetByHeight RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetValidatorSetByHeightRequest {
    #[prost(int64, tag = "1")]
    pub height: i64,
    /// pagination defines an pagination for the request.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::query::v1beta1::PageRequest>,
}
/// GetValidatorSetByHeightResponse is the response type for the Query/GetValidatorSetByHeight RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetValidatorSetByHeightResponse {
    #[prost(int64, tag = "1")]
    pub block_height: i64,
    #[prost(message, repeated, tag = "2")]
    pub validators: ::prost::alloc::vec::Vec<Validator>,
    /// pagination defines an pagination for the response.
    #[prost(message, optional, tag = "3")]
    pub pagination: ::core::option::Option<super::super::query::v1beta1::PageResponse>,
}
/// GetLatestValidatorSetRequest is the request type for the Query/GetValidatorSetByHeight RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLatestValidatorSetRequest {
    /// pagination defines an pagination for the request.
    #[prost(message, optional, tag = "1")]
    pub pagination: ::core::option::Option<super::super::query::v1beta1::PageRequest>,
}
/// GetLatestValidatorSetResponse is the response type for the Query/GetValidatorSetByHeight RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLatestValidatorSetResponse {
    #[prost(int64, tag = "1")]
    pub block_height: i64,
    #[prost(message, repeated, tag = "2")]
    pub validators: ::prost::alloc::vec::Vec<Validator>,
    /// pagination defines an pagination for the response.
    #[prost(message, optional, tag = "3")]
    pub pagination: ::core::option::Option<super::super::query::v1beta1::PageResponse>,
}
/// Validator is the type for the validator-set.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Validator {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub pub_key: ::core::option::Option<::prost_types::Any>,
    #[prost(int64, tag = "3")]
    pub voting_power: i64,
    #[prost(int64, tag = "4")]
    pub proposer_priority: i64,
}
/// GetBlockByHeightRequest is the request type for the Query/GetBlockByHeight RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBlockByHeightRequest {
    #[prost(int64, tag = "1")]
    pub height: i64,
}
/// GetBlockByHeightResponse is the response type for the Query/GetBlockByHeight RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBlockByHeightResponse {
    #[prost(message, optional, tag = "1")]
    pub block_id: ::core::option::Option<tendermint_proto::types::BlockId>,
    /// Deprecated: please use `sdk_block` instead
    #[prost(message, optional, tag = "2")]
    pub block: ::core::option::Option<tendermint_proto::types::Block>,
    /// Since: cosmos-sdk 0.47
    #[prost(message, optional, tag = "3")]
    pub sdk_block: ::core::option::Option<Block>,
}
/// GetLatestBlockRequest is the request type for the Query/GetLatestBlock RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLatestBlockRequest {}
/// GetLatestBlockResponse is the response type for the Query/GetLatestBlock RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLatestBlockResponse {
    #[prost(message, optional, tag = "1")]
    pub block_id: ::core::option::Option<tendermint_proto::types::BlockId>,
    /// Deprecated: please use `sdk_block` instead
    #[prost(message, optional, tag = "2")]
    pub block: ::core::option::Option<tendermint_proto::types::Block>,
    /// Since: cosmos-sdk 0.47
    #[prost(message, optional, tag = "3")]
    pub sdk_block: ::core::option::Option<Block>,
}
/// GetSyncingRequest is the request type for the Query/GetSyncing RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSyncingRequest {}
/// GetSyncingResponse is the response type for the Query/GetSyncing RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetSyncingResponse {
    #[prost(bool, tag = "1")]
    pub syncing: bool,
}
/// GetNodeInfoRequest is the request type for the Query/GetNodeInfo RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetNodeInfoRequest {}
/// GetNodeInfoResponse is the response type for the Query/GetNodeInfo RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetNodeInfoResponse {
    #[prost(message, optional, tag = "1")]
    pub default_node_info: ::core::option::Option<tendermint_proto::p2p::DefaultNodeInfo>,
    #[prost(message, optional, tag = "2")]
    pub application_version: ::core::option::Option<VersionInfo>,
}
/// VersionInfo is the type for the GetNodeInfoResponse message.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VersionInfo {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub app_name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub version: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub git_commit: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub build_tags: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub go_version: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "7")]
    pub build_deps: ::prost::alloc::vec::Vec<Module>,
    /// Since: cosmos-sdk 0.43
    #[prost(string, tag = "8")]
    pub cosmos_sdk_version: ::prost::alloc::string::String,
}
/// Module is the type for VersionInfo
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Module {
    /// module path
    #[prost(string, tag = "1")]
    pub path: ::prost::alloc::string::String,
    /// module version
    #[prost(string, tag = "2")]
    pub version: ::prost::alloc::string::String,
    /// checksum
    #[prost(string, tag = "3")]
    pub sum: ::prost::alloc::string::String,
}
/// ABCIQueryRequest defines the request structure for the ABCIQuery gRPC query.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AbciQueryRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    #[prost(string, tag = "2")]
    pub path: ::prost::alloc::string::String,
    #[prost(int64, tag = "3")]
    pub height: i64,
    #[prost(bool, tag = "4")]
    pub prove: bool,
}
/// ABCIQueryResponse defines the response structure for the ABCIQuery gRPC query.
///
/// Note: This type is a duplicate of the ResponseQuery proto type defined in
/// Tendermint.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AbciQueryResponse {
    #[prost(uint32, tag = "1")]
    pub code: u32,
    /// nondeterministic
    #[prost(string, tag = "3")]
    pub log: ::prost::alloc::string::String,
    /// nondeterministic
    #[prost(string, tag = "4")]
    pub info: ::prost::alloc::string::String,
    #[prost(int64, tag = "5")]
    pub index: i64,
    #[prost(bytes = "vec", tag = "6")]
    pub key: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "7")]
    pub value: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "8")]
    pub proof_ops: ::core::option::Option<ProofOps>,
    #[prost(int64, tag = "9")]
    pub height: i64,
    #[prost(string, tag = "10")]
    pub codespace: ::prost::alloc::string::String,
}
/// ProofOp defines an operation used for calculating Merkle root. The data could
/// be arbitrary format, providing necessary data for example neighbouring node
/// hash.
///
/// Note: This type is a duplicate of the ProofOp proto type defined in Tendermint.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProofOp {
    #[prost(string, tag = "1")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "2")]
    pub key: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "3")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
/// ProofOps is Merkle proof defined by the list of ProofOps.
///
/// Note: This type is a duplicate of the ProofOps proto type defined in Tendermint.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProofOps {
    #[prost(message, repeated, tag = "1")]
    pub ops: ::prost::alloc::vec::Vec<ProofOp>,
}
/// Generated client implementations.
#[cfg(feature = "grpc")]
#[cfg_attr(docsrs, doc(cfg(feature = "grpc")))]
pub mod service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    /// Service defines the gRPC querier service for tendermint queries.
    #[derive(Debug, Clone)]
    pub struct ServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    #[cfg(feature = "grpc-transport")]
    #[cfg_attr(docsrs, doc(cfg(feature = "grpc-transport")))]
    impl ServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> ServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> ServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            ServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// GetNodeInfo queries the current node info.
        pub async fn get_node_info(
            &mut self,
            request: impl tonic::IntoRequest<super::GetNodeInfoRequest>,
        ) -> std::result::Result<tonic::Response<super::GetNodeInfoResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cosmos.base.tendermint.v1beta1.Service/GetNodeInfo",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "cosmos.base.tendermint.v1beta1.Service",
                "GetNodeInfo",
            ));
            self.inner.unary(req, path, codec).await
        }
        /// GetSyncing queries node syncing.
        pub async fn get_syncing(
            &mut self,
            request: impl tonic::IntoRequest<super::GetSyncingRequest>,
        ) -> std::result::Result<tonic::Response<super::GetSyncingResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cosmos.base.tendermint.v1beta1.Service/GetSyncing",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "cosmos.base.tendermint.v1beta1.Service",
                "GetSyncing",
            ));
            self.inner.unary(req, path, codec).await
        }
        /// GetLatestBlock returns the latest block.
        pub async fn get_latest_block(
            &mut self,
            request: impl tonic::IntoRequest<super::GetLatestBlockRequest>,
        ) -> std::result::Result<tonic::Response<super::GetLatestBlockResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cosmos.base.tendermint.v1beta1.Service/GetLatestBlock",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "cosmos.base.tendermint.v1beta1.Service",
                "GetLatestBlock",
            ));
            self.inner.unary(req, path, codec).await
        }
        /// GetBlockByHeight queries block for given height.
        pub async fn get_block_by_height(
            &mut self,
            request: impl tonic::IntoRequest<super::GetBlockByHeightRequest>,
        ) -> std::result::Result<tonic::Response<super::GetBlockByHeightResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cosmos.base.tendermint.v1beta1.Service/GetBlockByHeight",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "cosmos.base.tendermint.v1beta1.Service",
                "GetBlockByHeight",
            ));
            self.inner.unary(req, path, codec).await
        }
        /// GetLatestValidatorSet queries latest validator-set.
        pub async fn get_latest_validator_set(
            &mut self,
            request: impl tonic::IntoRequest<super::GetLatestValidatorSetRequest>,
        ) -> std::result::Result<tonic::Response<super::GetLatestValidatorSetResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cosmos.base.tendermint.v1beta1.Service/GetLatestValidatorSet",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "cosmos.base.tendermint.v1beta1.Service",
                "GetLatestValidatorSet",
            ));
            self.inner.unary(req, path, codec).await
        }
        /// GetValidatorSetByHeight queries validator-set at a given height.
        pub async fn get_validator_set_by_height(
            &mut self,
            request: impl tonic::IntoRequest<super::GetValidatorSetByHeightRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetValidatorSetByHeightResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cosmos.base.tendermint.v1beta1.Service/GetValidatorSetByHeight",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "cosmos.base.tendermint.v1beta1.Service",
                "GetValidatorSetByHeight",
            ));
            self.inner.unary(req, path, codec).await
        }
        /// ABCIQuery defines a query handler that supports ABCI queries directly to the
        /// application, bypassing Tendermint completely. The ABCI query must contain
        /// a valid and supported path, including app, custom, p2p, and store.
        ///
        /// Since: cosmos-sdk 0.46
        pub async fn abci_query(
            &mut self,
            request: impl tonic::IntoRequest<super::AbciQueryRequest>,
        ) -> std::result::Result<tonic::Response<super::AbciQueryResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cosmos.base.tendermint.v1beta1.Service/ABCIQuery",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "cosmos.base.tendermint.v1beta1.Service",
                "ABCIQuery",
            ));
            self.inner.unary(req, path, codec).await
        }
    }
}
