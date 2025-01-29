/// Params defines the set of move parameters.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    #[prost(string, tag = "1")]
    pub base_denom: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub base_min_gas_price: ::prost::alloc::string::String,
    /// CSR: Percentage of fees distributed to developers
    #[prost(string, tag = "3")]
    pub contract_shared_revenue_ratio: ::prost::alloc::string::String,
    /// flag whether to enable script execution
    #[prost(bool, tag = "4")]
    pub script_enabled: bool,
    /// It is a list of addresses with permission to distribute contracts,
    /// and an empty list is interpreted as allowing anyone to distribute.
    #[prost(string, repeated, tag = "5")]
    pub allowed_publishers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// RawParams defines the raw params to store.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RawParams {
    #[prost(string, tag = "1")]
    pub base_denom: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub base_min_gas_price: ::prost::alloc::string::String,
    /// CSR: Percentage of fees distributed to developers
    #[prost(string, tag = "3")]
    pub contract_shared_revenue_ratio: ::prost::alloc::string::String,
    /// flag whether to enable script execution
    #[prost(bool, tag = "4")]
    pub script_enabled: bool,
}
/// Module is data for the uploaded contract move code
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Module {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub module_name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub abi: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "4")]
    pub raw_bytes: ::prost::alloc::vec::Vec<u8>,
    #[prost(enumeration = "UpgradePolicy", tag = "5")]
    pub upgrade_policy: i32,
}
/// Checksum is checksum of the uploaded contract move code
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Checksum {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub module_name: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "3")]
    pub checksum: ::prost::alloc::vec::Vec<u8>,
}
/// Resource is data for the stored move resource
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Resource {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub struct_tag: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub move_resource: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "4")]
    pub raw_bytes: ::prost::alloc::vec::Vec<u8>,
}
/// TableInfo is data stored under Table address
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TableInfo {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub key_type: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub value_type: ::prost::alloc::string::String,
}
/// TableEntry is data stored under Table address and the key bytes
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TableEntry {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub key: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub value: ::prost::alloc::string::String,
    #[prost(bytes = "vec", tag = "4")]
    pub key_bytes: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "5")]
    pub value_bytes: ::prost::alloc::vec::Vec<u8>,
}
/// proto wrapper to store the value
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpgradePolicyProto {
    #[prost(enumeration = "UpgradePolicy", tag = "1")]
    pub policy: i32,
}
/// DexPair contains coin metadata address
/// std::dex::Pool and std::dex::Config resources.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DexPair {
    #[prost(string, tag = "1")]
    pub metadata_quote: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub metadata_lp: ::prost::alloc::string::String,
}
/// ExecuteAuthorizationItem is the information for granting module execution
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecuteAuthorizationItem {
    /// ModuleAddr is the address of the module deployer
    #[prost(string, tag = "1")]
    pub module_address: ::prost::alloc::string::String,
    /// ModuleName is the names of module to execute
    #[prost(string, tag = "2")]
    pub module_name: ::prost::alloc::string::String,
    /// FunctionName is the name of function to execute with wildcard '*' support
    #[prost(string, repeated, tag = "3")]
    pub function_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// UpgradePolicy is the policy for upgrading a move module.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum UpgradePolicy {
    /// UNSPECIFIED: a placeholder for an unspecified upgrade policy.
    Unspecified = 0,
    /// COMPATIBLE: Whether a compatibility check should be performed for upgrades. The check only passes if
    /// a new module has (a) the same public functions (b) for existing resources, no layout change.
    Compatible = 1,
    /// IMMUTABLE: Whether the modules in the package are immutable and cannot be upgraded.
    Immutable = 2,
}
impl UpgradePolicy {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            UpgradePolicy::Unspecified => "UNSPECIFIED",
            UpgradePolicy::Compatible => "COMPATIBLE",
            UpgradePolicy::Immutable => "IMMUTABLE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNSPECIFIED" => Some(Self::Unspecified),
            "COMPATIBLE" => Some(Self::Compatible),
            "IMMUTABLE" => Some(Self::Immutable),
            _ => None,
        }
    }
}
/// GenesisState - genesis state of x/move
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    #[prost(uint64, tag = "2")]
    pub execution_counter: u64,
    #[prost(bytes = "vec", repeated, tag = "3")]
    pub stdlibs: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(message, repeated, tag = "4")]
    pub modules: ::prost::alloc::vec::Vec<Module>,
    #[prost(message, repeated, tag = "5")]
    pub checksums: ::prost::alloc::vec::Vec<Checksum>,
    #[prost(message, repeated, tag = "6")]
    pub resources: ::prost::alloc::vec::Vec<Resource>,
    #[prost(message, repeated, tag = "7")]
    pub table_infos: ::prost::alloc::vec::Vec<TableInfo>,
    #[prost(message, repeated, tag = "8")]
    pub table_entries: ::prost::alloc::vec::Vec<TableEntry>,
    #[prost(message, repeated, tag = "9")]
    pub dex_pairs: ::prost::alloc::vec::Vec<DexPair>,
}
/// MsgPublish is the message to store compiled Move module
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgPublish {
    /// Sender is the that actor that signed the messages
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    /// CodeBytes is raw move module bytes code
    #[prost(bytes = "vec", repeated, tag = "2")]
    pub code_bytes: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    /// UpgradePolicy defines upgrade rules which will be applied
    /// at next publish message.
    /// Upgrades in the direction of enhancing security are permitted.
    /// `ARBITRARY` => `COMPATIBLE`
    /// `ARBITRARY` => `IMMUTABLE`
    /// `COMPATIBLE` => `IMMUTABLE`
    /// but reverse ways are not allowed (ignored).
    #[prost(enumeration = "UpgradePolicy", tag = "3")]
    pub upgrade_policy: i32,
}
/// MsgPublishResponse returns store result data.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgPublishResponse {}
/// MsgExecute is the message to execute the given module function
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgExecute {
    /// Sender is the that actor that signed the messages
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    /// ModuleAddr is the address of the module deployer
    #[prost(string, tag = "2")]
    pub module_address: ::prost::alloc::string::String,
    /// ModuleName is the name of module to execute
    #[prost(string, tag = "3")]
    pub module_name: ::prost::alloc::string::String,
    /// FunctionName is the name of a function to execute
    #[prost(string, tag = "4")]
    pub function_name: ::prost::alloc::string::String,
    /// TypeArgs is the type arguments of a function to execute
    /// ex) "0x1::BasicCoin::Initia", "bool", "u8", "u64"
    #[prost(string, repeated, tag = "5")]
    pub type_args: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Args is the arguments of a function to execute
    /// - number: little endian
    /// - string: base64 bytes
    #[prost(bytes = "vec", repeated, tag = "6")]
    pub args: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// MsgExecuteResponse returns execution result data.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgExecuteResponse {}
/// MsgExecuteJSON is the message to execute the given module function
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgExecuteJson {
    /// Sender is the that actor that signed the messages
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    /// ModuleAddr is the address of the module deployer
    #[prost(string, tag = "2")]
    pub module_address: ::prost::alloc::string::String,
    /// ModuleName is the name of module to execute
    #[prost(string, tag = "3")]
    pub module_name: ::prost::alloc::string::String,
    /// FunctionName is the name of a function to execute
    #[prost(string, tag = "4")]
    pub function_name: ::prost::alloc::string::String,
    /// TypeArgs is the type arguments of a function to execute
    /// ex) "0x1::BasicCoin::Initia", "bool", "u8", "u64"
    #[prost(string, repeated, tag = "5")]
    pub type_args: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Args is the arguments of a function to execute in json stringify format
    #[prost(string, repeated, tag = "6")]
    pub args: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// MsgExecuteJSONResponse returns execution result data.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgExecuteJsonResponse {}
/// MsgScript is the message to execute script code with sender as signer
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgScript {
    /// Sender is the that actor that signed the messages
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    /// CodeBytes is the script bytes code to execute
    #[prost(bytes = "vec", tag = "2")]
    pub code_bytes: ::prost::alloc::vec::Vec<u8>,
    /// TypeArgs is the type arguments of a function to execute
    /// ex) "0x1::BasicCoin::Initia", "bool", "u8", "u64"
    #[prost(string, repeated, tag = "3")]
    pub type_args: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Args is the arguments of a function to execute
    /// - number: little endian
    /// - string: base64 bytes
    #[prost(bytes = "vec", repeated, tag = "4")]
    pub args: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// MsgScriptResponse returns execution result data.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgScriptResponse {}
/// MsgScriptJSON is the message to execute script code with sender as signer
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgScriptJson {
    /// Sender is the that actor that signed the messages
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    /// CodeBytes is the script bytes code to execute
    #[prost(bytes = "vec", tag = "2")]
    pub code_bytes: ::prost::alloc::vec::Vec<u8>,
    /// TypeArgs is the type arguments of a function to execute
    /// ex) "0x1::BasicCoin::Initia", "bool", "u8", "u64"
    #[prost(string, repeated, tag = "3")]
    pub type_args: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Args is the arguments of a function to execute in json stringify format
    #[prost(string, repeated, tag = "4")]
    pub args: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// MsgScriptJSONResponse returns execution result data.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgScriptJsonResponse {}
/// MsgGovPublish is the message to store compiled Move module via gov proposal
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgGovPublish {
    /// authority is the address that controls the module
    /// (defaults to x/gov unless overwritten).
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// Sender is the that actor that signed the messages
    #[prost(string, tag = "2")]
    pub sender: ::prost::alloc::string::String,
    /// CodeBytes is raw move module bytes code
    #[prost(bytes = "vec", repeated, tag = "3")]
    pub code_bytes: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    /// UpgradePolicy defines upgrade rules which will be applied
    /// at next publish message.
    /// Upgrades in the direction of enhancing security are permitted.
    /// `ARBITRARY` => `COMPATIBLE`
    /// `ARBITRARY` => `IMMUTABLE`
    /// `COMPATIBLE` => `IMMUTABLE`
    /// but reverse ways are not allowed (ignored).
    #[prost(enumeration = "UpgradePolicy", tag = "4")]
    pub upgrade_policy: i32,
}
/// MsgGovPublishResponse returns execution result data.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgGovPublishResponse {}
/// MsgGovExecute is the message to execute the given module
/// function via gov proposal
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgGovExecute {
    /// authority is the address that controls the module
    /// (defaults to x/gov unless overwritten).
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// Sender is the that actor that signed the messages
    #[prost(string, tag = "2")]
    pub sender: ::prost::alloc::string::String,
    /// ModuleAddr is the address of the module deployer
    #[prost(string, tag = "3")]
    pub module_address: ::prost::alloc::string::String,
    /// ModuleName is the name of module to execute
    #[prost(string, tag = "4")]
    pub module_name: ::prost::alloc::string::String,
    /// FunctionName is the name of a function to execute
    #[prost(string, tag = "5")]
    pub function_name: ::prost::alloc::string::String,
    /// TypeArgs is the type arguments of a function to execute
    /// ex) "0x1::BasicCoin::Initia", "bool", "u8", "u64"
    #[prost(string, repeated, tag = "6")]
    pub type_args: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Args is the arguments of a function to execute
    /// - number: little endian
    /// - string: base64 bytes
    #[prost(bytes = "vec", repeated, tag = "7")]
    pub args: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// MsgGovExecuteResponse returns execution result data.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgGovExecuteResponse {}
/// MsgGovExecuteJSON is the message to execute the given module
/// function via gov proposal
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgGovExecuteJson {
    /// authority is the address that controls the module
    /// (defaults to x/gov unless overwritten).
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// Sender is the that actor that signed the messages
    #[prost(string, tag = "2")]
    pub sender: ::prost::alloc::string::String,
    /// ModuleAddr is the address of the module deployer
    #[prost(string, tag = "3")]
    pub module_address: ::prost::alloc::string::String,
    /// ModuleName is the name of module to execute
    #[prost(string, tag = "4")]
    pub module_name: ::prost::alloc::string::String,
    /// FunctionName is the name of a function to execute
    #[prost(string, tag = "5")]
    pub function_name: ::prost::alloc::string::String,
    /// TypeArgs is the type arguments of a function to execute
    /// ex) "0x1::BasicCoin::Initia", "bool", "u8", "u64"
    #[prost(string, repeated, tag = "6")]
    pub type_args: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Args is the arguments of a function to execute in json stringify format
    #[prost(string, repeated, tag = "7")]
    pub args: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// MsgGovExecuteJSONResponse returns execution result data.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgGovExecuteJsonResponse {}
/// MsgGovScript is the message to execute script code with sender as signer via gov
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgGovScript {
    /// authority is the address that controls the module
    /// (defaults to x/gov unless overwritten).
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// Sender is the that actor that signed the messages
    #[prost(string, tag = "2")]
    pub sender: ::prost::alloc::string::String,
    /// CodeBytes is the script bytes code to execute
    #[prost(bytes = "vec", tag = "3")]
    pub code_bytes: ::prost::alloc::vec::Vec<u8>,
    /// TypeArgs is the type arguments of a function to execute
    /// ex) "0x1::BasicCoin::Initia", "bool", "u8", "u64"
    #[prost(string, repeated, tag = "4")]
    pub type_args: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Args is the arguments of a function to execute
    /// - number: little endian
    /// - string: base64 bytes
    #[prost(bytes = "vec", repeated, tag = "5")]
    pub args: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// MsgGovScriptResponse returns execution result data.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgGovScriptResponse {}
/// MsgGovScriptJSON is the message to execute script code with sender as signer via gov
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgGovScriptJson {
    /// authority is the address that controls the module
    /// (defaults to x/gov unless overwritten).
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// Sender is the that actor that signed the messages
    #[prost(string, tag = "2")]
    pub sender: ::prost::alloc::string::String,
    /// CodeBytes is the script bytes code to execute
    #[prost(bytes = "vec", tag = "3")]
    pub code_bytes: ::prost::alloc::vec::Vec<u8>,
    /// TypeArgs is the type arguments of a function to execute
    /// ex) "0x1::BasicCoin::Initia", "bool", "u8", "u64"
    #[prost(string, repeated, tag = "4")]
    pub type_args: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Args is the arguments of a function to execute in json stringify format
    #[prost(string, repeated, tag = "5")]
    pub args: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// MsgGovScriptJSONResponse returns execution result data.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgGovScriptJsonResponse {}
/// MsgWhitelist is a message to register a dex pair to
/// whitelist of various features.
/// - whitelist from coin register operation
/// - allow counter party denom can be used as gas fee
/// - register lp denom as staking denom
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgWhitelist {
    /// authority is the address that controls the module
    /// (defaults to x/gov unless overwritten).
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// Dex coin LP metadata address
    #[prost(string, tag = "2")]
    pub metadata_lp: ::prost::alloc::string::String,
    /// RewardWeight is registered to distribution's Params
    #[prost(string, tag = "3")]
    pub reward_weight: ::prost::alloc::string::String,
}
/// MsgWhitelistResponse returns result data.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgWhitelistResponse {}
/// MsgDelist is a message to unregister a dex pair
/// from the whitelist of various features.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDelist {
    /// authority is the address that controls the module
    /// (defaults to x/gov unless overwritten).
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// Dex coin LP metadata address
    #[prost(string, tag = "2")]
    pub metadata_lp: ::prost::alloc::string::String,
}
/// MsgDelistResponse returns result data.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgDelistResponse {}
/// MsgUpdateParams is the Msg/UpdateParams request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateParams {
    /// authority is the address that controls the module
    /// (defaults to x/gov unless overwritten).
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// params defines the x/staking parameters to update.
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
    /// Msg defines the move Msg service.
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
        /// Publish stores compiled Move module
        pub async fn publish(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgPublish>,
        ) -> std::result::Result<tonic::Response<super::MsgPublishResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/initia.move.v1.Msg/Publish");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("initia.move.v1.Msg", "Publish"));
            self.inner.unary(req, path, codec).await
        }
        /// Deprecated: Use ExecuteJSON instead
        /// Execute runs a entry function with the given message
        pub async fn execute(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgExecute>,
        ) -> std::result::Result<tonic::Response<super::MsgExecuteResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/initia.move.v1.Msg/Execute");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("initia.move.v1.Msg", "Execute"));
            self.inner.unary(req, path, codec).await
        }
        /// ExecuteJSON runs a entry function with the given message
        pub async fn execute_json(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgExecuteJson>,
        ) -> std::result::Result<tonic::Response<super::MsgExecuteJsonResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/initia.move.v1.Msg/ExecuteJSON");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("initia.move.v1.Msg", "ExecuteJSON"));
            self.inner.unary(req, path, codec).await
        }
        /// Deprecated: Use ScriptJSON instead
        /// Script runs a scripts with the given message
        pub async fn script(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgScript>,
        ) -> std::result::Result<tonic::Response<super::MsgScriptResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/initia.move.v1.Msg/Script");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("initia.move.v1.Msg", "Script"));
            self.inner.unary(req, path, codec).await
        }
        /// ScriptJSON runs a scripts with the given message
        pub async fn script_json(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgScriptJson>,
        ) -> std::result::Result<tonic::Response<super::MsgScriptJsonResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/initia.move.v1.Msg/ScriptJSON");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("initia.move.v1.Msg", "ScriptJSON"));
            self.inner.unary(req, path, codec).await
        }
        /// GovPublish stores compiled Move module via gov proposal
        pub async fn gov_publish(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgGovPublish>,
        ) -> std::result::Result<tonic::Response<super::MsgGovPublishResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/initia.move.v1.Msg/GovPublish");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("initia.move.v1.Msg", "GovPublish"));
            self.inner.unary(req, path, codec).await
        }
        /// Deprecated: Use GovExecuteJSON instead
        /// GovExecute runs a entry function with the given message via gov proposal
        pub async fn gov_execute(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgGovExecute>,
        ) -> std::result::Result<tonic::Response<super::MsgGovExecuteResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/initia.move.v1.Msg/GovExecute");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("initia.move.v1.Msg", "GovExecute"));
            self.inner.unary(req, path, codec).await
        }
        /// GovExecuteJSON runs a entry function with the given message via gov proposal
        pub async fn gov_execute_json(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgGovExecuteJson>,
        ) -> std::result::Result<tonic::Response<super::MsgGovExecuteJsonResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/initia.move.v1.Msg/GovExecuteJSON");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("initia.move.v1.Msg", "GovExecuteJSON"));
            self.inner.unary(req, path, codec).await
        }
        /// Deprecated: Use GovScriptJSON instead
        /// GovScript runs a scripts with the given message via gov proposal
        pub async fn gov_script(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgGovScript>,
        ) -> std::result::Result<tonic::Response<super::MsgGovScriptResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/initia.move.v1.Msg/GovScript");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("initia.move.v1.Msg", "GovScript"));
            self.inner.unary(req, path, codec).await
        }
        /// GovScriptJSON runs a scripts with the given message via gov proposal
        pub async fn gov_script_json(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgGovScriptJson>,
        ) -> std::result::Result<tonic::Response<super::MsgGovScriptJsonResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/initia.move.v1.Msg/GovScriptJSON");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("initia.move.v1.Msg", "GovScriptJSON"));
            self.inner.unary(req, path, codec).await
        }
        /// Whitelist registers a dex pair to whitelist of various features.
        /// - whitelist from coin register operation
        /// - allow counter party denom can be used as gas fee
        /// - register lp denom as staking denom
        pub async fn whitelist(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgWhitelist>,
        ) -> std::result::Result<tonic::Response<super::MsgWhitelistResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/initia.move.v1.Msg/Whitelist");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("initia.move.v1.Msg", "Whitelist"));
            self.inner.unary(req, path, codec).await
        }
        /// Delist unregisters a dex pair from the whitelist.
        pub async fn delist(
            &mut self,
            request: impl tonic::IntoRequest<super::MsgDelist>,
        ) -> std::result::Result<tonic::Response<super::MsgDelistResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/initia.move.v1.Msg/Delist");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("initia.move.v1.Msg", "Delist"));
            self.inner.unary(req, path, codec).await
        }
        /// UpdateParams defines an operation for updating the x/move module
        /// parameters.
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
            let path = http::uri::PathAndQuery::from_static("/initia.move.v1.Msg/UpdateParams");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("initia.move.v1.Msg", "UpdateParams"));
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
        /// Publish stores compiled Move module
        async fn publish(
            &self,
            request: tonic::Request<super::MsgPublish>,
        ) -> std::result::Result<tonic::Response<super::MsgPublishResponse>, tonic::Status>;
        /// Deprecated: Use ExecuteJSON instead
        /// Execute runs a entry function with the given message
        async fn execute(
            &self,
            request: tonic::Request<super::MsgExecute>,
        ) -> std::result::Result<tonic::Response<super::MsgExecuteResponse>, tonic::Status>;
        /// ExecuteJSON runs a entry function with the given message
        async fn execute_json(
            &self,
            request: tonic::Request<super::MsgExecuteJson>,
        ) -> std::result::Result<tonic::Response<super::MsgExecuteJsonResponse>, tonic::Status>;
        /// Deprecated: Use ScriptJSON instead
        /// Script runs a scripts with the given message
        async fn script(
            &self,
            request: tonic::Request<super::MsgScript>,
        ) -> std::result::Result<tonic::Response<super::MsgScriptResponse>, tonic::Status>;
        /// ScriptJSON runs a scripts with the given message
        async fn script_json(
            &self,
            request: tonic::Request<super::MsgScriptJson>,
        ) -> std::result::Result<tonic::Response<super::MsgScriptJsonResponse>, tonic::Status>;
        /// GovPublish stores compiled Move module via gov proposal
        async fn gov_publish(
            &self,
            request: tonic::Request<super::MsgGovPublish>,
        ) -> std::result::Result<tonic::Response<super::MsgGovPublishResponse>, tonic::Status>;
        /// Deprecated: Use GovExecuteJSON instead
        /// GovExecute runs a entry function with the given message via gov proposal
        async fn gov_execute(
            &self,
            request: tonic::Request<super::MsgGovExecute>,
        ) -> std::result::Result<tonic::Response<super::MsgGovExecuteResponse>, tonic::Status>;
        /// GovExecuteJSON runs a entry function with the given message via gov proposal
        async fn gov_execute_json(
            &self,
            request: tonic::Request<super::MsgGovExecuteJson>,
        ) -> std::result::Result<tonic::Response<super::MsgGovExecuteJsonResponse>, tonic::Status>;
        /// Deprecated: Use GovScriptJSON instead
        /// GovScript runs a scripts with the given message via gov proposal
        async fn gov_script(
            &self,
            request: tonic::Request<super::MsgGovScript>,
        ) -> std::result::Result<tonic::Response<super::MsgGovScriptResponse>, tonic::Status>;
        /// GovScriptJSON runs a scripts with the given message via gov proposal
        async fn gov_script_json(
            &self,
            request: tonic::Request<super::MsgGovScriptJson>,
        ) -> std::result::Result<tonic::Response<super::MsgGovScriptJsonResponse>, tonic::Status>;
        /// Whitelist registers a dex pair to whitelist of various features.
        /// - whitelist from coin register operation
        /// - allow counter party denom can be used as gas fee
        /// - register lp denom as staking denom
        async fn whitelist(
            &self,
            request: tonic::Request<super::MsgWhitelist>,
        ) -> std::result::Result<tonic::Response<super::MsgWhitelistResponse>, tonic::Status>;
        /// Delist unregisters a dex pair from the whitelist.
        async fn delist(
            &self,
            request: tonic::Request<super::MsgDelist>,
        ) -> std::result::Result<tonic::Response<super::MsgDelistResponse>, tonic::Status>;
        /// UpdateParams defines an operation for updating the x/move module
        /// parameters.
        async fn update_params(
            &self,
            request: tonic::Request<super::MsgUpdateParams>,
        ) -> std::result::Result<tonic::Response<super::MsgUpdateParamsResponse>, tonic::Status>;
    }
    /// Msg defines the move Msg service.
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
                "/initia.move.v1.Msg/Publish" => {
                    #[allow(non_camel_case_types)]
                    struct PublishSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgPublish> for PublishSvc<T> {
                        type Response = super::MsgPublishResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgPublish>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).publish(request).await };
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
                        let method = PublishSvc(inner);
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
                "/initia.move.v1.Msg/Execute" => {
                    #[allow(non_camel_case_types)]
                    struct ExecuteSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgExecute> for ExecuteSvc<T> {
                        type Response = super::MsgExecuteResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgExecute>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).execute(request).await };
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
                        let method = ExecuteSvc(inner);
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
                "/initia.move.v1.Msg/ExecuteJSON" => {
                    #[allow(non_camel_case_types)]
                    struct ExecuteJSONSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgExecuteJson> for ExecuteJSONSvc<T> {
                        type Response = super::MsgExecuteJsonResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgExecuteJson>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).execute_json(request).await };
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
                        let method = ExecuteJSONSvc(inner);
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
                "/initia.move.v1.Msg/Script" => {
                    #[allow(non_camel_case_types)]
                    struct ScriptSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgScript> for ScriptSvc<T> {
                        type Response = super::MsgScriptResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgScript>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).script(request).await };
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
                        let method = ScriptSvc(inner);
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
                "/initia.move.v1.Msg/ScriptJSON" => {
                    #[allow(non_camel_case_types)]
                    struct ScriptJSONSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgScriptJson> for ScriptJSONSvc<T> {
                        type Response = super::MsgScriptJsonResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgScriptJson>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).script_json(request).await };
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
                        let method = ScriptJSONSvc(inner);
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
                "/initia.move.v1.Msg/GovPublish" => {
                    #[allow(non_camel_case_types)]
                    struct GovPublishSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgGovPublish> for GovPublishSvc<T> {
                        type Response = super::MsgGovPublishResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgGovPublish>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).gov_publish(request).await };
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
                        let method = GovPublishSvc(inner);
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
                "/initia.move.v1.Msg/GovExecute" => {
                    #[allow(non_camel_case_types)]
                    struct GovExecuteSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgGovExecute> for GovExecuteSvc<T> {
                        type Response = super::MsgGovExecuteResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgGovExecute>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).gov_execute(request).await };
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
                        let method = GovExecuteSvc(inner);
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
                "/initia.move.v1.Msg/GovExecuteJSON" => {
                    #[allow(non_camel_case_types)]
                    struct GovExecuteJSONSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgGovExecuteJson> for GovExecuteJSONSvc<T> {
                        type Response = super::MsgGovExecuteJsonResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgGovExecuteJson>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).gov_execute_json(request).await };
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
                        let method = GovExecuteJSONSvc(inner);
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
                "/initia.move.v1.Msg/GovScript" => {
                    #[allow(non_camel_case_types)]
                    struct GovScriptSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgGovScript> for GovScriptSvc<T> {
                        type Response = super::MsgGovScriptResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgGovScript>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).gov_script(request).await };
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
                        let method = GovScriptSvc(inner);
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
                "/initia.move.v1.Msg/GovScriptJSON" => {
                    #[allow(non_camel_case_types)]
                    struct GovScriptJSONSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgGovScriptJson> for GovScriptJSONSvc<T> {
                        type Response = super::MsgGovScriptJsonResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgGovScriptJson>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).gov_script_json(request).await };
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
                        let method = GovScriptJSONSvc(inner);
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
                "/initia.move.v1.Msg/Whitelist" => {
                    #[allow(non_camel_case_types)]
                    struct WhitelistSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgWhitelist> for WhitelistSvc<T> {
                        type Response = super::MsgWhitelistResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgWhitelist>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).whitelist(request).await };
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
                        let method = WhitelistSvc(inner);
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
                "/initia.move.v1.Msg/Delist" => {
                    #[allow(non_camel_case_types)]
                    struct DelistSvc<T: Msg>(pub Arc<T>);
                    impl<T: Msg> tonic::server::UnaryService<super::MsgDelist> for DelistSvc<T> {
                        type Response = super::MsgDelistResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MsgDelist>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).delist(request).await };
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
                        let method = DelistSvc(inner);
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
                "/initia.move.v1.Msg/UpdateParams" => {
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
        const NAME: &'static str = "initia.move.v1.Msg";
    }
}
/// PublishAuthorization defines authorization for publish a move module.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PublishAuthorization {
    #[prost(string, repeated, tag = "1")]
    pub module_names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// ExecuteAuthorization defines authorization for execute a move function.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecuteAuthorization {
    #[prost(message, repeated, tag = "1")]
    pub items: ::prost::alloc::vec::Vec<ExecuteAuthorizationItem>,
}
/// QueryModuleRequest is the request type for the Query/Module RPC
/// method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryModuleRequest {
    /// address is the owner address of the module to query
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// module_name is the module name to query
    #[prost(string, tag = "2")]
    pub module_name: ::prost::alloc::string::String,
}
/// QueryModuleResponse is the response type for the Query/Module RPC
/// method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryModuleResponse {
    #[prost(message, optional, tag = "1")]
    pub module: ::core::option::Option<Module>,
}
/// QueryModulesRequest is the request type for the Query/Modules
/// RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryModulesRequest {
    /// address is the owner address of the modules to query
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryModulesResponse is the response type for the
/// Query/Modules RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryModulesResponse {
    #[prost(message, repeated, tag = "1")]
    pub modules: ::prost::alloc::vec::Vec<Module>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryResourceRequest is the request type for the Query/Resource RPC
