# Staking Contract

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
}
```

## ExecuteMsg

```rust
pub enum ExecuteMsg {
	/// Call to initiate bonding process for a user
    LiquidStake {}, 
	
	/// Call to initiate unbonding process for a user
	LiquidUnstake {},

	SubmitBatch {
		batch_id u64,
	},

	/// Call to claim unbonded amount and accrued staking rewards
	Claim {}, 
	
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
	}

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

```rust
pub struct ConfigResponse {
	/// Denomination of underlying token (IBC denom of TIA)
    pub native_token_denom: String, 
	/// Denomination of the liquid staking token (stTIA)
    pub liquid_stake_token_denom: String, 
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
	/// Minimum amount to liquid stake
    pub minimum_liquid_stake_amount: Uint128, 
}
```

2. Get State

```rust
pub struct StateResponse {
    pub total_native_token: Uint128,
    pub total_liquid_stake_token: Uint128,
    pub rate: Decimal,
    pub pending_owner: String,
    pub total_reward_amount: Uint128,
}
```

3. Get Batch

```rust
pub struct BatchResponse {
    pub batch_total_liquid_stake: Uint128,
    pub expected_native_unstaked: Uint128,
    pub next_batch_action_time: Timestamp,
    pub status: String,
    pub requests: Vec<LiquidUnstakeRequestResponse>,
}
```

4. Get Batches
```rust
pub struct BatchesResponse {
    pub batches: Vec<BatchResponse>,
}
```
