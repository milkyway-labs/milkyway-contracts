/// BasicAllowance implements Allowance with a one-time grant of coins
/// that optionally expires. The grantee can use up to SpendLimit to cover fees.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BasicAllowance {
    /// spend_limit specifies the maximum amount of coins that can be spent
    /// by this allowance and will be updated as coins are spent. If it is
    /// empty, there is no spend limit and any amount of coins can be spent.
    #[prost(message, repeated, tag = "1")]
    pub spend_limit: ::prost::alloc::vec::Vec<super::super::base::v1beta1::Coin>,
    /// expiration specifies an optional time when this allowance expires
    #[prost(message, optional, tag = "2")]
    pub expiration: ::core::option::Option<::prost_types::Timestamp>,
}
/// PeriodicAllowance extends Allowance to allow for both a maximum cap,
/// as well as a limit per time period.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PeriodicAllowance {
    /// basic specifies a struct of `BasicAllowance`
    #[prost(message, optional, tag = "1")]
    pub basic: ::core::option::Option<BasicAllowance>,
    /// period specifies the time duration in which period_spend_limit coins can
    /// be spent before that allowance is reset
    #[prost(message, optional, tag = "2")]
    pub period: ::core::option::Option<::prost_types::Duration>,
    /// period_spend_limit specifies the maximum number of coins that can be spent
    /// in the period
    #[prost(message, repeated, tag = "3")]
    pub period_spend_limit: ::prost::alloc::vec::Vec<super::super::base::v1beta1::Coin>,
    /// period_can_spend is the number of coins left to be spent before the period_reset time
    #[prost(message, repeated, tag = "4")]
    pub period_can_spend: ::prost::alloc::vec::Vec<super::super::base::v1beta1::Coin>,
    /// period_reset is the time at which this period resets and a new one begins,
    /// it is calculated from the start time of the first transaction after the
    /// last period ended
    #[prost(message, optional, tag = "5")]
    pub period_reset: ::core::option::Option<::prost_types::Timestamp>,
}
/// AllowedMsgAllowance creates allowance only for specified message types.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AllowedMsgAllowance {
    /// allowance can be any of basic and periodic fee allowance.
    #[prost(message, optional, tag = "1")]
    pub allowance: ::core::option::Option<::prost_types::Any>,
    /// allowed_messages are the messages for which the grantee has the access.
    #[prost(string, repeated, tag = "2")]
    pub allowed_messages: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Grant is stored in the KVStore to record a grant with full context
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Grant {
    /// granter is the address of the user granting an allowance of their funds.
    #[prost(string, tag = "1")]
    pub granter: ::prost::alloc::string::String,
    /// grantee is the address of the user being granted an allowance of another user's funds.
    #[prost(string, tag = "2")]
    pub grantee: ::prost::alloc::string::String,
    /// allowance can be any of basic, periodic, allowed fee allowance.
    #[prost(message, optional, tag = "3")]
    pub allowance: ::core::option::Option<::prost_types::Any>,
}
/// GenesisState contains a set of fee allowances, persisted from the store
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    #[prost(message, repeated, tag = "1")]
    pub allowances: ::prost::alloc::vec::Vec<Grant>,
}
/// MsgGrantAllowance adds permission for Grantee to spend up to Allowance
/// of fees from the account of Granter.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgGrantAllowance {
    /// granter is the address of the user granting an allowance of their funds.
    #[prost(string, tag = "1")]
    pub granter: ::prost::alloc::string::String,
    /// grantee is the address of the user being granted an allowance of another user's funds.
    #[prost(string, tag = "2")]
    pub grantee: ::prost::alloc::string::String,
    /// allowance can be any of basic, periodic, allowed fee allowance.
    #[prost(message, optional, tag = "3")]
    pub allowance: ::core::option::Option<::prost_types::Any>,
}
/// MsgGrantAllowanceResponse defines the Msg/GrantAllowanceResponse response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgGrantAllowanceResponse {}
/// MsgRevokeAllowance removes any existing Allowance from Granter to Grantee.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRevokeAllowance {
    /// granter is the address of the user granting an allowance of their funds.
    #[prost(string, tag = "1")]
    pub granter: ::prost::alloc::string::String,
    /// grantee is the address of the user being granted an allowance of another user's funds.
    #[prost(string, tag = "2")]
    pub grantee: ::prost::alloc::string::String,
}
/// MsgRevokeAllowanceResponse defines the Msg/RevokeAllowanceResponse response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRevokeAllowanceResponse {}
/// MsgPruneAllowances prunes expired fee allowances.
///
/// Since cosmos-sdk 0.50
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgPruneAllowances {
    /// pruner is the address of the user pruning expired allowances.
    #[prost(string, tag = "1")]
    pub pruner: ::prost::alloc::string::String,
}
/// MsgPruneAllowancesResponse defines the Msg/PruneAllowancesResponse response type.
///
/// Since cosmos-sdk 0.50
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgPruneAllowancesResponse {}
/// Generated client implementations.
#[cfg(feature = "grpc")]
#[cfg_attr(docsrs, doc(cfg(feature = "grpc")))]
pub mod msg_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    /// Msg defines the feegrant msg service.
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
        /// GrantAllowance grants fee allowance to the grantee on the granter's
        /// account with the provided expiration time.
        pub async fn grant_allowance(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgGrantAllowance>,
        ) -> std::result::Result<tonic::Response<super::MsgGrantAllowanceResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/cosmos.feegrant.v1beta1.Msg/GrantAllowance");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "cosmos.feegrant.v1beta1.Msg",
                "GrantAllowance",
            ));
            self.inner.unary(req, path, codec).await
        }
        /// RevokeAllowance revokes any fee allowance of granter's account that
        /// has been granted to the grantee.
        pub async fn revoke_allowance(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgRevokeAllowance>,
        ) -> std::result::Result<tonic::Response<super::MsgRevokeAllowanceResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cosmos.feegrant.v1beta1.Msg/RevokeAllowance",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "cosmos.feegrant.v1beta1.Msg",
                "RevokeAllowance",
            ));
            self.inner.unary(req, path, codec).await
        }
        /// PruneAllowances prunes expired fee allowances, currently up to 75 at a time.
        ///
        /// Since cosmos-sdk 0.50
        pub async fn prune_allowances(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgPruneAllowances>,
        ) -> std::result::Result<tonic::Response<super::MsgPruneAllowancesResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cosmos.feegrant.v1beta1.Msg/PruneAllowances",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "cosmos.feegrant.v1beta1.Msg",
                "PruneAllowances",
            ));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// QueryAllowanceRequest is the request type for the Query/Allowance RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllowanceRequest {
    /// granter is the address of the user granting an allowance of their funds.
    #[prost(string, tag = "1")]
    pub granter: ::prost::alloc::string::String,
    /// grantee is the address of the user being granted an allowance of another user's funds.
    #[prost(string, tag = "2")]
    pub grantee: ::prost::alloc::string::String,
}
/// QueryAllowanceResponse is the response type for the Query/Allowance RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllowanceResponse {
    /// allowance is a allowance granted for grantee by granter.
    #[prost(message, optional, tag = "1")]
    pub allowance: ::core::option::Option<Grant>,
}
/// QueryAllowancesRequest is the request type for the Query/Allowances RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllowancesRequest {
    #[prost(string, tag = "1")]
    pub grantee: ::prost::alloc::string::String,
    /// pagination defines an pagination for the request.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageRequest>,
}
/// QueryAllowancesResponse is the response type for the Query/Allowances RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllowancesResponse {
    /// allowances are allowance's granted for grantee by granter.
    #[prost(message, repeated, tag = "1")]
    pub allowances: ::prost::alloc::vec::Vec<Grant>,
    /// pagination defines an pagination for the response.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageResponse>,
}
/// QueryAllowancesByGranterRequest is the request type for the Query/AllowancesByGranter RPC method.
///
/// Since: cosmos-sdk 0.46
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllowancesByGranterRequest {
    #[prost(string, tag = "1")]
    pub granter: ::prost::alloc::string::String,
    /// pagination defines an pagination for the request.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageRequest>,
}
/// QueryAllowancesByGranterResponse is the response type for the Query/AllowancesByGranter RPC method.
///
/// Since: cosmos-sdk 0.46
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllowancesByGranterResponse {
    /// allowances that have been issued by the granter.
    #[prost(message, repeated, tag = "1")]
    pub allowances: ::prost::alloc::vec::Vec<Grant>,
    /// pagination defines an pagination for the response.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageResponse>,
}
/// Generated client implementations.
#[cfg(feature = "grpc")]
#[cfg_attr(docsrs, doc(cfg(feature = "grpc")))]
pub mod query_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    /// Query defines the gRPC querier service.
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
        /// Allowance returns granted allwance to the grantee by the granter.
        pub async fn allowance(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryAllowanceRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryAllowanceResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/cosmos.feegrant.v1beta1.Query/Allowance");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "cosmos.feegrant.v1beta1.Query",
                "Allowance",
            ));
            self.inner.unary(req, path, codec).await
        }
        /// Allowances returns all the grants for the given grantee address.
        pub async fn allowances(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryAllowancesRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryAllowancesResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/cosmos.feegrant.v1beta1.Query/Allowances");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "cosmos.feegrant.v1beta1.Query",
                "Allowances",
            ));
            self.inner.unary(req, path, codec).await
        }
        /// AllowancesByGranter returns all the grants given by an address
        ///
        /// Since: cosmos-sdk 0.46
        pub async fn allowances_by_granter(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryAllowancesByGranterRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryAllowancesByGranterResponse>,
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
                "/cosmos.feegrant.v1beta1.Query/AllowancesByGranter",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "cosmos.feegrant.v1beta1.Query",
                "AllowancesByGranter",
            ));
            self.inner.unary(req, path, codec).await
        }
    }
}
