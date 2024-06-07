#!/bin/bash

SCRIPT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)
BINS_DIR=$SCRIPT_DIR/../bins
PATH=$BINS_DIR:$PATH

CODE_ID=$(osmosisd query wasm list-code --output json | jq -r '.code_infos[-1].code_id')
CONTRACT=$(osmosisd query wasm list-contract-by-code $CODE_ID --output json | jq -r '.contracts[-1]')
RES=$(osmosisd tx wasm store ./artifacts/staking-aarch64.wasm --from test_master --keyring-backend test --output json -y -b block --gas-prices 0.025stake --gas-adjustment 1.7 --gas auto --chain-id osmosis-dev-1)
CODE_ID=$(echo $RES | jq -r '.logs[0].events[-1].attributes[-1].value')
osmosisd tx wasm migrate $CONTRACT $CODE_ID '{}' \
  --from test_master -y \
  -b block --keyring-backend test \
  --gas-prices 0.025stake --gas-adjustment 1.7 --gas auto \
  --chain-id osmosis-dev-1
