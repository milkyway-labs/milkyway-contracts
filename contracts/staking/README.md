# Staking Contract

The staking contract contains the core functionalities for managing liquid staking derivatives. It allows users and protocol components to interact with the liquid staking system via a defined set of messages.

Below is a summary of the available messages and how they can be used to interact with the contract.

## InstantiateMsg

```rust
pub struct InstantiateMsg {
    /// Information about the chain from which the token
    /// (for which we are creating the LST token) originates.
    pub native_chain_config: UnsafeNativeChainConfig,

    /// Information about the chain where the contract is deployed.
    pub protocol_chain_config: UnsafeProtocolChainConfig,

    /// Protocol fee configuration.
    pub protocol_fee_config: UnsafeProtocolFeeConfig,

    /// Denomination of the liquid staking token (e.g., stTIA).
    pub liquid_stake_token_denom: String,

    /// Frequency (in seconds) at which the unbonding queue is executed.
    pub batch_period: u64,

    /// Set of addresses allowed to trigger a circuit break.
    pub monitors: Vec<String>,
}
```

## ExecuteMsg

```rust
pub enum ExecuteMsg {
    /// Initiates the bonding process for a user.
    LiquidStake {
        /// Address to receive the minted LST tokens.
        /// Can belong to either the chain where this contract is deployed
        /// or the chain from which the native token originates.
        /// If `None`, the tokens are sent to the message sender.
        mint_to: Option<String>,

        /// When both native and protocol chains share the same address prefix,
        /// this flag determines whether to send tokens to the native or protocol chain.
        transfer_to_native_chain: Option<bool>,

        /// Minimum expected amount of LST tokens to be received
        /// for the operation to be considered valid.
        expected_mint_amount: Option<Uint128>,
    },

    /// Initiates the unbonding process for a user.
    LiquidUnstake {},

    /// Withdraws unstaked tokens.
    Withdraw {
        /// ID of the batch from which to withdraw.
        batch_id: u64,
    },

    /// Processes the pending batch.
    SubmitBatch {
        /// ID of the batch to process.
        batch_id: u64,
    },

    /// Adds a validator to the validator set; callable by the owner.
    AddValidator {
        /// Address of the validator to add.
        new_validator: String,
    },

    /// Removes a validator from the validator set; callable by the owner.
    RemoveValidator {
        /// Address of the validator to remove.
        validator: String,
    },

    /// Transfers ownership to another account; callable by the owner.
    /// The new owner must accept the transfer for it to take effect.
    TransferOwnership {
        /// Address of the new owner on the protocol chain.
        new_owner: String,
    },

    /// Accepts ownership transfer; callable by the new owner.
    AcceptOwnership {},

    /// Revokes ownership transfer; callable by the current owner.
    RevokeOwnershipTransfer {},

    /// Updates contract configuration; callable by the owner.
    UpdateConfig {
        /// Updated native chain configuration.
        native_chain_config: Option<UnsafeNativeChainConfig>,

        /// Updated protocol chain configuration.
        protocol_chain_config: Option<UnsafeProtocolChainConfig>,

        /// Updated protocol fee configuration.
        protocol_fee_config: Option<UnsafeProtocolFeeConfig>,

        /// Updated list of circuit breaker monitors.
        monitors: Option<Vec<String>>,

        /// Updated unbonding batch execution frequency (in seconds).
        batch_period: Option<u64>,
    },

    /// Receives rewards from the native chain.
    ReceiveRewards {},

    /// Receives unstaked tokens from the native chain.
    ReceiveUnstakedTokens {
        /// ID of the batch that originated the unstake request.
        batch_id: u64,
    },

    /// Stops the contract due to irregularities; callable by monitors and admin.
    CircuitBreaker {},

    /// Resumes the contract; callable by the admin.
    ResumeContract {
        /// Updated total native tokens delegated (used post-slashing).
        total_native_token: Uint128,

        /// Updated total issued liquid staked tokens.
        total_liquid_stake_token: Uint128,

        /// Updated total protocol rewards.
        total_reward_amount: Uint128,
    },

    /// Recovers IBC transfers that timed out or failed.
    RecoverPendingIbcTransfers {
        /// If true and neither `selected_packets` nor `receiver` are specified,
        /// recovers only the 10 oldest failed IBC transfers.
        paginated: Option<bool>,

        /// Specific packet IDs to recover.
        /// Overrides other parameters if provided.
        selected_packets: Option<Vec<u64>>,

        /// Recovers packets addressed to this account.
        /// Considered only if `selected_packets` is not provided.
        receiver: Option<String>,
    },

    /// Sends the protocol fee to the treasury.
    FeeWithdraw {
        /// Amount to send to the treasury.
        amount: Uint128,
    },
}
```

