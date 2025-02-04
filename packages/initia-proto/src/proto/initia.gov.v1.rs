/// Params defines the parameters for the x/gov module.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    /// Minimum deposit for a proposal to enter voting period.
    #[prost(message, repeated, tag = "1")]
    pub min_deposit: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    /// Maximum period for Atom holders to deposit on a proposal. Initial value: 2
    /// months.
    #[prost(message, optional, tag = "2")]
    pub max_deposit_period: ::core::option::Option<::prost_types::Duration>,
    /// Duration of the voting period.
    #[prost(message, optional, tag = "3")]
    pub voting_period: ::core::option::Option<::prost_types::Duration>,
    ///   Minimum percentage of total stake needed to vote for a result to be
    ///   considered valid.
    #[prost(string, tag = "4")]
    pub quorum: ::prost::alloc::string::String,
    ///   Minimum proportion of Yes votes for proposal to pass. Default value: 0.5.
    #[prost(string, tag = "5")]
    pub threshold: ::prost::alloc::string::String,
    ///   Minimum value of Veto votes to Total votes ratio for proposal to be
    ///   vetoed. Default value: 1/3.
    #[prost(string, tag = "6")]
    pub veto_threshold: ::prost::alloc::string::String,
    ///   The ratio representing the proportion of the deposit value that must be paid at proposal submission.
    #[prost(string, tag = "7")]
    pub min_initial_deposit_ratio: ::prost::alloc::string::String,
    /// The cancel ratio which will not be returned back to the depositors when a proposal is cancelled.
    ///
    /// Since: cosmos-sdk 0.50
    #[prost(string, tag = "8")]
    pub proposal_cancel_ratio: ::prost::alloc::string::String,
    /// The address which will receive (proposal_cancel_ratio * deposit) proposal deposits.
    /// If empty, the (proposal_cancel_ratio * deposit) proposal deposits will be burned.
    ///
    /// Since: cosmos-sdk 0.50
    #[prost(string, tag = "9")]
    pub proposal_cancel_dest: ::prost::alloc::string::String,
    /// Duration of the voting period of an expedited proposal.
    ///
    /// Since: cosmos-sdk 0.50
    #[prost(message, optional, tag = "10")]
    pub expedited_voting_period: ::core::option::Option<::prost_types::Duration>,
    /// Minimum proportion of Yes votes for proposal to pass. Default value: 0.67.
    ///
    /// Since: cosmos-sdk 0.50
    #[prost(string, tag = "11")]
    pub expedited_threshold: ::prost::alloc::string::String,
    ///   Minimum expedited deposit for a proposal to enter voting period.
    #[prost(message, repeated, tag = "12")]
    pub expedited_min_deposit:
        ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    /// burn deposits if a proposal does not meet quorum
    #[prost(bool, tag = "13")]
    pub burn_vote_quorum: bool,
    /// burn deposits if the proposal does not enter voting period
    #[prost(bool, tag = "14")]
    pub burn_proposal_deposit_prevote: bool,
    /// burn deposits if quorum with vote type no_veto is met
    #[prost(bool, tag = "15")]
    pub burn_vote_veto: bool,
    /// The ratio representing the proportion of the deposit value minimum that must be met when making a deposit.
    /// Default value: 0.01. Meaning that for a chain with a min_deposit of 100stake, a deposit of 1stake would be
    /// required.
    ///
    /// Since: cosmos-sdk 0.50
    #[prost(string, tag = "16")]
    pub min_deposit_ratio: ::prost::alloc::string::String,
    /// Minimum deposit for a emergency proposal to enter voting period.
    #[prost(message, repeated, tag = "91")]
    pub emergency_min_deposit:
        ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    /// Tally interval for emergency proposal.
    #[prost(message, optional, tag = "92")]
    pub emergency_tally_interval: ::core::option::Option<::prost_types::Duration>,
    /// Low threshold functions for emergency and expedited proposal.
    /// These are Move function identifiers and can only be applied
    /// for Move execute messages.
    #[prost(string, repeated, tag = "93")]
    pub low_threshold_functions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Vesting is the vesting contract info for tally.
    #[prost(message, optional, tag = "94")]
    pub vesting: ::core::option::Option<Vesting>,
}
/// Vesting defines the vesting contract info for MsgVestingVote.
/// At tally time, gov module would check vesting token amount.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Vesting {
    /// module_addr is the address of the vesting module.
    #[prost(string, tag = "1")]
    pub module_addr: ::prost::alloc::string::String,
    /// module_name is the name of the vesting module.
    #[prost(string, tag = "2")]
    pub module_name: ::prost::alloc::string::String,
    /// creator_addr is the address of the creator of the vesting contract.
    #[prost(string, tag = "3")]
    pub creator_addr: ::prost::alloc::string::String,
}
/// TallyResult defines the result of a tally.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TallyResult {
    #[prost(uint64, tag = "1")]
    pub tally_height: u64,
    #[prost(string, tag = "2")]
    pub total_staking_power: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub total_vesting_power: ::prost::alloc::string::String,
    /// v1_tally_result is the original TallyResult from cosmos-sdk,
    /// which contains both staking and vesting power.
    #[prost(message, optional, tag = "4")]
    pub v1_tally_result: ::core::option::Option<super::super::super::cosmos::gov::v1::TallyResult>,
}
/// Proposal defines the core field members of a governance proposal.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Proposal {
    /// id defines the unique id of the proposal.
    #[prost(uint64, tag = "1")]
    pub id: u64,
    /// messages are the arbitrary messages to be executed if the proposal passes.
    #[prost(message, repeated, tag = "2")]
    pub messages: ::prost::alloc::vec::Vec<::prost_types::Any>,
    /// status defines the proposal status.
    #[prost(
        enumeration = "super::super::super::cosmos::gov::v1::ProposalStatus",
        tag = "3"
    )]
    pub status: i32,
    /// final_tally_result is the final tally result of the proposal. When
    /// querying a proposal via gRPC, this field is not populated until the
    /// proposal's voting period has ended.
    #[prost(message, optional, tag = "4")]
    pub final_tally_result: ::core::option::Option<TallyResult>,
    /// submit_time is the time of proposal submission.
    #[prost(message, optional, tag = "5")]
    pub submit_time: ::core::option::Option<::prost_types::Timestamp>,
    /// deposit_end_time is the end time for deposition.
    #[prost(message, optional, tag = "6")]
    pub deposit_end_time: ::core::option::Option<::prost_types::Timestamp>,
    /// total_deposit is the total deposit on the proposal.
    #[prost(message, repeated, tag = "7")]
    pub total_deposit: ::prost::alloc::vec::Vec<super::super::super::cosmos::base::v1beta1::Coin>,
    /// voting_start_time is the starting time to vote on a proposal.
    #[prost(message, optional, tag = "8")]
    pub voting_start_time: ::core::option::Option<::prost_types::Timestamp>,
    /// voting_end_time is the end time of voting on a proposal.
    #[prost(message, optional, tag = "9")]
    pub voting_end_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag = "10")]
    pub emergency_start_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(message, optional, tag = "11")]
    pub emergency_next_tally_time: ::core::option::Option<::prost_types::Timestamp>,
    /// metadata is any arbitrary metadata attached to the proposal.
    /// the recommended format of the metadata is to be found here:
    /// <https://docs.cosmos.network/v0.47/modules/gov#proposal-3>
    #[prost(string, tag = "12")]
    pub metadata: ::prost::alloc::string::String,
    /// title is the title of the proposal
    ///
    /// Since: cosmos-sdk 0.47
    #[prost(string, tag = "13")]
    pub title: ::prost::alloc::string::String,
    /// summary is a short summary of the proposal
    ///
    /// Since: cosmos-sdk 0.47
    #[prost(string, tag = "14")]
    pub summary: ::prost::alloc::string::String,
    /// proposer is the address of the proposal sumbitter
    ///
    /// Since: cosmos-sdk 0.47
    #[prost(string, tag = "15")]
    pub proposer: ::prost::alloc::string::String,
    /// expedited defines if the proposal is expedited
    ///
    /// Since: cosmos-sdk 0.50
    #[prost(bool, tag = "16")]
    pub expedited: bool,
    #[prost(bool, tag = "17")]
    pub emergency: bool,
    /// failed_reason defines the reason why the proposal failed
    ///
    /// Since: cosmos-sdk 0.50
    #[prost(string, tag = "18")]
    pub failed_reason: ::prost::alloc::string::String,
}
/// GenesisState defines the gov module's genesis state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// starting_proposal_id is the ID of the starting proposal.
    #[prost(uint64, tag = "1")]
    pub starting_proposal_id: u64,
    /// deposits defines all the deposits present at genesis.
    #[prost(message, repeated, tag = "2")]
    pub deposits: ::prost::alloc::vec::Vec<super::super::super::cosmos::gov::v1::Deposit>,
    /// votes defines all the votes present at genesis.
    #[prost(message, repeated, tag = "3")]
    pub votes: ::prost::alloc::vec::Vec<super::super::super::cosmos::gov::v1::Vote>,
    /// proposals defines all the proposals present at genesis.
    #[prost(message, repeated, tag = "4")]
    pub proposals: ::prost::alloc::vec::Vec<Proposal>,
    /// params defines all the parameters of x/gov module.
    #[prost(message, optional, tag = "5")]
    pub params: ::core::option::Option<Params>,
    /// The constitution allows builders to lay a foundation and define purpose.
    /// This is an immutable string set in genesis.
    /// There are no amendments, to go outside of scope, just fork.
    /// constitution is an immutable string in genesis for a chain builder to lay out their vision, ideas and ideals.
    ///
    /// Since: cosmos-sdk 0.50
    #[prost(string, tag = "6")]
    pub constitution: ::prost::alloc::string::String,
}
/// MsgUpdateParams is the Msg/UpdateParams request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateParams {
    /// authority is the address that controls the module (defaults to x/gov unless overwritten).
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// params defines the x/gov parameters to update.
    ///
    /// NOTE: All parameters must be supplied.
    #[prost(message, optional, tag = "2")]
    pub params: ::core::option::Option<Params>,
}
/// MsgUpdateParamsResponse defines the response structure for executing a
/// MsgUpdateParams message.
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
    /// Msg defines the gov Msg service.
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
        /// UpdateParams defines a governance operation for updating the x/gov
        /// module parameters. The authority is defined in the keeper.
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
            let path = http::uri::PathAndQuery::from_static("/initia.gov.v1.Msg/UpdateParams");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("initia.gov.v1.Msg", "UpdateParams"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
#[cfg(feature = "grpc")]
#[cfg_attr(docsrs, doc(cfg(feature = "grpc")))]
pub mod msg_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with MsgServer.
    #[async_trait]
    pub trait Msg: Send + Sync + 'static {
        /// UpdateParams defines a governance operation for updating the x/gov
        /// module parameters. The authority is defined in the keeper.
        async fn update_params(
            &self,
            request: tonic::Request<super::MsgUpdateParams>,
        ) -> std::result::Result<tonic::Response<super::MsgUpdateParamsResponse>, tonic::Status>;
    }
    /// Msg defines the gov Msg service.
    #[derive(Debug)]
    pub struct MsgServer<T: Msg> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Msg> MsgServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for MsgServer<T>
    where
        T: Msg,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/initia.gov.v1.Msg/UpdateParams" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateParamsSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgUpdateParams> for UpdateParamsSvc<T> {
                        type Response = super::MsgUpdateParamsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgUpdateParams>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).update_params(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UpdateParamsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: Msg> Clone for MsgServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: Msg> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Msg> tonic::server::NamedService for MsgServer<T> {
        const NAME: &'static str = "initia.gov.v1.Msg";
    }
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
/// QueryEmergencyProposalsRequest is the request type for the
/// Query/EmergencyProposals RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryEmergencyProposalsRequest {
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "1")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryEmergencyProposalsResponse is the response type for the
/// Query/EmergencyProposals RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryEmergencyProposalsResponse {
    #[prost(message, repeated, tag = "1")]
    pub proposals: ::prost::alloc::vec::Vec<Proposal>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryProposalRequest is the request type for the Query/Proposal RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryProposalRequest {
    /// proposal_id defines the unique id of the proposal.
    #[prost(uint64, tag = "1")]
    pub proposal_id: u64,
}
/// QueryProposalResponse is the response type for the Query/Proposal RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryProposalResponse {
    /// proposal is the requested governance proposal.
    #[prost(message, optional, tag = "1")]
    pub proposal: ::core::option::Option<Proposal>,
}
/// QueryProposalsRequest is the request type for the Query/Proposals RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryProposalsRequest {
    /// proposal_status defines the status of the proposals.
    #[prost(
        enumeration = "super::super::super::cosmos::gov::v1::ProposalStatus",
        tag = "1"
    )]
    pub proposal_status: i32,
    /// voter defines the voter address for the proposals.
    #[prost(string, tag = "2")]
    pub voter: ::prost::alloc::string::String,
    /// depositor defines the deposit addresses from the proposals.
    #[prost(string, tag = "3")]
    pub depositor: ::prost::alloc::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "4")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryProposalsResponse is the response type for the Query/Proposals RPC
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryProposalsResponse {
    /// proposals defines all the requested governance proposals.
    #[prost(message, repeated, tag = "1")]
    pub proposals: ::prost::alloc::vec::Vec<Proposal>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryTallyResultRequest is the request type for the Query/Tally RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTallyResultRequest {
    /// proposal_id defines the unique id of the proposal.
    #[prost(uint64, tag = "1")]
    pub proposal_id: u64,
}
/// QueryTallyResultResponse is the response type for the Query/Tally RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTallyResultResponse {
    /// tally defines the requested tally.
    #[prost(message, optional, tag = "1")]
    pub tally_result: ::core::option::Option<TallyResult>,
}
/// Generated client implementations.
#[cfg(feature = "grpc")]
#[cfg_attr(docsrs, doc(cfg(feature = "grpc")))]
pub mod query_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    /// Query defines the gRPC querier service for gov module.
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
        /// Params queries params of the gov module.
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
            let path = http::uri::PathAndQuery::from_static("/initia.gov.v1.Query/Params");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("initia.gov.v1.Query", "Params"));
            self.inner.unary(req, path, codec).await
        }
        /// EmergencyProposals queries emergency proposals.
        pub async fn emergency_proposals(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryEmergencyProposalsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryEmergencyProposalsResponse>,
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
                http::uri::PathAndQuery::from_static("/initia.gov.v1.Query/EmergencyProposals");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("initia.gov.v1.Query", "EmergencyProposals"));
            self.inner.unary(req, path, codec).await
        }
        /// Proposal queries proposal details based on ProposalID.
        pub async fn proposal(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryProposalRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryProposalResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/initia.gov.v1.Query/Proposal");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("initia.gov.v1.Query", "Proposal"));
            self.inner.unary(req, path, codec).await
        }
        /// Proposals queries all proposals based on given status.
        pub async fn proposals(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryProposalsRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryProposalsResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/initia.gov.v1.Query/Proposals");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("initia.gov.v1.Query", "Proposals"));
            self.inner.unary(req, path, codec).await
        }
        /// TallyResult queries the tally of a proposal vote.
        pub async fn tally_result(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryTallyResultRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryTallyResultResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/initia.gov.v1.Query/TallyResult");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("initia.gov.v1.Query", "TallyResult"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
#[cfg(feature = "grpc")]
#[cfg_attr(docsrs, doc(cfg(feature = "grpc")))]
pub mod query_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with QueryServer.
    #[async_trait]
    pub trait Query: Send + Sync + 'static {
        /// Params queries params of the gov module.
        async fn params(
            &self,
            request: tonic::Request<super::QueryParamsRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryParamsResponse>, tonic::Status>;
        /// EmergencyProposals queries emergency proposals.
        async fn emergency_proposals(
            &self,
            request: tonic::Request<super::QueryEmergencyProposalsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryEmergencyProposalsResponse>,
            tonic::Status,
        >;
        /// Proposal queries proposal details based on ProposalID.
        async fn proposal(
            &self,
            request: tonic::Request<super::QueryProposalRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryProposalResponse>, tonic::Status>;
        /// Proposals queries all proposals based on given status.
        async fn proposals(
            &self,
            request: tonic::Request<super::QueryProposalsRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryProposalsResponse>, tonic::Status>;
        /// TallyResult queries the tally of a proposal vote.
        async fn tally_result(
            &self,
            request: tonic::Request<super::QueryTallyResultRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryTallyResultResponse>, tonic::Status>;
    }
    /// Query defines the gRPC querier service for gov module.
    #[derive(Debug)]
    pub struct QueryServer<T: Query> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Query> QueryServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for QueryServer<T>
    where
        T: Query,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/initia.gov.v1.Query/Params" => {
                    #[allow(non_camel_case_types)]
                    struct ParamsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryParamsRequest> for ParamsSvc<T> {
                        type Response = super::QueryParamsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryParamsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).params(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ParamsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/initia.gov.v1.Query/EmergencyProposals" => {
                    #[allow(non_camel_case_types)]
                    struct EmergencyProposalsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query>
                        tonic::server::UnaryService<super::QueryEmergencyProposalsRequest>
                        for EmergencyProposalsSvc<T>
                    {
                        type Response = super::QueryEmergencyProposalsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryEmergencyProposalsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).emergency_proposals(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = EmergencyProposalsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/initia.gov.v1.Query/Proposal" => {
                    #[allow(non_camel_case_types)]
                    struct ProposalSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryProposalRequest> for ProposalSvc<T> {
                        type Response = super::QueryProposalResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryProposalRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).proposal(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ProposalSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/initia.gov.v1.Query/Proposals" => {
                    #[allow(non_camel_case_types)]
                    struct ProposalsSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryProposalsRequest> for ProposalsSvc<T> {
                        type Response = super::QueryProposalsResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryProposalsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).proposals(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ProposalsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/initia.gov.v1.Query/TallyResult" => {
                    #[allow(non_camel_case_types)]
                    struct TallyResultSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryTallyResultRequest> for TallyResultSvc<T> {
                        type Response = super::QueryTallyResultResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryTallyResultRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).tally_result(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = TallyResultSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: Query> Clone for QueryServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: Query> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Query> tonic::server::NamedService for QueryServer<T> {
        const NAME: &'static str = "initia.gov.v1.Query";
    }
}
