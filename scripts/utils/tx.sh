#!/bin/bash

SCRIPT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)
source "$SCRIPT_DIR/../testnet/params.sh"

function wait_tx_included() {
  # Get function parameters
  local binary=$1
  local tx_hash=$2
  local node=$3

  local count=0
  while [ $count -lt 30  ]; do
    local output=$($binary q tx "$tx_hash" --node $node --output json 2>/dev/null || echo "failed")
    if [ "$output" == "failed" ]; then
      count=$((count+1))
      sleep 1
    else
      local code=$(echo $output | jq -r '.code')
      if [ "$code" == "0" ]; then
        break
      else
        echo "Tx $tx_hash failed"
        echo $output | jq '.raw_log'
        echo ""
        echo $output
        exit 1
      fi
    fi
  done
}

function wait_osmossis_tx() {
  local tx_hash=$1
  wait_tx_included osmosisd $tx_hash $OSMOSIS_NODE
}

function wait_celestia_tx() {
  local tx_hash=$1
  wait_tx_included celestia-appd $tx_hash $CELESTIA_NODE
}

function wait_tx() {
  local command=$@
  # Extract the binary name
  local bin="$1"
  # Get the transaction hash
  # of the command that has been provided
  local tx_hash=$($command --output json | jq -r '.txhash')

  if [ "$bin" == "osmosisd" ]; then
    wait_osmossis_tx $tx_hash
  elif [ "$bin" == "celestia-appd" ]; then
    wait_celestia_tx $tx_hash
  else
    echo "Can wait tx of $bin"
    exit 1
  fi
}
