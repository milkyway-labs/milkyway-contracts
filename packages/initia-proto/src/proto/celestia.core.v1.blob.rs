/// Blob (named after binary large object) is a chunk of data submitted by a user
/// to be published to the Celestia blockchain. The data of a Blob is published
/// to a namespace and is encoded into shares based on the format specified by
/// share_version.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Blob {
    #[prost(bytes = "vec", tag = "1")]
    pub namespace_id: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint32, tag = "3")]
    pub share_version: u32,
    #[prost(uint32, tag = "4")]
    pub namespace_version: u32,
}
/// BlobTx wraps an encoded sdk.Tx with a second field to contain blobs of data.
/// The raw bytes of the blobs are not signed over, instead we verify each blob
/// using the relevant MsgPayForBlobs that is signed over in the encoded sdk.Tx.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlobTx {
    #[prost(bytes = "vec", tag = "1")]
    pub tx: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, repeated, tag = "2")]
    pub blobs: ::prost::alloc::vec::Vec<Blob>,
    #[prost(string, tag = "3")]
    pub type_id: ::prost::alloc::string::String,
}
