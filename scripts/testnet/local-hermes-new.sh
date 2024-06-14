#!/bin/bash

SCRIPT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)
BINS_DIR=$SCRIPT_DIR/bins
PATH=$BINS_DIR:$PATH

# Include the utils function
source "$SCRIPT_DIR"/utils/tx.sh

# Stop script execution if an error is encountered
set -o errexit
# Stop script execution if an undefined variable is used
set -o nounset

killall hermes || true

mkdir -p ~/.hermes
cp ./hermes-config.toml ~/.hermes/config.toml

celestia-appd keys add relayer --output=json --keyring-backend test 2>./celestia-relayer-key.json
osmosisd keys add relayer --output=json --keyring-backend test 2>./osmosis-relayer-key.json
hermes keys add --chain osmosis-dev-1 --key-file './osmosis-relayer-key.json'
hermes keys add --chain celestia-dev-1 --key-file './celestia-relayer-key.json'

OSMOSIS_ADDR=$(jq -r '.address' ./osmosis-relayer-key.json)
wait_tx osmosisd tx bank send validator1 "$OSMOSIS_ADDR" 50000000stake \
  --keyring-backend=test --home="$HOME"/.osmosisd/validator1 \
  --chain-id osmosis-dev-1 --fees 875stake -y
CELESTIA_ADDR=$(jq -r '.address' ./celestia-relayer-key.json)
wait_tx celestia-appd tx bank send validator1 "$CELESTIA_ADDR" 5000000000utia \
  --keyring-backend=test --home="$HOME"/.celestia-app/validator1 \
  --chain-id celestia-dev-1 --fees 21000utia -y --node http://0.0.0.0:26661

hermes create client --host-chain celestia-dev-1 --reference-chain osmosis-dev-1
hermes create client --host-chain osmosis-dev-1 --reference-chain celestia-dev-1
hermes create connection --a-chain celestia-dev-1 --b-chain osmosis-dev-1
hermes create channel --a-chain celestia-dev-1 --a-connection connection-0 --a-port transfer --b-port transfer

echo "--- HERMES READY ---"

# in case update the config here with the new channels

tmux new -s hermes -d hermes start

# start a loop to clear packets
tmux new -s hermes-clear-lopp -d sh ./local-hermes-clear-packets.sh
