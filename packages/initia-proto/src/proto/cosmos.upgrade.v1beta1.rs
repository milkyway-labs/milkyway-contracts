/// Plan specifies information about a planned upgrade and when it should occur.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Plan {
    /// Sets the name for the upgrade. This name will be used by the upgraded
    /// version of the software to apply any special "on-upgrade" commands during
    /// the first BeginBlock method after the upgrade is applied. It is also used
    /// to detect whether a software version can handle a given upgrade. If no
    /// upgrade handler with this name has been set in the software, it will be
    /// assumed that the software is out-of-date when the upgrade Time or Height is
    /// reached and the software will exit.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// Deprecated: Time based upgrades have been deprecated. Time based upgrade logic
    /// has been removed from the SDK.
    /// If this field is not empty, an error will be thrown.
    #[deprecated]
    #[prost(message, optional, tag = "2")]
    pub time: ::core::option::Option<::prost_types::Timestamp>,
    /// The height at which the upgrade must be performed.
    #[prost(int64, tag = "3")]
    pub height: i64,
    /// Any application specific upgrade info to be included on-chain
    /// such as a git commit that validators could automatically upgrade to
    #[prost(string, tag = "4")]
    pub info: ::prost::alloc::string::String,
    /// Deprecated: UpgradedClientState field has been deprecated. IBC upgrade logic has been
    /// moved to the IBC module in the sub module 02-client.
    /// If this field is not empty, an error will be thrown.
    #[deprecated]
    #[prost(message, optional, tag = "5")]
    pub upgraded_client_state: ::core::option::Option<::prost_types::Any>,
}
/// SoftwareUpgradeProposal is a gov Content type for initiating a software
/// upgrade.
/// Deprecated: This legacy proposal is deprecated in favor of Msg-based gov
/// proposals, see MsgSoftwareUpgrade.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SoftwareUpgradeProposal {
    /// title of the proposal
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// description of the proposal
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    /// plan of the proposal
    #[prost(message, optional, tag = "3")]
    pub plan: ::core::option::Option<Plan>,
}
/// CancelSoftwareUpgradeProposal is a gov Content type for cancelling a software
/// upgrade.
/// Deprecated: This legacy proposal is deprecated in favor of Msg-based gov
/// proposals, see MsgCancelUpgrade.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CancelSoftwareUpgradeProposal {
    /// title of the proposal
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    /// description of the proposal
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
}
/// ModuleVersion specifies a module and its consensus version.
///
/// Since: cosmos-sdk 0.43
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModuleVersion {
    /// name of the app module
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
    /// consensus version of the app module
    #[prost(uint64, tag = "2")]
    pub version: u64,
}
/// MsgSoftwareUpgrade is the Msg/SoftwareUpgrade request type.
///
/// Since: cosmos-sdk 0.46
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSoftwareUpgrade {
    /// authority is the address that controls the module (defaults to x/gov unless overwritten).
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// plan is the upgrade plan.
    #[prost(message, optional, tag = "2")]
    pub plan: ::core::option::Option<Plan>,
}
/// MsgSoftwareUpgradeResponse is the Msg/SoftwareUpgrade response type.
///
/// Since: cosmos-sdk 0.46
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSoftwareUpgradeResponse {}
/// MsgCancelUpgrade is the Msg/CancelUpgrade request type.
///
/// Since: cosmos-sdk 0.46
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCancelUpgrade {
    /// authority is the address that controls the module (defaults to x/gov unless overwritten).
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
}
/// MsgCancelUpgradeResponse is the Msg/CancelUpgrade response type.
///
/// Since: cosmos-sdk 0.46
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCancelUpgradeResponse {}
/// Generated client implementations.
#[cfg(feature = "grpc")]
#[cfg_attr(docsrs, doc(cfg(feature = "grpc")))]
pub mod msg_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    /// Msg defines the upgrade Msg service.
    #[derive(Debug, Clone)]
    pub struct MsgClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    #[cfg(feature = "grpc-transport")]
    #[cfg_attr(docsrs, doc(cfg(feature = "grpc-transport")))]
    impl MsgClient<tonic::transport::Channel> {
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
    impl<T> MsgClient<T>
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
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> MsgClient<InterceptedService<T, F>>
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
            MsgClient::new(InterceptedService::new(inner, interceptor))
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
        /// SoftwareUpgrade is a governance operation for initiating a software upgrade.
        ///
        /// Since: cosmos-sdk 0.46
        pub async fn software_upgrade(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgSoftwareUpgrade>,
        ) -> std::result::Result<tonic::Response<super::MsgSoftwareUpgradeResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/cosmos.upgrade.v1beta1.Msg/SoftwareUpgrade");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "cosmos.upgrade.v1beta1.Msg",
                "SoftwareUpgrade",
            ));
            self.inner.unary(req, path, codec).await
        }
        /// CancelUpgrade is a governance operation for cancelling a previously
        /// approved software upgrade.
        ///
        /// Since: cosmos-sdk 0.46
        pub async fn cancel_upgrade(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgCancelUpgrade>,
        ) -> std::result::Result<tonic::Response<super::MsgCancelUpgradeResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/cosmos.upgrade.v1beta1.Msg/CancelUpgrade");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "cosmos.upgrade.v1beta1.Msg",
                "CancelUpgrade",
            ));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// QueryCurrentPlanRequest is the request type for the Query/CurrentPlan RPC
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryCurrentPlanRequest {}
/// QueryCurrentPlanResponse is the response type for the Query/CurrentPlan RPC
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryCurrentPlanResponse {
    /// plan is the current upgrade plan.
    #[prost(message, optional, tag = "1")]
    pub plan: ::core::option::Option<Plan>,
}
/// QueryCurrentPlanRequest is the request type for the Query/AppliedPlan RPC
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAppliedPlanRequest {
    /// name is the name of the applied plan to query for.
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// QueryAppliedPlanResponse is the response type for the Query/AppliedPlan RPC
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAppliedPlanResponse {
    /// height is the block height at which the plan was applied.
    #[prost(int64, tag = "1")]
    pub height: i64,
}
/// QueryUpgradedConsensusStateRequest is the request type for the Query/UpgradedConsensusState
/// RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryUpgradedConsensusStateRequest {
    /// last height of the current chain must be sent in request
    /// as this is the height under which next consensus state is stored
    #[prost(int64, tag = "1")]
    pub last_height: i64,
}
/// QueryUpgradedConsensusStateResponse is the response type for the Query/UpgradedConsensusState
/// RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryUpgradedConsensusStateResponse {
    /// Since: cosmos-sdk 0.43
    #[prost(bytes = "vec", tag = "2")]
    pub upgraded_consensus_state: ::prost::alloc::vec::Vec<u8>,
}
/// QueryModuleVersionsRequest is the request type for the Query/ModuleVersions
/// RPC method.
///
/// Since: cosmos-sdk 0.43
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryModuleVersionsRequest {
    /// module_name is a field to query a specific module
    /// consensus version from state. Leaving this empty will
    /// fetch the full list of module versions from state
    #[prost(string, tag = "1")]
    pub module_name: ::prost::alloc::string::String,
}
/// QueryModuleVersionsResponse is the response type for the Query/ModuleVersions
/// RPC method.
///
/// Since: cosmos-sdk 0.43
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryModuleVersionsResponse {
    /// module_versions is a list of module names with their consensus versions.
    #[prost(message, repeated, tag = "1")]
    pub module_versions: ::prost::alloc::vec::Vec<ModuleVersion>,
}
/// QueryAuthorityRequest is the request type for Query/Authority
///
/// Since: cosmos-sdk 0.46
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAuthorityRequest {}
/// QueryAuthorityResponse is the response type for Query/Authority
///
/// Since: cosmos-sdk 0.46
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAuthorityResponse {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
/// Generated client implementations.
#[cfg(feature = "grpc")]
#[cfg_attr(docsrs, doc(cfg(feature = "grpc")))]
pub mod query_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    /// Query defines the gRPC upgrade querier service.
    #[derive(Debug, Clone)]
    pub struct QueryClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    #[cfg(feature = "grpc-transport")]
    #[cfg_attr(docsrs, doc(cfg(feature = "grpc-transport")))]
    impl QueryClient<tonic::transport::Channel> {
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
    impl<T> QueryClient<T>
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
        ) -> QueryClient<InterceptedService<T, F>>
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
            QueryClient::new(InterceptedService::new(inner, interceptor))
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
        /// CurrentPlan queries the current upgrade plan.
        pub async fn current_plan(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryCurrentPlanRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryCurrentPlanResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/cosmos.upgrade.v1beta1.Query/CurrentPlan");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "cosmos.upgrade.v1beta1.Query",
                "CurrentPlan",
            ));
            self.inner.unary(req, path, codec).await
        }
        /// AppliedPlan queries a previously applied upgrade plan by its name.
        pub async fn applied_plan(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryAppliedPlanRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryAppliedPlanResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/cosmos.upgrade.v1beta1.Query/AppliedPlan");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "cosmos.upgrade.v1beta1.Query",
                "AppliedPlan",
            ));
            self.inner.unary(req, path, codec).await
        }
        /// UpgradedConsensusState queries the consensus state that will serve
        /// as a trusted kernel for the next version of this chain. It will only be
        /// stored at the last height of this chain.
        /// UpgradedConsensusState RPC not supported with legacy querier
        /// This rpc is deprecated now that IBC has its own replacement
        /// (https://github.com/cosmos/ibc-go/blob/2c880a22e9f9cc75f62b527ca94aa75ce1106001/proto/ibc/core/client/v1/query.proto#L54)
        pub async fn upgraded_consensus_state(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryUpgradedConsensusStateRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryUpgradedConsensusStateResponse>,
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
                "/cosmos.upgrade.v1beta1.Query/UpgradedConsensusState",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "cosmos.upgrade.v1beta1.Query",
                "UpgradedConsensusState",
            ));
            self.inner.unary(req, path, codec).await
        }
        /// ModuleVersions queries the list of module versions from state.
        ///
        /// Since: cosmos-sdk 0.43
        pub async fn module_versions(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryModuleVersionsRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryModuleVersionsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cosmos.upgrade.v1beta1.Query/ModuleVersions",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "cosmos.upgrade.v1beta1.Query",
                "ModuleVersions",
            ));
            self.inner.unary(req, path, codec).await
        }
        /// Returns the account with authority to conduct upgrades
        ///
        /// Since: cosmos-sdk 0.46
        pub async fn authority(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryAuthorityRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryAuthorityResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/cosmos.upgrade.v1beta1.Query/Authority");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("cosmos.upgrade.v1beta1.Query", "Authority"));
            self.inner.unary(req, path, codec).await
        }
    }
}
