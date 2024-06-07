#!/usr/bin/env bash

set -e

SCRIPT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)
CONTRACTS_DIR="$SCRIPT_DIR/../../artifacts"
STAKING_CONTRACT="$CONTRACTS_DIR/staking.wasm"
TREASURY_CONTRACT="$CONTRACTS_DIR/treasury.wasm"

# Include the bins to the path
PATH="$SCRIPT_DIR/../bins:$PATH"

# Global config
OSMOSIS_CHAIN_ID="osmosis-dev-1"
OSMOSIS_GAS_PRICE="0.025stake"
OSMOSIS_GAS_ADJUSTMENT="1.7"
OSMOSIS_NODE="http://localhost:26657"
OSMOSIS_TX_PARAMS="--node $OSMOSIS_NODE --chain-id $OSMOSIS_CHAIN_ID --gas-prices $OSMOSIS_GAS_PRICE --gas-adjustment $OSMOSIS_GAS_ADJUSTMENT --gas auto"

function store_contract() {
  local sender=$1
  local contract_path=$2
  osmosisd tx wasm store "$contract_path" \
    --from "$sender" $OSMOSIS_TX_PARAMS -y |
    jq -r '.txhash'
}