## QueryMsg

```rust
pub enum QueryMsg {
    /// Queries the contract configuration.
    #[returns(ConfigResponse)]
    Config {},

    /// Queries the current state of the contract.
    #[returns(StateResponse)]
    State {},

    /// Queries the information of a specific batch by its ID.
    #[returns(BatchResponse)]
    Batch {
        /// ID of the batch to query.
        id: u64,
    },

    /// Queries a paginated list of all batches stored in contract storage.
    #[returns(BatchesResponse)]
    Batches {
        /// If provided, starts listing batches after this batch ID.
        start_after: Option<u64>,

        /// Maximum number of batches to return.
        limit: Option<u32>,

        /// Optional filter to return only batches with the given status.
        status: Option<BatchStatus>,
    },

    /// Queries the batches with the provided list of IDs.
    #[returns(BatchesResponse)]
    BatchesByIds {
        /// List of batch IDs to fetch.
        ids: Vec<u64>,
    },

    /// Queries the current batch that is pending processing.
    #[returns(BatchResponse)]
    PendingBatch {},

    /// Queries the unstake requests made by a specific user.
    #[returns(Vec<UnstakeRequest>)]
    UnstakeRequests {
        /// Address of the user whose unstake requests are to be queried.
        user: Addr,
    },

    /// Queries all unstake requests in the contract.
    #[returns(Vec<UnstakeRequestResponse>)]
    AllUnstakeRequests {
        /// If provided, starts listing unstake requests after this ID.
        start_after: Option<u64>,

        /// Maximum number of unstake requests to return.
        limit: Option<u32>,
    },

    /// Queries all unstake requests with simplified structure.
    /// Returns tuples of `(user_address, batch_id, amount)`.
    #[returns(Vec<(String, u64, Uint128)>)]
    AllUnstakeRequestsV2 {
        /// If provided, starts listing unstake requests after this ID.
        start_after: Option<u64>,

        /// Maximum number of unstake requests to return.
        limit: Option<u32>,
    },

    /// Queries the IBC packets that were received as replies from the native chain.
    #[returns(IBCQueueResponse)]
    IbcQueue {
        /// If provided, starts listing IBC replies after this packet ID.
        start_after: Option<u64>,

        /// Maximum number of IBC reply entries to return.
        limit: Option<u32>,
    },

    /// Queries IBC packets that have been sent and are still waiting for a reply.
    #[returns(IBCReplyQueueResponse)]
    IbcReplyQueue {
        /// If provided, starts listing sent-but-unreplied packets after this ID.
        start_after: Option<u64>,

        /// Maximum number of IBC reply queue entries to return.
        limit: Option<u32>,
    },
}
```

## Query Responses

### Config

```json
{
  "native_chain_config": {
    "account_address_prefix": "atom",
    "validator_address_prefix": "cosmosvaloper",
    "token_denom": "uatom",
    "validators": [],
    "unbonding_period": 1209600,
    "staker_address": "cosmos1xyzxyzxyzxyzxyzxyzxyzxyzxyzxyzxyzxyz",
    "reward_collector_address": "cosmos1abcabcabcabcabcabcabcabcabcabcabcabc"
  },
  "protocol_chain_config": {
    "account_address_prefix": "stake",
    "ibc_channel_id": "channel-99",
    "ibc_token_denom": "ibc/1234567890ABCDEF1234567890ABCDEF1234567890ABCDEF1234567890ABCDEF",
    "minimum_liquid_stake_amount": "50000",
    "oracle_address": "cosmos1oracleoracleoracleoracleoracleoracleoracleoraclemq8"
  },
  "protocol_fee_config": {
    "dao_treasury_fee": "5000",
    "treasury_address": "cosmos1treasurytreasurytreasurytreasurytreasurytreasurytrea"
  },
  "monitors": [
    "cosmos1monitoroneoneoneoneoneoneoneoneoneonexxx",
    "cosmos1monitortwotwotwotwotwotwotwotwotwotwozzz"
  ],
  "liquid_stake_token_denom": "factory/cosmos1exampleaddresshere/ulstATOM",
  "batch_period": 86400,
  "stopped": false
}
```

