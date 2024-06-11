#!/usr/bin/env bash

SCRIPT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)
PATH=$SCRIPT_DIR/../bins:$PATH
source "$SCRIPT_DIR/../testnet/params.sh"

echo "Adding accounts to test kyering"
echo $TESTNET_MNEMONIC
echo "$TESTNET_MNEMONIC" | osmosisd keys add test_master --recover --keyring-backend test
echo "$TESTNET_MNEMONIC" | celestia-appd keys add test_master --recover --keyring-backend test
