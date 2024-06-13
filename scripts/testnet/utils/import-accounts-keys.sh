#!/usr/bin/env bash

SCRIPT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)
PATH=$SCRIPT_DIR/../bins:$PATH
source "$SCRIPT_DIR/../params.sh"

echo "Adding accounts to test kyering"
echo $TESTNET_MNEMONIC
echo $TESTNET_MNEMONIC | osmosisd keys add test_master --recover --keyring-backend test
echo $TESTNET_MNEMONIC | osmosisd keys add trader --account $OSMOSIS_TRADER_ACCOUNT_INDEX --recover --keyring-backend test
echo $TESTNET_MNEMONIC | celestia-appd keys add staker --account $CELESTIA_STAKER_ACCOUNT_INDEX --recover --keyring-backend test
echo $TESTNET_MNEMONIC | celestia-appd keys add grantee --account $CELESTIA_GRANTEE_ACCOUNT_INDEX --recover --keyring-backend test
echo $TESTNET_MNEMONIC | celestia-appd keys add rewards_collector --account $CELESTIA_REWARDS_COLLECTOR_ACCOUNT_INDEX --recover --keyring-backend test
