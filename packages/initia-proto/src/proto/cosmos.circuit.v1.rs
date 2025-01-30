/// Permissions are the permissions that an account has to trip
/// or reset the circuit breaker.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Permissions {
    /// level is the level of permissions granted to this account.
    #[prost(enumeration = "permissions::Level", tag = "1")]
    pub level: i32,
    /// limit_type_urls is used with LEVEL_SOME_MSGS to limit the lists of Msg type
    /// URLs that the account can trip. It is an error to use limit_type_urls with
    /// a level other than LEVEL_SOME_MSGS.
    #[prost(string, repeated, tag = "2")]
    pub limit_type_urls: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Nested message and enum types in `Permissions`.
pub mod permissions {
    /// Level is the permission level.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Level {
        /// LEVEL_NONE_UNSPECIFIED indicates that the account will have no circuit
        /// breaker permissions.
        NoneUnspecified = 0,
        /// LEVEL_SOME_MSGS indicates that the account will have permission to
        /// trip or reset the circuit breaker for some Msg type URLs. If this level
        /// is chosen, a non-empty list of Msg type URLs must be provided in
        /// limit_type_urls.
        SomeMsgs = 1,
        /// LEVEL_ALL_MSGS indicates that the account can trip or reset the circuit
        /// breaker for Msg's of all type URLs.
        AllMsgs = 2,
        /// LEVEL_SUPER_ADMIN indicates that the account can take all circuit breaker
        /// actions and can grant permissions to other accounts.
        SuperAdmin = 3,
    }
    impl Level {
        /// String value of the enum field names used in the ProtoBuf definition.
        ///
        /// The values are not transformed in any way and thus are considered stable
        /// (if the ProtoBuf definition does not change) and safe for programmatic use.
        pub fn as_str_name(&self) -> &'static str {
            match self {
                Level::NoneUnspecified => "LEVEL_NONE_UNSPECIFIED",
                Level::SomeMsgs => "LEVEL_SOME_MSGS",
                Level::AllMsgs => "LEVEL_ALL_MSGS",
                Level::SuperAdmin => "LEVEL_SUPER_ADMIN",
            }
        }
        /// Creates an enum from field names used in the ProtoBuf definition.
        pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
            match value {
                "LEVEL_NONE_UNSPECIFIED" => Some(Self::NoneUnspecified),
                "LEVEL_SOME_MSGS" => Some(Self::SomeMsgs),
                "LEVEL_ALL_MSGS" => Some(Self::AllMsgs),
                "LEVEL_SUPER_ADMIN" => Some(Self::SuperAdmin),
                _ => None,
            }
        }
    }
}
/// GenesisAccountPermissions is the account permissions for the circuit breaker in genesis
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisAccountPermissions {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub permissions: ::core::option::Option<Permissions>,
}
/// GenesisState is the state that must be provided at genesis.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    #[prost(message, repeated, tag = "1")]
    pub account_permissions: ::prost::alloc::vec::Vec<GenesisAccountPermissions>,
    #[prost(string, repeated, tag = "2")]
    pub disabled_type_urls: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// MsgAuthorizeCircuitBreaker defines the Msg/AuthorizeCircuitBreaker request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgAuthorizeCircuitBreaker {
    /// granter is the granter of the circuit breaker permissions and must have
    /// LEVEL_SUPER_ADMIN.
    #[prost(string, tag = "1")]
    pub granter: ::prost::alloc::string::String,
    /// grantee is the account authorized with the provided permissions.
    #[prost(string, tag = "2")]
    pub grantee: ::prost::alloc::string::String,
    /// permissions are the circuit breaker permissions that the grantee receives.
    /// These will overwrite any existing permissions. LEVEL_NONE_UNSPECIFIED can
    /// be specified to revoke all permissions.
    #[prost(message, optional, tag = "3")]
    pub permissions: ::core::option::Option<Permissions>,
}
/// MsgAuthorizeCircuitBreakerResponse defines the Msg/AuthorizeCircuitBreaker response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgAuthorizeCircuitBreakerResponse {
    #[prost(bool, tag = "1")]
    pub success: bool,
}
/// MsgTripCircuitBreaker defines the Msg/TripCircuitBreaker request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgTripCircuitBreaker {
    /// authority is the account authorized to trip the circuit breaker.
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// msg_type_urls specifies a list of type URLs to immediately stop processing.
    /// IF IT IS LEFT EMPTY, ALL MSG PROCESSING WILL STOP IMMEDIATELY.
    /// This value is validated against the authority's permissions and if the
    /// authority does not have permissions to trip the specified msg type URLs
    /// (or all URLs), the operation will fail.
    #[prost(string, repeated, tag = "2")]
    pub msg_type_urls: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// MsgTripCircuitBreakerResponse defines the Msg/TripCircuitBreaker response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgTripCircuitBreakerResponse {
    #[prost(bool, tag = "1")]
    pub success: bool,
}
/// MsgResetCircuitBreaker defines the Msg/ResetCircuitBreaker request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgResetCircuitBreaker {
    /// authority is the account authorized to trip or reset the circuit breaker.
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// msg_type_urls specifies a list of Msg type URLs to resume processing. If
    /// it is left empty all Msg processing for type URLs that the account is
    /// authorized to trip will resume.
    #[prost(string, repeated, tag = "3")]
    pub msg_type_urls: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// MsgResetCircuitBreakerResponse defines the Msg/ResetCircuitBreaker response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgResetCircuitBreakerResponse {
    #[prost(bool, tag = "1")]
    pub success: bool,
}
/// Generated client implementations.
#[cfg(feature = "grpc")]
#[cfg_attr(docsrs, doc(cfg(feature = "grpc")))]
pub mod msg_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    /// Msg defines the circuit Msg service.
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
        /// AuthorizeCircuitBreaker allows a super-admin to grant (or revoke) another
        /// account's circuit breaker permissions.
        pub async fn authorize_circuit_breaker(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgAuthorizeCircuitBreaker>,
        ) -> std::result::Result<
            tonic::Response<super::MsgAuthorizeCircuitBreakerResponse>,
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
                "/cosmos.circuit.v1.Msg/AuthorizeCircuitBreaker",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "cosmos.circuit.v1.Msg",
                "AuthorizeCircuitBreaker",
            ));
            self.inner.unary(req, path, codec).await
        }
        /// TripCircuitBreaker pauses processing of Msg's in the state machine.
        pub async fn trip_circuit_breaker(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgTripCircuitBreaker>,
        ) -> std::result::Result<tonic::Response<super::MsgTripCircuitBreakerResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/cosmos.circuit.v1.Msg/TripCircuitBreaker");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "cosmos.circuit.v1.Msg",
                "TripCircuitBreaker",
            ));
            self.inner.unary(req, path, codec).await
        }
        /// ResetCircuitBreaker resumes processing of Msg's in the state machine that
        /// have been been paused using TripCircuitBreaker.
        pub async fn reset_circuit_breaker(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgResetCircuitBreaker>,
        ) -> std::result::Result<
            tonic::Response<super::MsgResetCircuitBreakerResponse>,
            tonic::Status,
        > {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/cosmos.circuit.v1.Msg/ResetCircuitBreaker");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "cosmos.circuit.v1.Msg",
                "ResetCircuitBreaker",
            ));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// QueryAccountRequest is the request type for the Query/Account RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAccountRequest {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