### State

```json
{
  "total_native_token": "100000000",
  "total_liquid_stake_token": "98000000",
  "rate": "0.98",
  "pending_owner": "cosmos1pendingownerxxxxxxxxxxxxxxxxxxxxxxxxxxx",
  "total_reward_amount": "2000000",
  "total_fees": "500000"
}
```

### Batch

```json
{
  "id": 42,
  "batch_total_liquid_stake": "50000000",
  "expected_native_unstaked": "48000000",
  "received_native_unstaked": "47500000",
  "unstake_request_count": 12,
  "next_batch_action_time": "2025-04-16T18:30:00Z",
  "status": "pending"
}
```

### Batches

```json
{
  "batches": [
    {
      "id": 1,
      "batch_total_liquid_stake": "50000000",
      "expected_native_unstaked": "48000000",
      "received_native_unstaked": "47500000",
      "unstake_request_count": 12,
      "next_batch_action_time": "2025-04-16T18:30:00Z",
      "status": "pending"
    },
    {
      "id": 2,
      "batch_total_liquid_stake": "50000000",
      "expected_native_unstaked": "48000000",
      "received_native_unstaked": "47500000",
      "unstake_request_count": 12,
      "next_batch_action_time": "2025-04-16T18:30:00Z",
      "status": "pending"
    }
  ]
}
```

### BatchesByIds

```json
{
  "batches": [
    {
      "id": 1,
      "batch_total_liquid_stake": "50000000",
      "expected_native_unstaked": "48000000",
      "received_native_unstaked": "47500000",
      "unstake_request_count": 12,
      "next_batch_action_time": "2025-04-16T18:30:00Z",
      "status": "pending"
    },
    {
      "id": 2,
      "batch_total_liquid_stake": "50000000",
      "expected_native_unstaked": "48000000",
      "received_native_unstaked": "47500000",
      "unstake_request_count": 12,
      "next_batch_action_time": "2025-04-16T18:30:00Z",
      "status": "pending"
    }
  ]
}
```

### PendingBatch

```json
{
  "id": 42,
  "batch_total_liquid_stake": "50000000",
  "expected_native_unstaked": "48000000",
  "received_native_unstaked": "47500000",
  "unstake_request_count": 12,
  "next_batch_action_time": "2025-04-16T18:30:00Z",
  "status": "pending"
}
```

### UnstakeRequests

```json
[
  {
    "batch_id": 42,
    "user": "cosmos1xyzxyzxyzxyzxyzxyzxyzxyzxyzxyzxyzxyz",
    "amount": "10000000"
  },
  {
    "batch_id": 43,
    "user": "cosmos1xyzxyzxyzxyzxyzxyzxyzxyzxyzxyzxyzxyz",
    "amount": "30000000"
  }
]

```

### AllUnstakeRequests

```json
[
  {
    "batch_id": 42,
    "user": "cosmos1xyzxyzxyzxyzxyzxyzxyzxyzxyzxyzxyzxyz",
    "amount": "10000000"
  },
  {
    "batch_id": 43,
    "user": "cosmos1xyzxyzxyzxyzxyzxyzxyzxyzxyzxyzxyzxyz",
    "amount": "30000000"
  }
]

```

### AllUnstakeRequestsV2

```json
[
  [
    "cosmos10032e6a3t784u7lzv7nm5z3zdpfescryjcjul8",
    121,
    "4993528"
  ],
  [
    "cosmos1007hp8222cum8hdmzud6f6495el2pqtl0ef0dj",
    110,
    "10089943"
  ]
]

```

### IbcQueue

```json
{
  "ibc_queue": [
    {
      "sequence": 101,
      "amount": {
        "denom": "stake",
        "amount": "15000000"
      },
      "receiver": "cosmos1m4c3zp5t2d5yn88wxj3q8svnp9azyd8q3mlw2c",
      "status": "Sent"
    }
  ]
}
```

### IbcReplyQueue

```json
{
  "ibc_queue": [
    {
      "amount": {
        "denom": "stake",
        "amount": "150000"
      },
      "receiver": "cosmos1w6rtscwnylx4kz3cv92dtwhm9d36k77ug4gchc"
    },
    {
      "amount": {
        "denom": "stake",
        "amount": "300000"
      },
      "receiver": "cosmos1j9ns0wkcj2nsym06s9eq4y9kqpx57zz72uc46e"
    }
  ]
}
```
