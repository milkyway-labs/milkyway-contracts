#!/usr/bin/env bash

# Osmosis config
OSMOSIS_NODE="http://localhost:26657"
OSMOSIS_CHAIN_ID="osmosis-dev-1"
OSMOSIS_GAS_PRICE="0.025stake"
OSMOSIS_GAS_ADJUSTMENT="1.7"
OSMOSIS_TX_PARAMS="--keyring-backend test --node $OSMOSIS_NODE --chain-id $OSMOSIS_CHAIN_ID --gas-prices $OSMOSIS_GAS_PRICE --gas-adjustment $OSMOSIS_GAS_ADJUSTMENT --gas auto -y"

# Celestia config
CELESTIA_NODE="http://localhost:26661"
CELESTIA_CHAIN_ID="celestia-dev-1"
CELESTIA_TX_PARAMS="--keyring-backend test --node $CELESTIA_NODE --chain-id $CELESTIA_CHAIN_ID --fees 21000utia -y"

# Accounts info
# Mnemonic used to generate accounts
TESTNET_MNEMONIC="boy view flame close solar robust crunch slot govern false jungle dirt blade minor shield bounce rent expand anxiety busy pull inject grace require"

# Osmosis accounts
OSMOSIS_ACCOUNT="osmo1sfhy3emrgp26wnzuu64p06kpkxd9phel8ym0ge"
OSMOSIS_TRADER="osmo1lh0u9sug6qh922gjpal3frwtacaums4s7lkyl9"
OSMOSIS_TRADER_ACCOUNT_INDEX=1

# Celestia accounts
CELESTIA_STAKER="celestia1sfhy3emrgp26wnzuu64p06kpkxd9phel74e0yx"
CELESTIA_STAKER_ACCOUNT_INDEX=0
CELESTIA_GRANTEE="celestia1lh0u9sug6qh922gjpal3frwtacaums4s8w5yn6"
CELESTIA_GRANTEE_ACCOUNT_INDEX=1
CELESTIA_REWARDS_COLLECTOR="celestia12rzczckgh8fqq533t0xqhqrrzdk76du3dxrx9q"
CELESTIA_REWARDS_COLLECTOR_ACCOUNT_INDEX=2
