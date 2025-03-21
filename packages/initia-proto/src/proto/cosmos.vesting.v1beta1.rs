/// BaseVestingAccount implements the VestingAccount interface. It contains all
/// the necessary fields needed for any vesting account implementation.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BaseVestingAccount {
    #[prost(message, optional, tag = "1")]
    pub base_account: ::core::option::Option<super::super::auth::v1beta1::BaseAccount>,
    #[prost(message, repeated, tag = "2")]
    pub original_vesting: ::prost::alloc::vec::Vec<super::super::base::v1beta1::Coin>,
    #[prost(message, repeated, tag = "3")]
    pub delegated_free: ::prost::alloc::vec::Vec<super::super::base::v1beta1::Coin>,
    #[prost(message, repeated, tag = "4")]
    pub delegated_vesting: ::prost::alloc::vec::Vec<super::super::base::v1beta1::Coin>,
    /// Vesting end time, as unix timestamp (in seconds).
    #[prost(int64, tag = "5")]
    pub end_time: i64,
}
/// ContinuousVestingAccount implements the VestingAccount interface. It
/// continuously vests by unlocking coins linearly with respect to time.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContinuousVestingAccount {
    #[prost(message, optional, tag = "1")]
    pub base_vesting_account: ::core::option::Option<BaseVestingAccount>,
    /// Vesting start time, as unix timestamp (in seconds).
    #[prost(int64, tag = "2")]
    pub start_time: i64,
}
/// DelayedVestingAccount implements the VestingAccount interface. It vests all
/// coins after a specific time, but non prior. In other words, it keeps them
/// locked until a specified time.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DelayedVestingAccount {
    #[prost(message, optional, tag = "1")]
    pub base_vesting_account: ::core::option::Option<BaseVestingAccount>,
}
/// Period defines a length of time and amount of coins that will vest.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Period {
    /// Period duration in seconds.
    #[prost(int64, tag = "1")]
    pub length: i64,
    #[prost(message, repeated, tag = "2")]
    pub amount: ::prost::alloc::vec::Vec<super::super::base::v1beta1::Coin>,
}
/// PeriodicVestingAccount implements the VestingAccount interface. It
/// periodically vests by unlocking coins during each specified period.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PeriodicVestingAccount {
    #[prost(message, optional, tag = "1")]
    pub base_vesting_account: ::core::option::Option<BaseVestingAccount>,
    #[prost(int64, tag = "2")]
    pub start_time: i64,
    #[prost(message, repeated, tag = "3")]
    pub vesting_periods: ::prost::alloc::vec::Vec<Period>,
}
/// PermanentLockedAccount implements the VestingAccount interface. It does
/// not ever release coins, locking them indefinitely. Coins in this account can
/// still be used for delegating and for governance votes even while locked.
///
/// Since: cosmos-sdk 0.43
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PermanentLockedAccount {
    #[prost(message, optional, tag = "1")]
    pub base_vesting_account: ::core::option::Option<BaseVestingAccount>,
}
/// MsgCreateVestingAccount defines a message that enables creating a vesting
/// account.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateVestingAccount {
    #[prost(string, tag = "1")]
    pub from_address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub to_address: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub amount: ::prost::alloc::vec::Vec<super::super::base::v1beta1::Coin>,
    /// end of vesting as unix time (in seconds).
    #[prost(int64, tag = "4")]
    pub end_time: i64,
    #[prost(bool, tag = "5")]
    pub delayed: bool,
}
/// MsgCreateVestingAccountResponse defines the Msg/CreateVestingAccount response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateVestingAccountResponse {}
/// MsgCreatePermanentLockedAccount defines a message that enables creating a permanent
/// locked account.
///
/// Since: cosmos-sdk 0.46
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreatePermanentLockedAccount {
    #[prost(string, tag = "1")]
    pub from_address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub to_address: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub amount: ::prost::alloc::vec::Vec<super::super::base::v1beta1::Coin>,
}
/// MsgCreatePermanentLockedAccountResponse defines the Msg/CreatePermanentLockedAccount response type.
///
/// Since: cosmos-sdk 0.46
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreatePermanentLockedAccountResponse {}
/// MsgCreateVestingAccount defines a message that enables creating a vesting
/// account.
///
/// Since: cosmos-sdk 0.46
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreatePeriodicVestingAccount {
    #[prost(string, tag = "1")]
    pub from_address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub to_address: ::prost::alloc::string::String,
    /// start of vesting as unix time (in seconds).
    #[prost(int64, tag = "3")]
    pub start_time: i64,
    #[prost(message, repeated, tag = "4")]
    pub vesting_periods: ::prost::alloc::vec::Vec<Period>,
}
/// MsgCreateVestingAccountResponse defines the Msg/CreatePeriodicVestingAccount
/// response type.
///
/// Since: cosmos-sdk 0.46
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreatePeriodicVestingAccountResponse {}
/// Generated client implementations.
#[cfg(feature = "grpc")]
#[cfg_attr(docsrs, doc(cfg(feature = "grpc")))]
pub mod msg_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    /// Msg defines the bank Msg service.
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
        /// CreateVestingAccount defines a method that enables creating a vesting
        /// account.
        pub async fn create_vesting_account(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgCreateVestingAccount>,
        ) -> std::result::Result<
            tonic::Response<super::MsgCreateVestingAccountResponse>,
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
                "/cosmos.vesting.v1beta1.Msg/CreateVestingAccount",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "cosmos.vesting.v1beta1.Msg",
                "CreateVestingAccount",
            ));
            self.inner.unary(req, path, codec).await
        }
        /// CreatePermanentLockedAccount defines a method that enables creating a permanent
        /// locked account.
        ///
        /// Since: cosmos-sdk 0.46
        pub async fn create_permanent_locked_account(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgCreatePermanentLockedAccount>,
        ) -> std::result::Result<
            tonic::Response<super::MsgCreatePermanentLockedAccountResponse>,
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
                "/cosmos.vesting.v1beta1.Msg/CreatePermanentLockedAccount",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "cosmos.vesting.v1beta1.Msg",
                "CreatePermanentLockedAccount",
            ));
            self.inner.unary(req, path, codec).await
        }
        /// CreatePeriodicVestingAccount defines a method that enables creating a
        /// periodic vesting account.
        ///
        /// Since: cosmos-sdk 0.46
        pub async fn create_periodic_vesting_account(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgCreatePeriodicVestingAccount>,
        ) -> std::result::Result<
            tonic::Response<super::MsgCreatePeriodicVestingAccountResponse>,
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
                "/cosmos.vesting.v1beta1.Msg/CreatePeriodicVestingAccount",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "cosmos.vesting.v1beta1.Msg",
                "CreatePeriodicVestingAccount",
            ));
            self.inner.unary(req, path, codec).await
        }
    }
}
