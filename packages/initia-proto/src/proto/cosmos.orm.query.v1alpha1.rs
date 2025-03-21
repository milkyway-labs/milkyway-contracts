/// GetRequest is the Query/Get request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRequest {
    /// message_name is the fully-qualified message name of the ORM table being queried.
    #[prost(string, tag = "1")]
    pub message_name: ::prost::alloc::string::String,
    /// index is the index fields expression used in orm definitions. If it
    /// is empty, the table's primary key is assumed. If it is non-empty, it must
    /// refer to an unique index.
    #[prost(string, tag = "2")]
    pub index: ::prost::alloc::string::String,
    /// values are the values of the fields corresponding to the requested index.
    /// There must be as many values provided as there are fields in the index and
    /// these values must correspond to the index field types.
    #[prost(message, repeated, tag = "3")]
    pub values: ::prost::alloc::vec::Vec<IndexValue>,
}
/// GetResponse is the Query/Get response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetResponse {
    /// result is the result of the get query. If no value is found, the gRPC
    /// status code NOT_FOUND will be returned.
    #[prost(message, optional, tag = "1")]
    pub result: ::core::option::Option<::prost_types::Any>,
}
/// ListRequest is the Query/List request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListRequest {
    /// message_name is the fully-qualified message name of the ORM table being queried.
    #[prost(string, tag = "1")]
    pub message_name: ::prost::alloc::string::String,
    /// index is the index fields expression used in orm definitions. If it
    /// is empty, the table's primary key is assumed.
    #[prost(string, tag = "2")]
    pub index: ::prost::alloc::string::String,
    /// pagination is the pagination request.
    #[prost(message, optional, tag = "5")]
    pub pagination: ::core::option::Option<super::super::super::base::query::v1beta1::PageRequest>,
    /// query is the query expression corresponding to the provided index. If
    /// neither prefix nor range is specified, the query will list all the fields
    /// in the index.
    #[prost(oneof = "list_request::Query", tags = "3, 4")]
    pub query: ::core::option::Option<list_request::Query>,
}
/// Nested message and enum types in `ListRequest`.
pub mod list_request {
    /// Prefix specifies the arguments to a prefix query.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Prefix {
        /// values specifies the index values for the prefix query.
        /// It is valid to special a partial prefix with fewer values than
        /// the number of fields in the index.
        #[prost(message, repeated, tag = "1")]
        pub values: ::prost::alloc::vec::Vec<super::IndexValue>,
    }
    /// Range specifies the arguments to a range query.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Range {
        /// start specifies the starting index values for the range query.
        /// It is valid to provide fewer values than the number of fields in the
        /// index.
        #[prost(message, repeated, tag = "1")]
        pub start: ::prost::alloc::vec::Vec<super::IndexValue>,
        /// end specifies the inclusive ending index values for the range query.
        /// It is valid to provide fewer values than the number of fields in the
        /// index.
        #[prost(message, repeated, tag = "2")]
        pub end: ::prost::alloc::vec::Vec<super::IndexValue>,
    }
    /// query is the query expression corresponding to the provided index. If
    /// neither prefix nor range is specified, the query will list all the fields
    /// in the index.
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Query {
        /// prefix defines a prefix query.
        #[prost(message, tag = "3")]
        Prefix(Prefix),
        /// range defines a range query.
        #[prost(message, tag = "4")]
        Range(Range),
    }
}
/// ListResponse is the Query/List response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListResponse {
    /// results are the results of the query.
    #[prost(message, repeated, tag = "1")]
    pub results: ::prost::alloc::vec::Vec<::prost_types::Any>,
    /// pagination is the pagination response.
    #[prost(message, optional, tag = "5")]
    pub pagination: ::core::option::Option<super::super::super::base::query::v1beta1::PageResponse>,
}
/// IndexValue represents the value of a field in an ORM index expression.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IndexValue {
    /// value specifies the index value
    #[prost(oneof = "index_value::Value", tags = "1, 2, 3, 4, 5, 6, 7, 8")]
    pub value: ::core::option::Option<index_value::Value>,
}
/// Nested message and enum types in `IndexValue`.
pub mod index_value {
    /// value specifies the index value
    #[allow(clippy::derive_partial_eq_without_eq)]
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Value {
        /// uint specifies a value for an uint32, fixed32, uint64, or fixed64
        /// index field.
        #[prost(uint64, tag = "1")]
        Uint(u64),
        /// int64 specifies a value for an int32, sfixed32, int64, or sfixed64
        /// index field.
        #[prost(int64, tag = "2")]
        Int(i64),
        /// str specifies a value for a string index field.
        #[prost(string, tag = "3")]
        Str(::prost::alloc::string::String),
        /// bytes specifies a value for a bytes index field.
        #[prost(bytes, tag = "4")]
        Bytes(::prost::alloc::vec::Vec<u8>),
        /// enum specifies a value for an enum index field.
        #[prost(string, tag = "5")]
        Enum(::prost::alloc::string::String),
        /// bool specifies a value for a bool index field.
        #[prost(bool, tag = "6")]
        Bool(bool),
        /// timestamp specifies a value for a timestamp index field.
        #[prost(message, tag = "7")]
        Timestamp(::prost_types::Timestamp),
        /// duration specifies a value for a duration index field.
        #[prost(message, tag = "8")]
        Duration(::prost_types::Duration),
    }
}
/// Generated client implementations.
#[cfg(feature = "grpc")]
#[cfg_attr(docsrs, doc(cfg(feature = "grpc")))]
pub mod query_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    /// Query is a generic gRPC service for querying ORM data.
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
        /// Get queries an ORM table against an unique index.
        pub async fn get(
            &mut self,
            request: impl tonic::IntoRequest<super::GetRequest>,
        ) -> std::result::Result<tonic::Response<super::GetResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/cosmos.orm.query.v1alpha1.Query/Get");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("cosmos.orm.query.v1alpha1.Query", "Get"));
            self.inner.unary(req, path, codec).await
        }
        /// List queries an ORM table against an index.
        pub async fn list(
            &mut self,
            request: impl tonic::IntoRequest<super::ListRequest>,
        ) -> std::result::Result<tonic::Response<super::ListResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/cosmos.orm.query.v1alpha1.Query/List");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("cosmos.orm.query.v1alpha1.Query", "List"));
            self.inner.unary(req, path, codec).await
        }
    }
}
