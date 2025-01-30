/// Params defines the set of params for the distribution module.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    #[prost(string, tag = "1")]
    pub community_tax: ::prost::alloc::string::String,
    /// Deprecated: The base_proposer_reward field is deprecated and is no longer used
    /// in the x/distribution module's reward mechanism.
    #[deprecated]
    #[prost(string, tag = "2")]
    pub base_proposer_reward: ::prost::alloc::string::String,
    /// Deprecated: The bonus_proposer_reward field is deprecated and is no longer used
    /// in the x/distribution module's reward mechanism.
    #[deprecated]
    #[prost(string, tag = "3")]
    pub bonus_proposer_reward: ::prost::alloc::string::String,
    #[prost(bool, tag = "4")]
    pub withdraw_addr_enabled: bool,
}
/// ValidatorHistoricalRewards represents historical rewards for a validator.
/// Height is implicit within the store key.
/// Cumulative reward ratio is the sum from the zeroeth period
/// until this period of rewards / tokens, per the spec.
/// The reference count indicates the number of objects
/// which might need to reference this historical entry at any point.
/// ReferenceCount =
///     number of outstanding delegations which ended the associated period (and
///     might need to read that record)
///   + number of slashes which ended the associated period (and might need to
///   read that record)
///   + one per validator for the zeroeth period, set on initialization
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidatorHistoricalRewards {
    #[prost(message, repeated, tag = "1")]
    pub cumulative_reward_ratio: ::prost::alloc::vec::Vec<super::super::base::v1beta1::DecCoin>,
    #[prost(uint32, tag = "2")]
    pub reference_count: u32,
}
/// ValidatorCurrentRewards represents current rewards and current
/// period for a validator kept as a running counter and incremented
/// each block as long as the validator's tokens remain constant.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidatorCurrentRewards {
    #[prost(message, repeated, tag = "1")]
    pub rewards: ::prost::alloc::vec::Vec<super::super::base::v1beta1::DecCoin>,
    #[prost(uint64, tag = "2")]
    pub period: u64,
}
/// ValidatorAccumulatedCommission represents accumulated commission
/// for a validator kept as a running counter, can be withdrawn at any time.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidatorAccumulatedCommission {
    #[prost(message, repeated, tag = "1")]
    pub commission: ::prost::alloc::vec::Vec<super::super::base::v1beta1::DecCoin>,
}
/// ValidatorOutstandingRewards represents outstanding (un-withdrawn) rewards
/// for a validator inexpensive to track, allows simple sanity checks.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidatorOutstandingRewards {
    #[prost(message, repeated, tag = "1")]
    pub rewards: ::prost::alloc::vec::Vec<super::super::base::v1beta1::DecCoin>,
}
/// ValidatorSlashEvent represents a validator slash event.
/// Height is implicit within the store key.
/// This is needed to calculate appropriate amount of staking tokens
/// for delegations which are withdrawn after a slash has occurred.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidatorSlashEvent {
    #[prost(uint64, tag = "1")]
    pub validator_period: u64,
    #[prost(string, tag = "2")]
    pub fraction: ::prost::alloc::string::String,
}
/// ValidatorSlashEvents is a collection of ValidatorSlashEvent messages.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidatorSlashEvents {
    #[prost(message, repeated, tag = "1")]
    pub validator_slash_events: ::prost::alloc::vec::Vec<ValidatorSlashEvent>,
}
/// FeePool is the global fee pool for distribution.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeePool {
    #[prost(message, repeated, tag = "1")]
    pub community_pool: ::prost::alloc::vec::Vec<super::super::base::v1beta1::DecCoin>,
}
/// CommunityPoolSpendProposal details a proposal for use of community funds,
/// together with how many coins are proposed to be spent, and to which
/// recipient account.
///
/// Deprecated: Do not use. As of the Cosmos SDK release v0.47.x, there is no
/// longer a need for an explicit CommunityPoolSpendProposal. To spend community
/// pool funds, a simple MsgCommunityPoolSpend can be invoked from the x/gov
/// module via a v1 governance proposal.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommunityPoolSpendProposal {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub recipient: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "4")]
    pub amount: ::prost::alloc::vec::Vec<super::super::base::v1beta1::Coin>,
}
/// DelegatorStartingInfo represents the starting info for a delegator reward
/// period. It tracks the previous validator period, the delegation's amount of
/// staking token, and the creation height (to check later on if any slashes have
/// occurred). NOTE: Even though validators are slashed to whole staking tokens,
/// the delegators within the validator may be left with less than a full token,
/// thus sdk.Dec is used.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DelegatorStartingInfo {
    #[prost(uint64, tag = "1")]
    pub previous_period: u64,
    #[prost(string, tag = "2")]
    pub stake: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    pub height: u64,
}
/// DelegationDelegatorReward represents the properties
/// of a delegator's delegation reward.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DelegationDelegatorReward {
    #[prost(string, tag = "1")]
    pub validator_address: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub reward: ::prost::alloc::vec::Vec<super::super::base::v1beta1::DecCoin>,
}
/// CommunityPoolSpendProposalWithDeposit defines a CommunityPoolSpendProposal
/// with a deposit
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CommunityPoolSpendProposalWithDeposit {
    #[prost(string, tag = "1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub recipient: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub amount: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub deposit: ::prost::alloc::string::String,
}
/// DelegatorWithdrawInfo is the address for where distributions rewards are
/// withdrawn to by default this struct is only used at genesis to feed in
/// default withdraw addresses.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DelegatorWithdrawInfo {
    /// delegator_address is the address of the delegator.
    #[prost(string, tag = "1")]
    pub delegator_address: ::prost::alloc::string::String,
    /// withdraw_address is the address to withdraw the delegation rewards to.
    #[prost(string, tag = "2")]
    pub withdraw_address: ::prost::alloc::string::String,
}
/// ValidatorOutstandingRewardsRecord is used for import/export via genesis json.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidatorOutstandingRewardsRecord {
    /// validator_address is the address of the validator.
    #[prost(string, tag = "1")]
    pub validator_address: ::prost::alloc::string::String,
    /// outstanding_rewards represents the outstanding rewards of a validator.
    #[prost(message, repeated, tag = "2")]
    pub outstanding_rewards: ::prost::alloc::vec::Vec<super::super::base::v1beta1::DecCoin>,
}
/// ValidatorAccumulatedCommissionRecord is used for import / export via genesis
/// json.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidatorAccumulatedCommissionRecord {
    /// validator_address is the address of the validator.
    #[prost(string, tag = "1")]
    pub validator_address: ::prost::alloc::string::String,
    /// accumulated is the accumulated commission of a validator.
    #[prost(message, optional, tag = "2")]
    pub accumulated: ::core::option::Option<ValidatorAccumulatedCommission>,
}
/// ValidatorHistoricalRewardsRecord is used for import / export via genesis
/// json.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidatorHistoricalRewardsRecord {
    /// validator_address is the address of the validator.
    #[prost(string, tag = "1")]
    pub validator_address: ::prost::alloc::string::String,
    /// period defines the period the historical rewards apply to.
    #[prost(uint64, tag = "2")]
    pub period: u64,
    /// rewards defines the historical rewards of a validator.
    #[prost(message, optional, tag = "3")]
    pub rewards: ::core::option::Option<ValidatorHistoricalRewards>,
}
/// ValidatorCurrentRewardsRecord is used for import / export via genesis json.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidatorCurrentRewardsRecord {
    /// validator_address is the address of the validator.
    #[prost(string, tag = "1")]
    pub validator_address: ::prost::alloc::string::String,
    /// rewards defines the current rewards of a validator.
    #[prost(message, optional, tag = "2")]
    pub rewards: ::core::option::Option<ValidatorCurrentRewards>,
}
/// DelegatorStartingInfoRecord used for import / export via genesis json.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DelegatorStartingInfoRecord {
    /// delegator_address is the address of the delegator.
    #[prost(string, tag = "1")]
    pub delegator_address: ::prost::alloc::string::String,
    /// validator_address is the address of the validator.
    #[prost(string, tag = "2")]
    pub validator_address: ::prost::alloc::string::String,
    /// starting_info defines the starting info of a delegator.
    #[prost(message, optional, tag = "3")]
    pub starting_info: ::core::option::Option<DelegatorStartingInfo>,
}
/// ValidatorSlashEventRecord is used for import / export via genesis json.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidatorSlashEventRecord {
    /// validator_address is the address of the validator.
    #[prost(string, tag = "1")]
    pub validator_address: ::prost::alloc::string::String,
    /// height defines the block height at which the slash event occurred.
    #[prost(uint64, tag = "2")]
    pub height: u64,
    /// period is the period of the slash event.
    #[prost(uint64, tag = "3")]
    pub period: u64,
    /// validator_slash_event describes the slash event.
    #[prost(message, optional, tag = "4")]
    pub validator_slash_event: ::core::option::Option<ValidatorSlashEvent>,
}
/// GenesisState defines the distribution module's genesis state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// params defines all the parameters of the module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    /// fee_pool defines the fee pool at genesis.
    #[prost(message, optional, tag = "2")]
    pub fee_pool: ::core::option::Option<FeePool>,
    /// fee_pool defines the delegator withdraw infos at genesis.
    #[prost(message, repeated, tag = "3")]
    pub delegator_withdraw_infos: ::prost::alloc::vec::Vec<DelegatorWithdrawInfo>,
    /// fee_pool defines the previous proposer at genesis.
    #[prost(string, tag = "4")]
    pub previous_proposer: ::prost::alloc::string::String,
    /// fee_pool defines the outstanding rewards of all validators at genesis.
    #[prost(message, repeated, tag = "5")]
    pub outstanding_rewards: ::prost::alloc::vec::Vec<ValidatorOutstandingRewardsRecord>,
    /// fee_pool defines the accumulated commissions of all validators at genesis.
    #[prost(message, repeated, tag = "6")]
    pub validator_accumulated_commissions:
        ::prost::alloc::vec::Vec<ValidatorAccumulatedCommissionRecord>,
    /// fee_pool defines the historical rewards of all validators at genesis.
    #[prost(message, repeated, tag = "7")]
    pub validator_historical_rewards: ::prost::alloc::vec::Vec<ValidatorHistoricalRewardsRecord>,
    /// fee_pool defines the current rewards of all validators at genesis.
    #[prost(message, repeated, tag = "8")]
    pub validator_current_rewards: ::prost::alloc::vec::Vec<ValidatorCurrentRewardsRecord>,
    /// fee_pool defines the delegator starting infos at genesis.
    #[prost(message, repeated, tag = "9")]
    pub delegator_starting_infos: ::prost::alloc::vec::Vec<DelegatorStartingInfoRecord>,
    /// fee_pool defines the validator slash events at genesis.
    #[prost(message, repeated, tag = "10")]
    pub validator_slash_events: ::prost::alloc::vec::Vec<ValidatorSlashEventRecord>,
}
/// MsgSetWithdrawAddress sets the withdraw address for
/// a delegator (or validator self-delegation).
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSetWithdrawAddress {
    #[prost(string, tag = "1")]
    pub delegator_address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub withdraw_address: ::prost::alloc::string::String,
}
/// MsgSetWithdrawAddressResponse defines the Msg/SetWithdrawAddress response
/// type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSetWithdrawAddressResponse {}
/// MsgWithdrawDelegatorReward represents delegation withdrawal to a delegator
/// from a single validator.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgWithdrawDelegatorReward {
    #[prost(string, tag = "1")]
    pub delegator_address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub validator_address: ::prost::alloc::string::String,
}
/// MsgWithdrawDelegatorRewardResponse defines the Msg/WithdrawDelegatorReward
/// response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgWithdrawDelegatorRewardResponse {
    /// Since: cosmos-sdk 0.46
    #[prost(message, repeated, tag = "1")]
    pub amount: ::prost::alloc::vec::Vec<super::super::base::v1beta1::Coin>,
}
/// MsgWithdrawValidatorCommission withdraws the full commission to the validator
/// address.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgWithdrawValidatorCommission {
    #[prost(string, tag = "1")]
    pub validator_address: ::prost::alloc::string::String,
}
/// MsgWithdrawValidatorCommissionResponse defines the
/// Msg/WithdrawValidatorCommission response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgWithdrawValidatorCommissionResponse {
    /// Since: cosmos-sdk 0.46
    #[prost(message, repeated, tag = "1")]
    pub amount: ::prost::alloc::vec::Vec<super::super::base::v1beta1::Coin>,
}
/// MsgFundCommunityPool allows an account to directly
/// fund the community pool.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgFundCommunityPool {
    #[prost(message, repeated, tag = "1")]
    pub amount: ::prost::alloc::vec::Vec<super::super::base::v1beta1::Coin>,
    #[prost(string, tag = "2")]
    pub depositor: ::prost::alloc::string::String,
}
/// MsgFundCommunityPoolResponse defines the Msg/FundCommunityPool response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgFundCommunityPoolResponse {}
/// MsgUpdateParams is the Msg/UpdateParams request type.
///
/// Since: cosmos-sdk 0.47
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateParams {
    /// authority is the address that controls the module (defaults to x/gov unless overwritten).
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// params defines the x/distribution parameters to update.
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
/// MsgCommunityPoolSpend defines a message for sending tokens from the community
/// pool to another account. This message is typically executed via a governance
/// proposal with the governance module being the executing authority.
///
/// Since: cosmos-sdk 0.47
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCommunityPoolSpend {
    /// authority is the address that controls the module (defaults to x/gov unless overwritten).
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub recipient: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub amount: ::prost::alloc::vec::Vec<super::super::base::v1beta1::Coin>,
}
/// MsgCommunityPoolSpendResponse defines the response to executing a
/// MsgCommunityPoolSpend message.
///
/// Since: cosmos-sdk 0.47
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCommunityPoolSpendResponse {}
/// DepositValidatorRewardsPool defines the request structure to provide
/// additional rewards to delegators from a specific validator.
///
/// Since: cosmos-sdk 0.50
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDepositValidatorRewardsPool {
    #[prost(string, tag = "1")]
    pub depositor: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub validator_address: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "3")]
    pub amount: ::prost::alloc::vec::Vec<super::super::base::v1beta1::Coin>,
}
/// MsgDepositValidatorRewardsPoolResponse defines the response to executing a
/// MsgDepositValidatorRewardsPool message.
///
/// Since: cosmos-sdk 0.50
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDepositValidatorRewardsPoolResponse {}
/// Generated client implementations.
#[cfg(feature = "grpc")]
#[cfg_attr(docsrs, doc(cfg(feature = "grpc")))]
pub mod msg_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    /// Msg defines the distribution Msg service.
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
        /// SetWithdrawAddress defines a method to change the withdraw address
        /// for a delegator (or validator self-delegation).
        pub async fn set_withdraw_address(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgSetWithdrawAddress>,
        ) -> std::result::Result<tonic::Response<super::MsgSetWithdrawAddressResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cosmos.distribution.v1beta1.Msg/SetWithdrawAddress",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "cosmos.distribution.v1beta1.Msg",
                "SetWithdrawAddress",
            ));
            self.inner.unary(req, path, codec).await
        }
        /// WithdrawDelegatorReward defines a method to withdraw rewards of delegator
        /// from a single validator.
        pub async fn withdraw_delegator_reward(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgWithdrawDelegatorReward>,
        ) -> std::result::Result<
            tonic::Response<super::MsgWithdrawDelegatorRewardResponse>,
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
                "/cosmos.distribution.v1beta1.Msg/WithdrawDelegatorReward",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "cosmos.distribution.v1beta1.Msg",
                "WithdrawDelegatorReward",
            ));
            self.inner.unary(req, path, codec).await
        }
        /// WithdrawValidatorCommission defines a method to withdraw the
        /// full commission to the validator address.
        pub async fn withdraw_validator_commission(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgWithdrawValidatorCommission>,
        ) -> std::result::Result<
            tonic::Response<super::MsgWithdrawValidatorCommissionResponse>,
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
                "/cosmos.distribution.v1beta1.Msg/WithdrawValidatorCommission",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "cosmos.distribution.v1beta1.Msg",
                "WithdrawValidatorCommission",
            ));
            self.inner.unary(req, path, codec).await
        }
        /// FundCommunityPool defines a method to allow an account to directly
        /// fund the community pool.
        pub async fn fund_community_pool(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgFundCommunityPool>,
        ) -> std::result::Result<tonic::Response<super::MsgFundCommunityPoolResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cosmos.distribution.v1beta1.Msg/FundCommunityPool",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "cosmos.distribution.v1beta1.Msg",
                "FundCommunityPool",
            ));
            self.inner.unary(req, path, codec).await
        }
        /// UpdateParams defines a governance operation for updating the x/distribution
        /// module parameters. The authority is defined in the keeper.
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
            let path = http::uri::PathAndQuery::from_static(
                "/cosmos.distribution.v1beta1.Msg/UpdateParams",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "cosmos.distribution.v1beta1.Msg",
                "UpdateParams",
            ));
            self.inner.unary(req, path, codec).await
        }
        /// CommunityPoolSpend defines a governance operation for sending tokens from
        /// the community pool in the x/distribution module to another account, which
        /// could be the governance module itself. The authority is defined in the
        /// keeper.
        ///
        /// Since: cosmos-sdk 0.47
        pub async fn community_pool_spend(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgCommunityPoolSpend>,
        ) -> std::result::Result<tonic::Response<super::MsgCommunityPoolSpendResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cosmos.distribution.v1beta1.Msg/CommunityPoolSpend",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "cosmos.distribution.v1beta1.Msg",
                "CommunityPoolSpend",
            ));
            self.inner.unary(req, path, codec).await
        }
        /// DepositValidatorRewardsPool defines a method to provide additional rewards
        /// to delegators to a specific validator.
        ///
        /// Since: cosmos-sdk 0.50
        pub async fn deposit_validator_rewards_pool(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgDepositValidatorRewardsPool>,
        ) -> std::result::Result<
            tonic::Response<super::MsgDepositValidatorRewardsPoolResponse>,
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
                "/cosmos.distribution.v1beta1.Msg/DepositValidatorRewardsPool",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "cosmos.distribution.v1beta1.Msg",
                "DepositValidatorRewardsPool",
            ));
            self.inner.unary(req, path, codec).await
        }
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
/// QueryValidatorDistributionInfoRequest is the request type for the Query/ValidatorDistributionInfo RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryValidatorDistributionInfoRequest {
    /// validator_address defines the validator address to query for.
    #[prost(string, tag = "1")]
    pub validator_address: ::prost::alloc::string::String,
}
/// QueryValidatorDistributionInfoResponse is the response type for the Query/ValidatorDistributionInfo RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryValidatorDistributionInfoResponse {
    /// operator_address defines the validator operator address.
    #[prost(string, tag = "1")]
    pub operator_address: ::prost::alloc::string::String,
    /// self_bond_rewards defines the self delegations rewards.
    #[prost(message, repeated, tag = "2")]
    pub self_bond_rewards: ::prost::alloc::vec::Vec<super::super::base::v1beta1::DecCoin>,
    /// commission defines the commission the validator received.
    #[prost(message, repeated, tag = "3")]
    pub commission: ::prost::alloc::vec::Vec<super::super::base::v1beta1::DecCoin>,
}
/// QueryValidatorOutstandingRewardsRequest is the request type for the
/// Query/ValidatorOutstandingRewards RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryValidatorOutstandingRewardsRequest {
    /// validator_address defines the validator address to query for.
    #[prost(string, tag = "1")]
    pub validator_address: ::prost::alloc::string::String,
}
/// QueryValidatorOutstandingRewardsResponse is the response type for the
/// Query/ValidatorOutstandingRewards RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryValidatorOutstandingRewardsResponse {
    #[prost(message, optional, tag = "1")]
    pub rewards: ::core::option::Option<ValidatorOutstandingRewards>,
}
/// QueryValidatorCommissionRequest is the request type for the
/// Query/ValidatorCommission RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryValidatorCommissionRequest {
    /// validator_address defines the validator address to query for.
    #[prost(string, tag = "1")]
    pub validator_address: ::prost::alloc::string::String,
}
/// QueryValidatorCommissionResponse is the response type for the
/// Query/ValidatorCommission RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryValidatorCommissionResponse {
    /// commission defines the commission the validator received.
    #[prost(message, optional, tag = "1")]
    pub commission: ::core::option::Option<ValidatorAccumulatedCommission>,
}
/// QueryValidatorSlashesRequest is the request type for the
/// Query/ValidatorSlashes RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryValidatorSlashesRequest {
    /// validator_address defines the validator address to query for.
    #[prost(string, tag = "1")]
    pub validator_address: ::prost::alloc::string::String,
    /// starting_height defines the optional starting height to query the slashes.
    #[prost(uint64, tag = "2")]
    pub starting_height: u64,
    /// starting_height defines the optional ending height to query the slashes.
    #[prost(uint64, tag = "3")]
    pub ending_height: u64,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "4")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageRequest>,
}
/// QueryValidatorSlashesResponse is the response type for the
/// Query/ValidatorSlashes RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryValidatorSlashesResponse {
    /// slashes defines the slashes the validator received.
    #[prost(message, repeated, tag = "1")]
    pub slashes: ::prost::alloc::vec::Vec<ValidatorSlashEvent>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageResponse>,
}
/// QueryDelegationRewardsRequest is the request type for the
/// Query/DelegationRewards RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDelegationRewardsRequest {
    /// delegator_address defines the delegator address to query for.
    #[prost(string, tag = "1")]
    pub delegator_address: ::prost::alloc::string::String,
    /// validator_address defines the validator address to query for.
    #[prost(string, tag = "2")]
    pub validator_address: ::prost::alloc::string::String,
}
/// QueryDelegationRewardsResponse is the response type for the
/// Query/DelegationRewards RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDelegationRewardsResponse {
    /// rewards defines the rewards accrued by a delegation.
    #[prost(message, repeated, tag = "1")]
    pub rewards: ::prost::alloc::vec::Vec<super::super::base::v1beta1::DecCoin>,
}
/// QueryDelegationTotalRewardsRequest is the request type for the
/// Query/DelegationTotalRewards RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDelegationTotalRewardsRequest {
    /// delegator_address defines the delegator address to query for.
    #[prost(string, tag = "1")]
    pub delegator_address: ::prost::alloc::string::String,
}
/// QueryDelegationTotalRewardsResponse is the response type for the
/// Query/DelegationTotalRewards RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDelegationTotalRewardsResponse {
    /// rewards defines all the rewards accrued by a delegator.
    #[prost(message, repeated, tag = "1")]
    pub rewards: ::prost::alloc::vec::Vec<DelegationDelegatorReward>,
    /// total defines the sum of all the rewards.
    #[prost(message, repeated, tag = "2")]
    pub total: ::prost::alloc::vec::Vec<super::super::base::v1beta1::DecCoin>,
}
/// QueryDelegatorValidatorsRequest is the request type for the
/// Query/DelegatorValidators RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDelegatorValidatorsRequest {
    /// delegator_address defines the delegator address to query for.
    #[prost(string, tag = "1")]
    pub delegator_address: ::prost::alloc::string::String,
}
/// QueryDelegatorValidatorsResponse is the response type for the
/// Query/DelegatorValidators RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDelegatorValidatorsResponse {
    /// validators defines the validators a delegator is delegating for.
    #[prost(string, repeated, tag = "1")]
    pub validators: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// QueryDelegatorWithdrawAddressRequest is the request type for the
