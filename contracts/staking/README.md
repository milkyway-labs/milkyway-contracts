# Staking Contract

The staking contract contains the core functionalities for liquid staking derivative.

## InstantiateMsg

```rust
pub struct InstantiateMsg {
    /// Denomination of underlying token (IBC denom of TIA)
    pub native_token_denom: Denom,

    /// Denomination of the liquid staking token (stTIA)
    pub liquid_stake_token_denom: Denom,

    /// Treasury contract address
    pub treasury_address: String,

    /// Set of operators who will operate the protocol
    pub operators: Vec<String>,

    /// Set of validators who will receive the delegations
    pub validators: Vec<String>,

    /// How often the unbonding queue is to be executed in seconds
    pub batch_period: u64,

    /// The staking module's unbonding period for Celestia in seconds
    pub unbonding_period: u64,

    /// Protocol fee configuration
    pub protocol_fee_config: ProtocolFeeConfig,

    /// Multisig address configuration
    pub multisig_address_config: MultisigAddressConfig,

    /// Minimum amount to liquid stake
    pub minimum_liquid_stake_amount: Uint128,

    /// Minimum amount to liquid unstake
    pub minimum_liquid_unstake_amount: Uint128,
}
```

## ExecuteMsg

```rust
pub enum ExecuteMsg {
    /// Call to initiate bonding process for a user
    LiquidStake {},

    /// Call to initiate unbonding process for a user
    LiquidUnstake {},

    /// Process the pending batch
    SubmitBatch {
      batch_id u64,
    },

    /// Add a validator from the validator set; callable by the owner
    AddValidator {
      new_validator: String,
    },

    /// Remove a validator from the validator set; callable by the owner
    RemoveValidator {
      validator: String,
    },

    /// Transfer ownership to another account; callable by the owner
    /// This will require the new owner to accept to take effect.
    TransferOwnership {
      new_owner: String,
    },

    /// Accept an ownership transfer; callable by the new owner
    AcceptOwnership {},

    /// Revoke an ownership transfer; callable by the owner
    RevokeOwnershipTransfer {},
}
```

## QueryMsg

```rust
pub enum QueryMsg {
    #[returns(ConfigResponse)]
    Config {},
    #[returns(StateResponse)]
    State {},
    #[returns(BatchResponse)]
    Batch { id: u64 },
    #[returns(BatchesResponse)]
    Batches {},
}
```

## Query Responses

1. Get Config

```json
{
    "native_token_denom": "NATIVE_TOKEN_DENOM",
    "liquid_stake_token_denom": "LIQUID_STAKE_TOKEN_DENOM",
    "treasury_address": "TREASURY_ADDR",
    "operators": ["OPERATOR_ADDR_1", "OPERATOR_ADDR_2"],
    "validators": ["VALIDATOR_ADDR_1", "VALIDATOR_ADDR_2"],
    "batch_period": 86400,
    "unbonding_period": 1209600,
    "minimum_liquid_stake_amount": 100,
    "minimum_liquid_unstake_amount": 100
}
```

2. Get State

```json
{
    "total_native_token": 1000000000,
    "total_liquid_stake_token": 1000000000,
    "rate": 1,
    "pending_owner": "",
    "total_reward_amount": 100000
}
```

3. Get Batch

| Param  | Type   |                     |
|--------|--------|---------------------|
| id     | number | The batch id to get |

```json
{
    "batch_total_liquid_stake": "500",
    "expected_native_unstaked": "0",
    "next_batch_action_time": "1573093420000000000",
    "status": "submitted",
    "requests": [
        {
        "user": "bob",
        "amount": "500"
        }
    ]
}

```

4. Get Batches
```json
{
  "batches": [
    {
      "batch_total_liquid_stake": "500",
      "expected_native_unstaked": "0",
      "next_batch_action_time": "1573093420000000000",
      "status": "submitted",
      "requests": [
        {
          "user": "bob",
          "amount": "500"
        }
      ]
    },
    {
      "batch_total_liquid_stake": "1500",
      "expected_native_unstaked": "0",
      "next_batch_action_time": "1571970220000000000",
      "status": "pending",
      "requests": [
        {
          "user": "alice",
          "amount": "1500"
        }
      ]
    }
  ]
}
```
