# MilkyWay Contracts

![image](./assets/banner.png)

## Overview

MilkyWay is a liquid staking protocol for the [Celestia](https://celestia.org/) ecosystem.

This repository contains MilkyWay's core contracts that is deployed and operated on [Osmosis](https://osmosis.zone/).

## Contracts

| Contract                           | Description                                             |
| ---------------------------------- | ------------------------------------------------------- |
| [`staking`](./contracts/staking)   | Core contract for liquid staking / liquid unstaking TIA |
| [`treasury`](./contracts/treasury) | MilkyWay DAO                                            |

## Testing

All tests can be found in the tests folder in each respective contract package.

Run all tests in the repo:

```rust
make test
```

## Testnets

There is Docker testnets available: Check `./scripts/testnet`

Deploy the contract using the available scripts:

```
make initlocal
```

```
make migratelocal
```

## License

This software is licensed under the Apache 2.0 license.