/// Query/DelegatorWithdrawAddress RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDelegatorWithdrawAddressRequest {
    /// delegator_address defines the delegator address to query for.
    #[prost(string, tag = "1")]
    pub delegator_address: ::prost::alloc::string::String,
}
/// QueryDelegatorWithdrawAddressResponse is the response type for the
/// Query/DelegatorWithdrawAddress RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDelegatorWithdrawAddressResponse {
    /// withdraw_address defines the delegator address to query for.
    #[prost(string, tag = "1")]
    pub withdraw_address: ::prost::alloc::string::String,
}
/// QueryCommunityPoolRequest is the request type for the Query/CommunityPool RPC
/// method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryCommunityPoolRequest {}
/// QueryCommunityPoolResponse is the response type for the Query/CommunityPool
/// RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryCommunityPoolResponse {
    /// pool defines community pool's coins.
    #[prost(message, repeated, tag = "1")]
    pub pool: ::prost::alloc::vec::Vec<super::super::base::v1beta1::DecCoin>,
}
/// Generated client implementations.
#[cfg(feature = "grpc")]
#[cfg_attr(docsrs, doc(cfg(feature = "grpc")))]
pub mod query_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    /// Query defines the gRPC querier service for distribution module.
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
        /// Params queries params of the distribution module.
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
            let path =
                http::uri::PathAndQuery::from_static("/cosmos.distribution.v1beta1.Query/Params");
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "cosmos.distribution.v1beta1.Query",
                "Params",
            ));
            self.inner.unary(req, path, codec).await
        }
        /// ValidatorDistributionInfo queries validator commission and self-delegation rewards for validator
        pub async fn validator_distribution_info(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryValidatorDistributionInfoRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryValidatorDistributionInfoResponse>,
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
                "/cosmos.distribution.v1beta1.Query/ValidatorDistributionInfo",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "cosmos.distribution.v1beta1.Query",
                "ValidatorDistributionInfo",
            ));
            self.inner.unary(req, path, codec).await
        }
        /// ValidatorOutstandingRewards queries rewards of a validator address.
        pub async fn validator_outstanding_rewards(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryValidatorOutstandingRewardsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryValidatorOutstandingRewardsResponse>,
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
                "/cosmos.distribution.v1beta1.Query/ValidatorOutstandingRewards",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "cosmos.distribution.v1beta1.Query",
                "ValidatorOutstandingRewards",
            ));
            self.inner.unary(req, path, codec).await
        }
        /// ValidatorCommission queries accumulated commission for a validator.
        pub async fn validator_commission(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryValidatorCommissionRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryValidatorCommissionResponse>,
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
                "/cosmos.distribution.v1beta1.Query/ValidatorCommission",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "cosmos.distribution.v1beta1.Query",
                "ValidatorCommission",
            ));
            self.inner.unary(req, path, codec).await
        }
        /// ValidatorSlashes queries slash events of a validator.
        pub async fn validator_slashes(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryValidatorSlashesRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryValidatorSlashesResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cosmos.distribution.v1beta1.Query/ValidatorSlashes",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "cosmos.distribution.v1beta1.Query",
                "ValidatorSlashes",
            ));
            self.inner.unary(req, path, codec).await
        }
        /// DelegationRewards queries the total rewards accrued by a delegation.
        pub async fn delegation_rewards(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryDelegationRewardsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryDelegationRewardsResponse>,
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
                "/cosmos.distribution.v1beta1.Query/DelegationRewards",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "cosmos.distribution.v1beta1.Query",
                "DelegationRewards",
            ));
            self.inner.unary(req, path, codec).await
        }
        /// DelegationTotalRewards queries the total rewards accrued by each
        /// validator.
        pub async fn delegation_total_rewards(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryDelegationTotalRewardsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryDelegationTotalRewardsResponse>,
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
                "/cosmos.distribution.v1beta1.Query/DelegationTotalRewards",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "cosmos.distribution.v1beta1.Query",
                "DelegationTotalRewards",
            ));
            self.inner.unary(req, path, codec).await
        }
        /// DelegatorValidators queries the validators of a delegator.
        pub async fn delegator_validators(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryDelegatorValidatorsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryDelegatorValidatorsResponse>,
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
                "/cosmos.distribution.v1beta1.Query/DelegatorValidators",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "cosmos.distribution.v1beta1.Query",
                "DelegatorValidators",
            ));
            self.inner.unary(req, path, codec).await
        }
        /// DelegatorWithdrawAddress queries withdraw address of a delegator.
        pub async fn delegator_withdraw_address(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryDelegatorWithdrawAddressRequest>,
        ) -> std::result::Result<
            tonic::Response<super::QueryDelegatorWithdrawAddressResponse>,
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
                "/cosmos.distribution.v1beta1.Query/DelegatorWithdrawAddress",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "cosmos.distribution.v1beta1.Query",
                "DelegatorWithdrawAddress",
            ));
            self.inner.unary(req, path, codec).await
        }
        /// CommunityPool queries the community pool coins.
        pub async fn community_pool(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryCommunityPoolRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryCommunityPoolResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/cosmos.distribution.v1beta1.Query/CommunityPool",
            );
            let mut req = request.into_request();
            req.extensions_mut().insert(GrpcMethod::new(
                "cosmos.distribution.v1beta1.Query",
                "CommunityPool",
            ));
            self.inner.unary(req, path, codec).await
        }
    }
}