/// method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryResourceRequest {
    /// address is the owner address of the module to query
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// struct_tag is the unique identifier of the resource to query
    #[prost(string, tag = "2")]
    pub struct_tag: ::prost::alloc::string::String,
}
/// QueryResourceResponse is the response type for the Query/Resource RPC
/// method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryResourceResponse {
    #[prost(message, optional, tag = "1")]
    pub resource: ::core::option::Option<Resource>,
}
/// QueryResourcesRequest is the request type for the Query/Resources RPC
/// method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryResourcesRequest {
    /// address is the owner address of the module to query
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryResourcesResponse is the response type for the Query/Resources RPC
/// method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryResourcesResponse {
    #[prost(message, repeated, tag = "1")]
    pub resources: ::prost::alloc::vec::Vec<Resource>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryTableInfoRequest is the request type for the Query/TableInfo RPC
/// method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTableInfoRequest {
    /// address is the table handle
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
/// QueryTableInfoResponse is the response type for the Query/TableInfo RPC
/// method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTableInfoResponse {
    #[prost(message, optional, tag = "1")]
    pub table_info: ::core::option::Option<TableInfo>,
}
/// QueryTableEntryRequest is the request type for the Query/TableEntry RPC
/// method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTableEntryRequest {
    /// address is the table handle
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// a key of the table entry
    #[prost(bytes = "vec", tag = "2")]
    pub key_bytes: ::prost::alloc::vec::Vec<u8>,
}
/// QueryTableEntryResponse is the response type for the Query/TableEntry RPC
/// method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTableEntryResponse {
    #[prost(message, optional, tag = "1")]
    pub table_entry: ::core::option::Option<TableEntry>,
}
/// QueryTableEntriesRequest is the request type for the Query/TableEntries RPC
/// method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTableEntriesRequest {
    /// address is the table handle
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// pagination defines an optional pagination for the request.
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageRequest>,
}
/// QueryTableEntriesResponse is the response type for the Query/TableEntries RPC
/// method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTableEntriesResponse {
    #[prost(message, repeated, tag = "1")]
    pub table_entries: ::prost::alloc::vec::Vec<TableEntry>,
    /// pagination defines the pagination in the response.
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::super::cosmos::base::query::v1beta1::PageResponse>,
}
/// QueryLegacyViewRequest is the request type for the QueryLegacyView
/// RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLegacyViewRequest {
    /// Address is the owner address of the module to query
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// ModuleName is the module name of the entry function to query
    #[prost(string, tag = "2")]
    pub module_name: ::prost::alloc::string::String,
    /// FunctionName is the name of a function to query
    #[prost(string, tag = "3")]
    pub function_name: ::prost::alloc::string::String,
    /// TypeArgs is the type arguments of a function to execute
    /// ex) "0x1::BasicCoin::Initia", "bool", "u8", "u64"
    #[prost(string, repeated, tag = "4")]
    pub type_args: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Args is the arguments of a function to execute
    /// - number: little endian
    /// - string: base64 bytes
    #[prost(bytes = "vec", repeated, tag = "5")]
    pub args: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// QueryLegacyViewResponse is the response type for the