/// AccountResponse is the response type for the Query/Account RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountResponse {
    #[prost(message, optional, tag = "1")]
    pub permission: ::core::option::Option<Permissions>,
}
/// QueryAccountsRequest is the request type for the Query/Accounts RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAccountsRequest {
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "1")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageRequest>,
}
/// AccountsResponse is the response type for the Query/Accounts RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountsResponse {
    #[prost(message, repeated, tag = "1")]
    pub accounts: ::prost::alloc::vec::Vec<GenesisAccountPermissions>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageResponse>,
}
/// QueryDisableListRequest is the request type for the Query/DisabledList RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDisabledListRequest {}
/// DisabledListResponse is the response type for the Query/DisabledList RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DisabledListResponse {
    #[prost(string, repeated, tag = "1")]
    pub disabled_list: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// Generated client implementations.
#[cfg(feature = "grpc")]
#[cfg_attr(docsrs, doc(cfg(feature = "grpc")))]
pub mod query_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    /// Query defines the circuit gRPC querier service.
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
        /// Account returns account permissions.
        pub async fn account(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryAccountRequest>,
        ) -> std::result::Result<tonic::Response<super::AccountResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cosmos.circuit.v1.Query/Account");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("cosmos.circuit.v1.Query", "Account"));
            self.inner.unary(req, path, codec).await
        }
        /// Account returns account permissions.
        pub async fn accounts(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryAccountsRequest>,
        ) -> std::result::Result<tonic::Response<super::AccountsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cosmos.circuit.v1.Query/Accounts");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("cosmos.circuit.v1.Query", "Accounts"));
            self.inner.unary(req, path, codec).await
        }
        /// DisabledList returns a list of disabled message urls
        pub async fn disabled_list(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryDisabledListRequest>,
        ) -> std::result::Result<tonic::Response<super::DisabledListResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/cosmos.circuit.v1.Query/DisabledList");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("cosmos.circuit.v1.Query", "DisabledList"));
            self.inner.unary(req, path, codec).await
        }
    }
}
