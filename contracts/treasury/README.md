# Treasury Contract

The treasury contract includes functionalities to properly manage the fees collected from the liquid staking protocol.
These functionalities are:

- Allowing a `trader` account to swap the collected fees into other assets to reduce the volatility of the treasury balance.
- Enabling the withdrawal of the treasury balance to another account.

## Deploy

To deploy the contract you have to build the optimized wasm file with the following command
from the repository root:

```shell
make optimize
```

Then you can deploy the contract using the following command:

```shell
osmosisd tx wasm store ./artifacts/treasury.wasm \
    --from <your-address> --chain-id "osmosis-1" \
    --gas=auto --gas-prices=0.04uosmo --gas-adjustment=1.5 \
    --node https://rpc.osmosis.zone:443
```

## Instantiate

To instantiate the contract you have to use the following command:

```shell
osmosisd tx wasm instantiate <code-id> '{"admin": "<admin-addr>", "trader": "<trader-addr>", "allowed_swap_routes": [<swap-route>]}' \
    --from <your-address> --label "treasury" \
    --chain-id "osmosis-1" \
    --gas=auto --gas-prices=0.04uosmo --gas-adjustment=1.5 \
    --node https://rpc.osmosis.zone:443
```

The instantiate message takes the following parameters:
- `admin` - Optional address of the admin account, if not provided the admin account will be the sender
- `trader` - The address of the trader account, if not provided the trader will be the sender
- `allowed_swap_routes` - The list of swap routes that the trader is allowed to use, you can see the definition [here](./src/state.rs#L20)

## Execute

Here are listed the actions that can be performed by the contract

### Spend funds

This allows the treasury to withdraw their funds to another account.

```shell
osmosisd tx wasm execute <contract-address> '{"spend_funds": {"amount": <coin>, "receiver": "<receiver-addr>", "chain-id": <optional-ibc-channel>}}' \
    --from <your-address> --chain-id "osmosis-1" \
    --gas=auto --gas-prices=0.04uosmo --gas-adjustment=1.5 \
    --node https://rpc.osmosis.zone:443
```

The message takes the following parameters:
- `amount` - The funds to withdraw
- `receiver` - Address of the account that will receive the funds
- `chain-id` - An optional IBC channel that should be used in case the receiver is an account of another chain

### Swap funds

This allows the `trader` account to swap the assets in the treasury balance to other assets.

This can be performed leveraging those Osmosis messages:
- `MsgSwapExactAmountIn`
- `MsgSwapExactAmountOut`

#### MsgSwapExactAmountIn

To perform a swap using the `MsgSwapExactAmountIn` you have to use the following command:

```shell
osmosisd tx wasm execute <contract-address> '{"swap_exact_amount_in": {"routes": [<swap-route>], "token_in": <coin>, "token_out_min_amount": "<amount>"}}' \
    --from <your-address> --chain-id "osmosis-1" \
    --gas=auto --gas-prices=0.04uosmo --gas-adjustment=1.5 \
    --node https://rpc.osmosis.zone:443
```

The message takes the following parameters:
- `routes` - The swap route that will be used to swap the `token_in`
- `token_in` - The asset to swap
- `token_out_min_amount` - The minimum amount of the token to receive after the swap

#### MsgSwapExactAmountOut

To perform a swap using the `MsgSwapExactAmountOut` you have to use the following command:

```shell
osmosisd tx wasm execute <contract-address> '{"swap_exact_amount_out": {"routes": [<swap-route>], "token_out": <coin>, "token_in_max_amount": "<amount>"}}' \
    --from <your-address> --chain-id "osmosis-1" \
    --gas=auto --gas-prices=0.04uosmo --gas-adjustment=1.5 \
    --node https://rpc.osmosis.zone:443
```

The message takes the following parameters:
- `routes` - The swap route to be used to receive the `token_out`.
- `token_out` - The asset we want to receive.
- `token_in_max_amount` - The maximum amount we are willing to spend to receive the `token_out`.

### Update config

To update the config of the contract you have to use the following command:

```shell
osmosisd tx wasm execute <contract-address> '{"update_config": {"trader": "<trader-addr>", "allowed_swap_routes": [<swap-route>]}}' \
    --from <your-address> --chain-id "osmosis-1" \
    --gas=auto --gas-prices=0.04uosmo --gas-adjustment=1.5 \
    --node https://rpc.osmosis.zone:443
```

The message takes the following parameters:
- `trader` - Optional address of the trader account, if not provided the trader account will not be updated.
- `allowed_swap_routes` - Optional list of swap routes that the trader is allowed to use, if not provided the allowed swap routes will not be updated.

**Note**: This message can only be executed by the `admin`

### Transfer the contract admin role

To update the contract admin you have to use the following command:

```shell
osmosisd tx wasm execute <contract-address> '{"transfer_ownership": {"new_owner": "<new-owner-addr>"}}' \
    --from <your-address> --chain-id "osmosis-1" \
    --gas=auto --gas-prices=0.04uosmo --gas-adjustment=1.5 \
    --node https://rpc.osmosis.zone:443
```

The message takes the following parameters:
- `new_owner` - The new owner address

**Note**: This message can only be executed by the `admin`

### Accept the contract admin role

To accept the contract admin role you have to use the following command:

```shell
osmosisd tx wasm execute <contract-address> '{"accept_ownership": {}}' \
    --from <your-address> --chain-id "osmosis-1" \
    --gas=auto --gas-prices=0.04uosmo --gas-adjustment=1.5 \
    --node https://rpc.osmosis.zone:443
```

### Cancel admin transfer

To cancel the admin transfer you have to use the following command:

```shell
osmosisd tx wasm execute <contract-address> '{"revoke_ownership_transfer": {}}' \
    --from <your-address> --chain-id "osmosis-1" \
    --gas=auto --gas-prices=0.04uosmo --gas-adjustment=1.5 \
    --node https://rpc.osmosis.zone:443
```

**Note**: This message can only be executed by the `admin`

## Query

Here are listed the data that can be queried from the contract

### Config

To query the config of the contract you have to use the following command:

```shell
osmosisd query wasm contract-state smart <contract-address> '{"config": {}}' \
    --node https://rpc.osmosis.zone:443
```
