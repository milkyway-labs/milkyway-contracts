/// BaseAccount defines a base account type. It contains all the necessary fields
/// for basic account functionality. Any custom account type should extend this
/// type for additional functionality (e.g. vesting).
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BaseAccount {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub pub_key: ::core::option::Option<::prost_types::Any>,
    #[prost(uint64, tag = "3")]
    pub account_number: u64,
    #[prost(uint64, tag = "4")]
    pub sequence: u64,
}
/// ModuleAccount defines an account for modules that holds coins on a pool.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModuleAccount {
    #[prost(message, optional, tag = "1")]
    pub base_account: ::core::option::Option<BaseAccount>,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "3")]
    pub permissions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// ModuleCredential represents a unclaimable pubkey for base accounts controlled by modules.
///
/// Since: cosmos-sdk 0.47
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ModuleCredential {
    /// module_name is the name of the module used for address derivation (passed into address.Module).
    #[prost(string, tag = "1")]
    pub module_name: ::prost::alloc::string::String,
    /// derivation_keys is for deriving a module account address (passed into address.Module)
    /// adding more keys creates sub-account addresses (passed into address.Derive)
    #[prost(bytes = "vec", repeated, tag = "2")]
    pub derivation_keys: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// Params defines the parameters for the auth module.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    #[prost(uint64, tag = "1")]
    pub max_memo_characters: u64,
    #[prost(uint64, tag = "2")]
    pub tx_sig_limit: u64,
    #[prost(uint64, tag = "3")]
    pub tx_size_cost_per_byte: u64,
    #[prost(uint64, tag = "4")]
    pub sig_verify_cost_ed25519: u64,
    #[prost(uint64, tag = "5")]
    pub sig_verify_cost_secp256k1: u64,
}
/// GenesisState defines the auth module's genesis state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// params defines all the parameters of the module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    /// accounts are the accounts present at genesis.
    #[prost(message, repeated, tag = "2")]
    pub accounts: ::prost::alloc::vec::Vec<::prost_types::Any>,
}
/// MsgUpdateParams is the Msg/UpdateParams request type.
///
/// Since: cosmos-sdk 0.47
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateParams {
    /// authority is the address that controls the module (defaults to x/gov unless overwritten).
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// params defines the x/auth parameters to update.
    ///
    /// NOTE: All parameters must be supplied.
    #[prost(message, optional, tag = "2")]
    pub params: ::core::option::Option<Params>,
}
/// MsgUpdateParamsResponse defines the response structure for executing a
/// MsgUpdateParams message.
///
/// Since: cosmos-sdk 0.47
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateParamsResponse {}
/// Generated client implementations.
#[cfg(feature = "grpc")]
#[cfg_attr(docsrs, doc(cfg(feature = "grpc")))]
pub mod msg_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    /// Msg defines the x/auth Msg service.
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
        /// UpdateParams defines a (governance) operation for updating the x/auth module
        /// parameters. The authority defaults to the x/gov module account.
        ///
        /// Since: cosmos-sdk 0.47
        pub async fn update_params(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgUpdateParams>,
        ) -> std::result::Result<tonic::Response<super::MsgUpdateParamsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/cosmos.auth.v1beta1.Msg/UpdateParams");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("cosmos.auth.v1beta1.Msg", "UpdateParams"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// QueryAccountsRequest is the request type for the Query/Accounts RPC method.
///
/// Since: cosmos-sdk 0.43
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAccountsRequest {
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "1")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageRequest>,
}
/// QueryAccountsResponse is the response type for the Query/Accounts RPC method.
///
/// Since: cosmos-sdk 0.43
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAccountsResponse {
    /// accounts are the existing accounts
    #[prost(message, repeated, tag = "1")]
    pub accounts: ::prost::alloc::vec::Vec<::prost_types::Any>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageResponse>,
}
/// QueryAccountRequest is the request type for the Query/Account RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAccountRequest {
    /// address defines the address to query for.
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
/// QueryAccountResponse is the response type for the Query/Account RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAccountResponse {
    /// account defines the account of the corresponding address.
    #[prost(message, optional, tag = "1")]
    pub account: ::core::option::Option<::prost_types::Any>,
}
/// QueryParamsRequest is the request type for the Query/Params RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsRequest {}
/// QueryParamsResponse is the response type for the Query/Params RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsResponse {
    /// params defines the parameters of the module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
