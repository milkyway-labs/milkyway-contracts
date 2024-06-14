#!/usr/bin/env bash

SCRIPT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)
# Include the binaries
PATH=$SCRIPT_DIR/../bins:$PATH
# Include the params
source "$SCRIPT_DIR/../params.sh"

echo "Configuring osmosisd"
osmosisd config chain-id $OSMOSIS_CHAIN_ID
osmosisd config gas auto
osmosisd config gas-adjustment $OSMOSIS_GAS_ADJUSTMENT
osmosisd config gas-prices $OSMOSIS_GAS_PRICE
osmosisd config 'keyring-backend' test

echo "Configuring celestia-appd"
celestia-appd config chain-id $CELESTIA_CHAIN_ID
celestia-appd config 'keyring-backend' test
celestia-appd config node $CELESTIA_NODE

echo "Clients configured"
