#!/bin/sh

# Stop script execution if an error is encountered
set -o errexit
# Stop script execution if an undefined variable is used
set -o nounset

killall celestia-appd || true

CELESTIA_APP_HOME="${HOME}/.celestia-app"

echo "celestia-app home: ${CELESTIA_APP_HOME}"CELESTIA_ADDR=$(jq -r '.address' ./celestia-relayer-key.json)
echo ""

# Start celestia-app
echo "Starting celestia-app..."
tmux new -s celestiavalidator1 -d celestia-appd start \
  --home ${CELESTIA_APP_HOME} \
  --api.enable \
  --grpc.enable \
  --grpc-web.enable \
  --p2p.laddr tcp://0.0.0.0:26660 \
   --rpc.laddr tcp://0.0.0.0:26661 \
   --api.address tcp://0.0.0.0:1340 \
   --grpc.address 0.0.0.0:9190 \
   --grpc-web.address 0.0.0.0:9191
