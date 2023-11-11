#!/bin/sh

# Stop script execution if an error is encountered
set -o errexit
# Stop script execution if an undefined variable is used
set -o nounset

killall celestia-appd || true

CHAIN_ID="celestia-dev-1"
KEY_NAME="validator"
KEYRING_BACKEND="test"
COINS="1000000000000000utia"
DELEGATION_AMOUNT="5000000000utia"
CELESTIA_APP_HOME="${HOME}/.celestia-app"
CELESTIA_APP_VERSION=$(celestia-appd version 2>&1)

echo "celestia-app home: ${CELESTIA_APP_HOME}"
echo "celestia-app version: ${CELESTIA_APP_VERSION}"
echo ""

echo "Initializing validator and node config files..."
celestia-appd init ${CHAIN_ID} \
  --chain-id ${CHAIN_ID} \
  --home ${CELESTIA_APP_HOME}

echo "Adding a new key to the keyring..."
celestia-appd keys add ${KEY_NAME} \
  --keyring-backend=${KEYRING_BACKEND} \
  --home ${CELESTIA_APP_HOME}

echo "Adding genesis account..."
celestia-appd add-genesis-account \
  $(celestia-appd keys show ${KEY_NAME} -a --keyring-backend=${KEYRING_BACKEND} --home ${CELESTIA_APP_HOME}) \
  $COINS \
  --home ${CELESTIA_APP_HOME}

echo "Creating a genesis tx..."
celestia-appd gentx ${KEY_NAME} ${DELEGATION_AMOUNT} \
  --keyring-backend=${KEYRING_BACKEND} \
  --chain-id ${CHAIN_ID} \
  --home ${CELESTIA_APP_HOME}

echo "Collecting genesis txs..."
celestia-appd collect-gentxs \
  --home ${CELESTIA_APP_HOME}

# Set proper defaults and change ports
# If you encounter: `sed: -I or -i may not be used with stdin` on MacOS you can mitigate by installing gnu-sed
# https://gist.github.com/andre3k1/e3a1a7133fded5de5a9ee99c87c6fa0d?permalink_comment_id=3082272#gistcomment-3082272
sed -i'.bak' 's#"tcp://127.0.0.1:26657"#"tcp://0.0.0.0:26657"#g' "${CELESTIA_APP_HOME}"/config/config.toml
sed -i'.bak' 's#"null"#"kv"#g' "${CELESTIA_APP_HOME}"/config/config.toml

# Start celestia-app
echo "Starting celestia-app..."
tmux new -s celestia1 -d celestia-appd start \
  --home ${CELESTIA_APP_HOME} \
  --api.enable \
  --grpc.enable \
  --grpc-web.enable \
  --p2p.laddr tcp://0.0.0.0:26660 \
  --rpc.laddr tcp://0.0.0.0:26661 \
  --api.address tcp://0.0.0.0:1340 \
  --grpc.address 0.0.0.0:9190 \
  --grpc-web.address 0.0.0.0:9191