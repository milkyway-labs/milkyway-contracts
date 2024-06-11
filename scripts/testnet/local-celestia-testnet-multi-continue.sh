#!/bin/bash

SCRIPT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)
BINS_DIR=$SCRIPT_DIR/bins
PATH=$BINS_DIR:$PATH

# Stop script execution if an error is encountered
set -o errexit
# Stop script execution if an undefined variable is used
set -o nounset

killall celestia-appd || true

# Start celestia-app
echo "Starting celestia-app..."
tmux new -s celestia2 -d celestia-appd start --home=$HOME/.celestia-app/validator2
tmux new -s celestia3 -d celestia-appd start --home=$HOME/.celestia-app/validator3
tmux new -s celestia1 -d celestia-appd start --home=$HOME/.celestia-app/validator1 \
  --api.enable \
  --grpc.enable \
  --grpc-web.enable \
  --api.address tcp://0.0.0.0:1314 \
  --grpc.address 0.0.0.0:9084 \
  --grpc-web.address 0.0.0.0:9085