/// QueryLegacyView RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLegacyViewResponse {
    #[prost(string, tag = "1")]
    pub data: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub events: ::prost::alloc::vec::Vec<VmEvent>,
    #[prost(uint64, tag = "3")]
    pub gas_used: u64,
}
/// QueryViewRequest is the request type for the QueryView
/// RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryViewRequest {
    /// Address is the owner address of the module to query
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// ModuleName is the module name of the entry function to query
    #[prost(string, tag = "2")]
    pub module_name: ::prost::alloc::string::String,
    /// FunctionName is the name of a function to query
    #[prost(string, tag = "3")]
    pub function_name: ::prost::alloc::string::String,
    /// TypeArgs is the type arguments of a function to execute
    /// ex) "0x1::BasicCoin::Initia", "bool", "u8", "u64"
    #[prost(string, repeated, tag = "4")]
    pub type_args: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Args is the arguments of a function to execute
    /// - number: little endian
    /// - string: base64 bytes
    #[prost(bytes = "vec", repeated, tag = "5")]
    pub args: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// QueryViewResponse is the response type for the
/// QueryView RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryViewResponse {
    #[prost(string, tag = "1")]
    pub data: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub events: ::prost::alloc::vec::Vec<VmEvent>,
    #[prost(uint64, tag = "3")]
    pub gas_used: u64,
}
/// QueryViewBatchRequest is the request type for the QueryViewBatch
/// RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryViewBatchRequest {
    #[prost(message, repeated, tag = "1")]
    pub requests: ::prost::alloc::vec::Vec<QueryViewRequest>,
}
/// QueryViewBatchResponse is the response type for the
/// QueryViewBatch RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryViewBatchResponse {
    #[prost(message, repeated, tag = "1")]
    pub responses: ::prost::alloc::vec::Vec<QueryViewResponse>,
}
/// QueryViewJSONRequest is the request type for the QueryViewJSON
/// RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryViewJsonRequest {
    /// Address is the owner address of the module to query
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// ModuleName is the module name of the entry function to query
    #[prost(string, tag = "2")]
    pub module_name: ::prost::alloc::string::String,
    /// FunctionName is the name of a function to query
    #[prost(string, tag = "3")]
    pub function_name: ::prost::alloc::string::String,
    /// TypeArgs is the type arguments of a function to execute
    /// ex) "0x1::BasicCoin::Initia", "bool", "u8", "u64"
    #[prost(string, repeated, tag = "4")]
    pub type_args: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Args is the arguments of a function to execute in json stringify format
    #[prost(string, repeated, tag = "5")]
    pub args: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// QueryViewJSONResponse is the response type for the
