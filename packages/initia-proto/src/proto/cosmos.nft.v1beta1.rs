/// Class defines the class of the nft type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Class {
    /// id defines the unique identifier of the NFT classification, similar to the contract address of ERC721
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// name defines the human-readable name of the NFT classification. Optional
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    /// symbol is an abbreviated name for nft classification. Optional
    #[prost(string, tag = "3")]
    pub symbol: ::prost::alloc::string::String,
    /// description is a brief description of nft classification. Optional
    #[prost(string, tag = "4")]
    pub description: ::prost::alloc::string::String,
    /// uri for the class metadata stored off chain. It can define schema for Class and NFT `Data` attributes. Optional
    #[prost(string, tag = "5")]
    pub uri: ::prost::alloc::string::String,
    /// uri_hash is a hash of the document pointed by uri. Optional
    #[prost(string, tag = "6")]
    pub uri_hash: ::prost::alloc::string::String,
    /// data is the app specific metadata of the NFT class. Optional
    #[prost(message, optional, tag = "7")]
    pub data: ::core::option::Option<::prost_types::Any>,
}
/// NFT defines the NFT.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Nft {
    /// class_id associated with the NFT, similar to the contract address of ERC721
    #[prost(string, tag = "1")]
    pub class_id: ::prost::alloc::string::String,
    /// id is a unique identifier of the NFT
    #[prost(string, tag = "2")]
    pub id: ::prost::alloc::string::String,
    /// uri for the NFT metadata stored off chain
    #[prost(string, tag = "3")]
    pub uri: ::prost::alloc::string::String,
    /// uri_hash is a hash of the document pointed by uri
    #[prost(string, tag = "4")]
    pub uri_hash: ::prost::alloc::string::String,
    /// data is an app specific data of the NFT. Optional
    #[prost(message, optional, tag = "10")]
    pub data: ::core::option::Option<::prost_types::Any>,
}
/// GenesisState defines the nft module's genesis state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    /// class defines the class of the nft type.
    #[prost(message, repeated, tag = "1")]
    pub classes: ::prost::alloc::vec::Vec<Class>,
    /// entry defines all nft owned by a person.
    #[prost(message, repeated, tag = "2")]
    pub entries: ::prost::alloc::vec::Vec<Entry>,
}
/// Entry Defines all nft owned by a person
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Entry {
    /// owner is the owner address of the following nft
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
    /// nfts is a group of nfts of the same owner
    #[prost(message, repeated, tag = "2")]
    pub nfts: ::prost::alloc::vec::Vec<Nft>,
}
/// MsgSend represents a message to send a nft from one account to another account.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSend {
    /// class_id defines the unique identifier of the nft classification, similar to the contract address of ERC721
    #[prost(string, tag = "1")]
    pub class_id: ::prost::alloc::string::String,
    /// id defines the unique identification of nft
    #[prost(string, tag = "2")]
    pub id: ::prost::alloc::string::String,
    /// sender is the address of the owner of nft
    #[prost(string, tag = "3")]
    pub sender: ::prost::alloc::string::String,
    /// receiver is the receiver address of nft
    #[prost(string, tag = "4")]
    pub receiver: ::prost::alloc::string::String,
}
/// MsgSendResponse defines the Msg/Send response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSendResponse {}
/// Generated client implementations.
#[cfg(feature = "grpc")]
#[cfg_attr(docsrs, doc(cfg(feature = "grpc")))]
pub mod msg_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    /// Msg defines the nft Msg service.
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
        /// Send defines a method to send a nft from one account to another account.
        pub async fn send(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgSend>,
        ) -> std::result::Result<tonic::Response<super::MsgSendResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cosmos.nft.v1beta1.Msg/Send");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("cosmos.nft.v1beta1.Msg", "Send"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// QueryBalanceRequest is the request type for the Query/Balance RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBalanceRequest {
    /// class_id associated with the nft
    #[prost(string, tag = "1")]
    pub class_id: ::prost::alloc::string::String,
    /// owner is the owner address of the nft
    #[prost(string, tag = "2")]
    pub owner: ::prost::alloc::string::String,
}
/// QueryBalanceResponse is the response type for the Query/Balance RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBalanceResponse {
    /// amount is the number of all NFTs of a given class owned by the owner
    #[prost(uint64, tag = "1")]
    pub amount: u64,
}
/// QueryOwnerRequest is the request type for the Query/Owner RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryOwnerRequest {
    /// class_id associated with the nft
    #[prost(string, tag = "1")]
    pub class_id: ::prost::alloc::string::String,
    /// id is a unique identifier of the NFT
    #[prost(string, tag = "2")]
    pub id: ::prost::alloc::string::String,
}
/// QueryOwnerResponse is the response type for the Query/Owner RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryOwnerResponse {
    /// owner is the owner address of the nft
    #[prost(string, tag = "1")]
    pub owner: ::prost::alloc::string::String,
}
/// QuerySupplyRequest is the request type for the Query/Supply RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySupplyRequest {
    /// class_id associated with the nft
    #[prost(string, tag = "1")]
    pub class_id: ::prost::alloc::string::String,
}
/// QuerySupplyResponse is the response type for the Query/Supply RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySupplyResponse {
    /// amount is the number of all NFTs from the given class
    #[prost(uint64, tag = "1")]
    pub amount: u64,
}
/// QueryNFTstRequest is the request type for the Query/NFTs RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryNfTsRequest {
    /// class_id associated with the nft
    #[prost(string, tag = "1")]
    pub class_id: ::prost::alloc::string::String,
    /// owner is the owner address of the nft
    #[prost(string, tag = "2")]
    pub owner: ::prost::alloc::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "3")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageRequest>,
}
/// QueryNFTsResponse is the response type for the Query/NFTs RPC methods
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryNfTsResponse {
    /// NFT defines the NFT
    #[prost(message, repeated, tag = "1")]
    pub nfts: ::prost::alloc::vec::Vec<Nft>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageResponse>,
}
/// QueryNFTRequest is the request type for the Query/NFT RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryNftRequest {
    /// class_id associated with the nft
    #[prost(string, tag = "1")]
    pub class_id: ::prost::alloc::string::String,
    /// id is a unique identifier of the NFT
    #[prost(string, tag = "2")]
    pub id: ::prost::alloc::string::String,
}
/// QueryNFTResponse is the response type for the Query/NFT RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryNftResponse {
    /// owner is the owner address of the nft
    #[prost(message, optional, tag = "1")]
    pub nft: ::core::option::Option<Nft>,
}
/// QueryClassRequest is the request type for the Query/Class RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryClassRequest {
    /// class_id associated with the nft
    #[prost(string, tag = "1")]
    pub class_id: ::prost::alloc::string::String,
}
/// QueryClassResponse is the response type for the Query/Class RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryClassResponse {
    /// class defines the class of the nft type.
    #[prost(message, optional, tag = "1")]
    pub class: ::core::option::Option<Class>,
}
/// QueryClassesRequest is the request type for the Query/Classes RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryClassesRequest {
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "1")]
    pub pagination: ::core::option::Option<super::super::base::query::v1beta1::PageRequest>,
}
/// QueryClassesResponse is the response type for the Query/Classes RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryClassesResponse {
    /// class defines the class of the nft type.
    #[prost(message, repeated, tag = "1")]
    pub classes: ::prost::alloc::vec::Vec<Class>,
    /// pagination defines the pagination in the response.
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
        /// Balance queries the number of NFTs of a given class owned by the owner, same as balanceOf in ERC721
        pub async fn balance(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryBalanceRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryBalanceResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cosmos.nft.v1beta1.Query/Balance");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("cosmos.nft.v1beta1.Query", "Balance"));
            self.inner.unary(req, path, codec).await
        }
        /// Owner queries the owner of the NFT based on its class and id, same as ownerOf in ERC721
        pub async fn owner(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryOwnerRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryOwnerResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cosmos.nft.v1beta1.Query/Owner");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("cosmos.nft.v1beta1.Query", "Owner"));
            self.inner.unary(req, path, codec).await
        }
        /// Supply queries the number of NFTs from the given class, same as totalSupply of ERC721.
        pub async fn supply(
            &mut self,
            request: impl tonic::IntoRequest<super::QuerySupplyRequest>,
        ) -> std::result::Result<tonic::Response<super::QuerySupplyResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cosmos.nft.v1beta1.Query/Supply");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("cosmos.nft.v1beta1.Query", "Supply"));
            self.inner.unary(req, path, codec).await
        }
        /// NFTs queries all NFTs of a given class or owner,choose at least one of the two, similar to tokenByIndex in
        /// ERC721Enumerable
        pub async fn nf_ts(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryNfTsRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryNfTsResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cosmos.nft.v1beta1.Query/NFTs");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("cosmos.nft.v1beta1.Query", "NFTs"));
            self.inner.unary(req, path, codec).await
        }
        /// NFT queries an NFT based on its class and id.
        pub async fn nft(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryNftRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryNftResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cosmos.nft.v1beta1.Query/NFT");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("cosmos.nft.v1beta1.Query", "NFT"));
            self.inner.unary(req, path, codec).await
        }
        /// Class queries an NFT class based on its id
        pub async fn class(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryClassRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryClassResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cosmos.nft.v1beta1.Query/Class");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("cosmos.nft.v1beta1.Query", "Class"));
            self.inner.unary(req, path, codec).await
        }
        /// Classes queries all NFT classes
        pub async fn classes(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryClassesRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryClassesResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cosmos.nft.v1beta1.Query/Classes");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("cosmos.nft.v1beta1.Query", "Classes"));
            self.inner.unary(req, path, codec).await
        }
    }
}
/// EventSend is emitted on Msg/Send
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventSend {
    /// class_id associated with the nft
    #[prost(string, tag = "1")]
    pub class_id: ::prost::alloc::string::String,
    /// id is a unique identifier of the nft
    #[prost(string, tag = "2")]
    pub id: ::prost::alloc::string::String,
    /// sender is the address of the owner of nft
    #[prost(string, tag = "3")]
    pub sender: ::prost::alloc::string::String,
    /// receiver is the receiver address of nft
    #[prost(string, tag = "4")]
    pub receiver: ::prost::alloc::string::String,
}
/// EventMint is emitted on Mint
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventMint {
    /// class_id associated with the nft
    #[prost(string, tag = "1")]
    pub class_id: ::prost::alloc::string::String,
    /// id is a unique identifier of the nft
    #[prost(string, tag = "2")]
    pub id: ::prost::alloc::string::String,
    /// owner is the owner address of the nft
    #[prost(string, tag = "3")]
    pub owner: ::prost::alloc::string::String,
}
/// EventBurn is emitted on Burn
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EventBurn {
    /// class_id associated with the nft
    #[prost(string, tag = "1")]
    pub class_id: ::prost::alloc::string::String,
    /// id is a unique identifier of the nft
    #[prost(string, tag = "2")]
    pub id: ::prost::alloc::string::String,
    /// owner is the owner address of the nft
    #[prost(string, tag = "3")]
    pub owner: ::prost::alloc::string::String,
}