/// QueryModuleAccountsRequest is the request type for the Query/ModuleAccounts RPC method.
///
/// Since: cosmos-sdk 0.46
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryModuleAccountsRequest {}
/// QueryModuleAccountsResponse is the response type for the Query/ModuleAccounts RPC method.
///
/// Since: cosmos-sdk 0.46
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryModuleAccountsResponse {
    #[prost(message, repeated, tag = "1")]
    pub accounts: ::prost::alloc::vec::Vec<::prost_types::Any>,
}
/// QueryModuleAccountByNameRequest is the request type for the Query/ModuleAccountByName RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryModuleAccountByNameRequest {
    #[prost(string, tag = "1")]
    pub name: ::prost::alloc::string::String,
}
/// QueryModuleAccountByNameResponse is the response type for the Query/ModuleAccountByName RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryModuleAccountByNameResponse {
    #[prost(message, optional, tag = "1")]
    pub account: ::core::option::Option<::prost_types::Any>,
}
/// Bech32PrefixRequest is the request type for Bech32Prefix rpc method.
///
/// Since: cosmos-sdk 0.46
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Bech32PrefixRequest {}
/// Bech32PrefixResponse is the response type for Bech32Prefix rpc method.
///
/// Since: cosmos-sdk 0.46
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Bech32PrefixResponse {
    #[prost(string, tag = "1")]
    pub bech32_prefix: ::prost::alloc::string::String,
}
/// AddressBytesToStringRequest is the request type for AddressString rpc method.
///
/// Since: cosmos-sdk 0.46
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddressBytesToStringRequest {
    #[prost(bytes = "vec", tag = "1")]
    pub address_bytes: ::prost::alloc::vec::Vec<u8>,
}
/// AddressBytesToStringResponse is the response type for AddressString rpc method.
///
/// Since: cosmos-sdk 0.46
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddressBytesToStringResponse {
    #[prost(string, tag = "1")]
    pub address_string: ::prost::alloc::string::String,
}
/// AddressStringToBytesRequest is the request type for AccountBytes rpc method.
///
/// Since: cosmos-sdk 0.46
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddressStringToBytesRequest {
    #[prost(string, tag = "1")]
    pub address_string: ::prost::alloc::string::String,
}
/// AddressStringToBytesResponse is the response type for AddressBytes rpc method.
///
/// Since: cosmos-sdk 0.46
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddressStringToBytesResponse {
    #[prost(bytes = "vec", tag = "1")]
    pub address_bytes: ::prost::alloc::vec::Vec<u8>,
}
/// QueryAccountAddressByIDRequest is the request type for AccountAddressByID rpc method
///
/// Since: cosmos-sdk 0.46.2
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAccountAddressByIdRequest {
    /// Deprecated, use account_id instead
    ///
    /// id is the account number of the address to be queried. This field
    /// should have been an uint64 (like all account numbers), and will be
    /// updated to uint64 in a future version of the auth query.
    #[deprecated]
    #[prost(int64, tag = "1")]
    pub id: i64,
    /// account_id is the account number of the address to be queried.
    ///
    /// Since: cosmos-sdk 0.47
    #[prost(uint64, tag = "2")]
    pub account_id: u64,
}
/// QueryAccountAddressByIDResponse is the response type for AccountAddressByID rpc method
///
/// Since: cosmos-sdk 0.46.2
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAccountAddressByIdResponse {
    #[prost(string, tag = "1")]
    pub account_address: ::prost::alloc::string::String,
}
/// QueryAccountInfoRequest is the Query/AccountInfo request type.
///
/// Since: cosmos-sdk 0.47
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAccountInfoRequest {
    /// address is the account address string.
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
/// QueryAccountInfoResponse is the Query/AccountInfo response type.
///
/// Since: cosmos-sdk 0.47
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAccountInfoResponse {
    /// info is the account info which is represented by BaseAccount.
    #[prost(message, optional, tag = "1")]
    pub info: ::core::option::Option<BaseAccount>,
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
        /// Accounts returns all the existing accounts.
        ///
        /// When called from another module, this query might consume a high amount of
        /// gas if the pagination field is incorrectly set.
        ///
        /// Since: cosmos-sdk 0.43
        pub async fn accounts(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryAccountsRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryAccountsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cosmos.auth.v1beta1.Query/Accounts");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("cosmos.auth.v1beta1.Query", "Accounts"));
            self.inner.unary(req, path, codec).await
        }
        /// Account returns account details based on address.
        pub async fn account(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryAccountRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryAccountResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cosmos.auth.v1beta1.Query/Account");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("cosmos.auth.v1beta1.Query", "Account"));
            self.inner.unary(req, path, codec).await
        }
        /// AccountAddressByID returns account address based on account number.
        ///
        /// Since: cosmos-sdk 0.46.2
        pub async fn account_address_by_id(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryAccountAddressByIdRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryAccountAddressByIdResponse>,
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
                "/cosmos.auth.v1beta1.Query/AccountAddressByID",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "cosmos.auth.v1beta1.Query",
                "AccountAddressByID",
            ));
            self.inner.unary(req, path, codec).await
        }
        /// Params queries all parameters.
        pub async fn params(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryParamsRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryParamsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cosmos.auth.v1beta1.Query/Params");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("cosmos.auth.v1beta1.Query", "Params"));
            self.inner.unary(req, path, codec).await
        }
        /// ModuleAccounts returns all the existing module accounts.
        ///
        /// Since: cosmos-sdk 0.46
        pub async fn module_accounts(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryModuleAccountsRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryModuleAccountsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/cosmos.auth.v1beta1.Query/ModuleAccounts");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "cosmos.auth.v1beta1.Query",
                "ModuleAccounts",
            ));
            self.inner.unary(req, path, codec).await
        }
        /// ModuleAccountByName returns the module account info by module name
        pub async fn module_account_by_name(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryModuleAccountByNameRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryModuleAccountByNameResponse>,
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
                "/cosmos.auth.v1beta1.Query/ModuleAccountByName",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "cosmos.auth.v1beta1.Query",
                "ModuleAccountByName",
            ));
            self.inner.unary(req, path, codec).await
        }
        /// Bech32Prefix queries bech32Prefix
        ///
        /// Since: cosmos-sdk 0.46
        pub async fn bech32_prefix(
            &mut self,
            request: impl tonic::IntoRequest<super::Bech32PrefixRequest>,
        ) -> std::result::Result<tonic::Response<super::Bech32PrefixResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/cosmos.auth.v1beta1.Query/Bech32Prefix");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("cosmos.auth.v1beta1.Query", "Bech32Prefix"));
            self.inner.unary(req, path, codec).await
        }
        /// AddressBytesToString converts Account Address bytes to string
        ///
        /// Since: cosmos-sdk 0.46
        pub async fn address_bytes_to_string(
            &mut self,
            request: impl tonic::IntoRequest<super::AddressBytesToStringRequest>,
        ) -> std::result::Result<tonic::Response<super::AddressBytesToStringResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cosmos.auth.v1beta1.Query/AddressBytesToString",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "cosmos.auth.v1beta1.Query",
                "AddressBytesToString",
            ));
            self.inner.unary(req, path, codec).await
        }
        /// AddressStringToBytes converts Address string to bytes
        ///
        /// Since: cosmos-sdk 0.46
        pub async fn address_string_to_bytes(
            &mut self,
            request: impl tonic::IntoRequest<super::AddressStringToBytesRequest>,
        ) -> std::result::Result<tonic::Response<super::AddressStringToBytesResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cosmos.auth.v1beta1.Query/AddressStringToBytes",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "cosmos.auth.v1beta1.Query",
                "AddressStringToBytes",
            ));
            self.inner.unary(req, path, codec).await
        }
        /// AccountInfo queries account info which is common to all account types.
        ///
        /// Since: cosmos-sdk 0.47
        pub async fn account_info(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryAccountInfoRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryAccountInfoResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/cosmos.auth.v1beta1.Query/AccountInfo");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("cosmos.auth.v1beta1.Query", "AccountInfo"));
            self.inner.unary(req, path, codec).await
        }
    }
}