/// QueryViewJSON RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryViewJsonResponse {
    #[prost(string, tag = "1")]
    pub data: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub events: ::prost::alloc::vec::Vec<VmEvent>,
    #[prost(uint64, tag = "3")]
    pub gas_used: u64,
}
/// QueryViewJSONBatchRequest is the request type for the QueryViewJSONBatch
/// RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryViewJsonBatchRequest {
    #[prost(message, repeated, tag = "1")]
    pub requests: ::prost::alloc::vec::Vec<QueryViewJsonRequest>,
}
/// QueryViewJSONBatchResponse is the response type for the
/// QueryViewJSONBatch RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryViewJsonBatchResponse {
    #[prost(message, repeated, tag = "1")]
    pub responses: ::prost::alloc::vec::Vec<QueryViewJsonResponse>,
}
/// VMEvent is the event emitted from vm.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VmEvent {
    #[prost(string, tag = "1")]
    pub type_tag: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub data: ::prost::alloc::string::String,
}
/// QueryScriptABIRequest is the request type for the Query/ScriptABI
/// RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryScriptAbiRequest {
    /// CodeBytes is the script code for query operation
    #[prost(bytes = "vec", tag = "1")]
    pub code_bytes: ::prost::alloc::vec::Vec<u8>,
}
/// QueryScriptABIResponse is the response type for the
/// Query/ScriptABI RPC method
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryScriptAbiResponse {
    #[prost(bytes = "vec", tag = "1")]
    pub abi: ::prost::alloc::vec::Vec<u8>,
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
/// QueryMetadataRequest is the request type for the Query/Metadata RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryMetadataRequest {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
}
/// QueryMetadataResponse is the response type for the Query/Metadata RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryMetadataResponse {
    #[prost(string, tag = "1")]
    pub metadata: ::prost::alloc::string::String,
}
/// QueryDenomRequest is the request type for the Query/Denom RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDenomRequest {
    #[prost(string, tag = "1")]
    pub metadata: ::prost::alloc::string::String,
}
/// QueryDenomResponse is the response type for the Query/Denom RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDenomResponse {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
}
/// Generated client implementations.
#[cfg(feature = "grpc")]
#[cfg_attr(docsrs, doc(cfg(feature = "grpc")))]
pub mod query_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::http::Uri;
    use tonic::codegen::*;
    /// Query provides defines the gRPC querier service
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
        /// Module gets the module info
        pub async fn module(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryModuleRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryModuleResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/initia.move.v1.Query/Module");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("initia.move.v1.Query", "Module"));
            self.inner.unary(req, path, codec).await
        }
        /// Modules gets the module infos
        pub async fn modules(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryModulesRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryModulesResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/initia.move.v1.Query/Modules");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("initia.move.v1.Query", "Modules"));
            self.inner.unary(req, path, codec).await
        }
        /// Resource gets the module info
        pub async fn resource(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryResourceRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryResourceResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/initia.move.v1.Query/Resource");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("initia.move.v1.Query", "Resource"));
            self.inner.unary(req, path, codec).await
        }
        /// Resources gets the module infos
        pub async fn resources(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryResourcesRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryResourcesResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/initia.move.v1.Query/Resources");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("initia.move.v1.Query", "Resources"));
            self.inner.unary(req, path, codec).await
        }
        /// Query table info of the given address
        pub async fn table_info(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryTableInfoRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryTableInfoResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/initia.move.v1.Query/TableInfo");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("initia.move.v1.Query", "TableInfo"));
            self.inner.unary(req, path, codec).await
        }
        /// Query table entry of the given key
        pub async fn table_entry(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryTableEntryRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryTableEntryResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/initia.move.v1.Query/TableEntry");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("initia.move.v1.Query", "TableEntry"));
            self.inner.unary(req, path, codec).await
        }
        /// Query table entries with pagination
        pub async fn table_entries(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryTableEntriesRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryTableEntriesResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/initia.move.v1.Query/TableEntries");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("initia.move.v1.Query", "TableEntries"));
            self.inner.unary(req, path, codec).await
        }
        /// Deprecated: Use Query/ViewJSON or Query/ViewJSONBatch
        /// LegacyView execute view function and return the view result.
        pub async fn legacy_view(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryLegacyViewRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryLegacyViewResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/initia.move.v1.Query/LegacyView");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("initia.move.v1.Query", "LegacyView"));
            self.inner.unary(req, path, codec).await
        }
        /// Deprecated: Use Query/ViewJSON or Query/ViewJSONBatch
        /// View execute view function and return the view result
        pub async fn view(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryViewRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryViewResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/initia.move.v1.Query/View");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("initia.move.v1.Query", "View"));
            self.inner.unary(req, path, codec).await
        }
        /// Deprecated: Use Query/ViewJSON or Query/ViewJSONBatch
        /// ViewBatch execute multiple view functions and return the view results
        pub async fn view_batch(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryViewBatchRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryViewBatchResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/initia.move.v1.Query/ViewBatch");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("initia.move.v1.Query", "ViewBatch"));
            self.inner.unary(req, path, codec).await
        }
        /// ViewJSON execute view function with json arguments and return the view result
        pub async fn view_json(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryViewJsonRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryViewJsonResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/initia.move.v1.Query/ViewJSON");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("initia.move.v1.Query", "ViewJSON"));
            self.inner.unary(req, path, codec).await
        }
        /// ViewJSONBatch execute multiple view functions with json arguments and return the view results
        pub async fn view_json_batch(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryViewJsonBatchRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryViewJsonBatchResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/initia.move.v1.Query/ViewJSONBatch");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("initia.move.v1.Query", "ViewJSONBatch"));
            self.inner.unary(req, path, codec).await
        }
        /// ScriptABI decode script bytes into ABI
        pub async fn script_abi(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryScriptAbiRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryScriptAbiResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/initia.move.v1.Query/ScriptABI");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("initia.move.v1.Query", "ScriptABI"));
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
            let path = http::uri::PathAndQuery::from_static("/initia.move.v1.Query/Params");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("initia.move.v1.Query", "Params"));
            self.inner.unary(req, path, codec).await
        }
        /// Metadata converts metadata to denom
        pub async fn metadata(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryMetadataRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryMetadataResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/initia.move.v1.Query/Metadata");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("initia.move.v1.Query", "Metadata"));
            self.inner.unary(req, path, codec).await
        }
        /// Denom converts denom to metadata
        pub async fn denom(
            &mut self,
            request: impl tonic::IntoRequest<super::QueryDenomRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryDenomResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/initia.move.v1.Query/Denom");
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("initia.move.v1.Query", "Denom"));
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
        /// Module gets the module info
        async fn module(
            &self,
            request: tonic::Request<super::QueryModuleRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryModuleResponse>, tonic::Status>;
        /// Modules gets the module infos
        async fn modules(
            &self,
            request: tonic::Request<super::QueryModulesRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryModulesResponse>, tonic::Status>;
        /// Resource gets the module info
        async fn resource(
            &self,
            request: tonic::Request<super::QueryResourceRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryResourceResponse>, tonic::Status>;
        /// Resources gets the module infos
        async fn resources(
            &self,
            request: tonic::Request<super::QueryResourcesRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryResourcesResponse>, tonic::Status>;
        /// Query table info of the given address
        async fn table_info(
            &self,
            request: tonic::Request<super::QueryTableInfoRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryTableInfoResponse>, tonic::Status>;
        /// Query table entry of the given key
        async fn table_entry(
            &self,
            request: tonic::Request<super::QueryTableEntryRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryTableEntryResponse>, tonic::Status>;
        /// Query table entries with pagination
        async fn table_entries(
            &self,
            request: tonic::Request<super::QueryTableEntriesRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryTableEntriesResponse>, tonic::Status>;
        /// Deprecated: Use Query/ViewJSON or Query/ViewJSONBatch
        /// LegacyView execute view function and return the view result.
        async fn legacy_view(
            &self,
            request: tonic::Request<super::QueryLegacyViewRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryLegacyViewResponse>, tonic::Status>;
        /// Deprecated: Use Query/ViewJSON or Query/ViewJSONBatch
        /// View execute view function and return the view result
        async fn view(
            &self,
            request: tonic::Request<super::QueryViewRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryViewResponse>, tonic::Status>;
        /// Deprecated: Use Query/ViewJSON or Query/ViewJSONBatch
        /// ViewBatch execute multiple view functions and return the view results
        async fn view_batch(
            &self,
            request: tonic::Request<super::QueryViewBatchRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryViewBatchResponse>, tonic::Status>;
        /// ViewJSON execute view function with json arguments and return the view result
        async fn view_json(
            &self,
            request: tonic::Request<super::QueryViewJsonRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryViewJsonResponse>, tonic::Status>;
        /// ViewJSONBatch execute multiple view functions with json arguments and return the view results
        async fn view_json_batch(
            &self,
            request: tonic::Request<super::QueryViewJsonBatchRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryViewJsonBatchResponse>, tonic::Status>;
        /// ScriptABI decode script bytes into ABI
        async fn script_abi(
            &self,
            request: tonic::Request<super::QueryScriptAbiRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryScriptAbiResponse>, tonic::Status>;
        /// Params queries all parameters.
        async fn params(
            &self,
            request: tonic::Request<super::QueryParamsRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryParamsResponse>, tonic::Status>;
        /// Metadata converts metadata to denom
        async fn metadata(
            &self,
            request: tonic::Request<super::QueryMetadataRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryMetadataResponse>, tonic::Status>;
        /// Denom converts denom to metadata
        async fn denom(
            &self,
            request: tonic::Request<super::QueryDenomRequest>,
        ) -> std::result::Result<tonic::Response<super::QueryDenomResponse>, tonic::Status>;
    }
    /// Query provides defines the gRPC querier service
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
                "/initia.move.v1.Query/Module" => {
                    #[allow(non_camel_case_types)]
                    struct ModuleSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryModuleRequest> for ModuleSvc<T> {
                        type Response = super::QueryModuleResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryModuleRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).module(request).await };
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
                        let method = ModuleSvc(inner);
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
                "/initia.move.v1.Query/Modules" => {
                    #[allow(non_camel_case_types)]
                    struct ModulesSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryModulesRequest> for ModulesSvc<T> {
                        type Response = super::QueryModulesResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryModulesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).modules(request).await };
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
                        let method = ModulesSvc(inner);
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
                "/initia.move.v1.Query/Resource" => {
                    #[allow(non_camel_case_types)]
                    struct ResourceSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryResourceRequest> for ResourceSvc<T> {
                        type Response = super::QueryResourceResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryResourceRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).resource(request).await };
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
                        let method = ResourceSvc(inner);
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
                "/initia.move.v1.Query/Resources" => {
                    #[allow(non_camel_case_types)]
                    struct ResourcesSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryResourcesRequest> for ResourcesSvc<T> {
                        type Response = super::QueryResourcesResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryResourcesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).resources(request).await };
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
                        let method = ResourcesSvc(inner);
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
                "/initia.move.v1.Query/TableInfo" => {
                    #[allow(non_camel_case_types)]
                    struct TableInfoSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryTableInfoRequest> for TableInfoSvc<T> {
                        type Response = super::QueryTableInfoResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryTableInfoRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).table_info(request).await };
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
                        let method = TableInfoSvc(inner);
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
                "/initia.move.v1.Query/TableEntry" => {
                    #[allow(non_camel_case_types)]
                    struct TableEntrySvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryTableEntryRequest> for TableEntrySvc<T> {
                        type Response = super::QueryTableEntryResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryTableEntryRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).table_entry(request).await };
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
                        let method = TableEntrySvc(inner);
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
                "/initia.move.v1.Query/TableEntries" => {
                    #[allow(non_camel_case_types)]
                    struct TableEntriesSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryTableEntriesRequest> for TableEntriesSvc<T> {
                        type Response = super::QueryTableEntriesResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryTableEntriesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).table_entries(request).await };
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
                        let method = TableEntriesSvc(inner);
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
                "/initia.move.v1.Query/LegacyView" => {
                    #[allow(non_camel_case_types)]
                    struct LegacyViewSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryLegacyViewRequest> for LegacyViewSvc<T> {
                        type Response = super::QueryLegacyViewResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryLegacyViewRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).legacy_view(request).await };
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
                        let method = LegacyViewSvc(inner);
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
                "/initia.move.v1.Query/View" => {
                    #[allow(non_camel_case_types)]
                    struct ViewSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryViewRequest> for ViewSvc<T> {
                        type Response = super::QueryViewResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryViewRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).view(request).await };
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
                        let method = ViewSvc(inner);
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
                "/initia.move.v1.Query/ViewBatch" => {
                    #[allow(non_camel_case_types)]
                    struct ViewBatchSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryViewBatchRequest> for ViewBatchSvc<T> {
                        type Response = super::QueryViewBatchResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryViewBatchRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).view_batch(request).await };
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
                        let method = ViewBatchSvc(inner);
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
                "/initia.move.v1.Query/ViewJSON" => {
                    #[allow(non_camel_case_types)]
                    struct ViewJSONSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryViewJsonRequest> for ViewJSONSvc<T> {
                        type Response = super::QueryViewJsonResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryViewJsonRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).view_json(request).await };
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
                        let method = ViewJSONSvc(inner);
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
                "/initia.move.v1.Query/ViewJSONBatch" => {
                    #[allow(non_camel_case_types)]
                    struct ViewJSONBatchSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryViewJsonBatchRequest>
                        for ViewJSONBatchSvc<T>
                    {
                        type Response = super::QueryViewJsonBatchResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryViewJsonBatchRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).view_json_batch(request).await };
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
                        let method = ViewJSONBatchSvc(inner);
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
                "/initia.move.v1.Query/ScriptABI" => {
                    #[allow(non_camel_case_types)]
                    struct ScriptABISvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryScriptAbiRequest> for ScriptABISvc<T> {
                        type Response = super::QueryScriptAbiResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryScriptAbiRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).script_abi(request).await };
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
                        let method = ScriptABISvc(inner);
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
                "/initia.move.v1.Query/Params" => {
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
                "/initia.move.v1.Query/Metadata" => {
                    #[allow(non_camel_case_types)]
                    struct MetadataSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryMetadataRequest> for MetadataSvc<T> {
                        type Response = super::QueryMetadataResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryMetadataRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).metadata(request).await };
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
                        let method = MetadataSvc(inner);
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
                "/initia.move.v1.Query/Denom" => {
                    #[allow(non_camel_case_types)]
                    struct DenomSvc<T: Query>(pub Arc<T>);
                    impl<T: Query> tonic::server::UnaryService<super::QueryDenomRequest> for DenomSvc<T> {
                        type Response = super::QueryDenomResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryDenomRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).denom(request).await };
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
                        let method = DenomSvc(inner);
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
        const NAME: &'static str = "initia.move.v1.Query";
    }
}
/// ObjectAccount defines an account for objects that holds coins without pubkey.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObjectAccount {
    #[prost(message, optional, tag = "1")]
    pub base_account:
        ::core::option::Option<super::super::super::cosmos::auth::v1beta1::BaseAccount>,
}
/// TableAccount defines an account for tables that holds items without pubkey.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TableAccount {
    #[prost(message, optional, tag = "1")]
    pub base_account:
        ::core::option::Option<super::super::super::cosmos::auth::v1beta1::BaseAccount>,
}
