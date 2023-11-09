# MilkyWay Contracts

MilkyWay is a liquid staking protocol tailored for the [Celestia](https://celestia.org/) ecosystem.
This repository contains MilkyWay's core contracts that is deployed and operated on [Osmosis](https://osmosis.zone/).

## Contracts

| Contract                           | Description                                             |
| ---------------------------------- | ------------------------------------------------------- |
| [`staking`](./contracts/staking)   | Core contract for liquid staking / liquid unstaking TIA |
| [`treasury`](./contracts/treasury) | MilkyWay DAO                                            |

## Deployment

### Mainnet (`osmosis-1`)

...

### Testnet (`osmo-test-5`)

...

## Testing

All tests can be found in the tests folder in each respective contract package.

Run all tests in the repo:

```rust
make test
```

