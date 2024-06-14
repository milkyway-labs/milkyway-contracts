#!/bin/bash

SCRIPT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)
BINS_DIR="$SCRIPT_DIR/bins"
PATH="$BINS_DIR:$PATH"

source "$SCRIPT_DIR/params.sh"
source "$SCRIPT_DIR/utils/tx.sh"

# this create a tia osmo pool on osmosis testnet

# send tia
OSMOSIS_VALIDATOR_1_ADDR=$(osmosisd keys show validator1 --address --keyring-backend=test --home="$HOME/.osmosisd/validator1")
wait_tx celestia-appd tx ibc-transfer transfer transfer channel-0 "$OSMOSIS_VALIDATOR_1_ADDR" 10000000000000utia \
  --from validator1  --keyring-backend=test --home="$HOME"/.celestia-app/validator1 \
  "$CELESTIA_TX_PARAMS"

# get tia ibc token name
echo "waiting for tia to arrive"
NATIVE_TOKEN_DENOM=""
while [ -z "$NATIVE_TOKEN_DENOM" ]; do
  BALANCES=$(osmosisd query bank balances "$OSMOSIS_VALIDATOR_1_ADDR" --output json)
  echo "$BALANCES"
  NATIVE_TOKEN_DENOM=$(echo "$BALANCES" | jq -r '.balances[].denom | select(. | contains("ibc/"))')
  sleep 3
done

echo '{
  "initial-deposit": "10000000000000uosmo,10000000000000ibc/C3E53D20BC7A4CC993B17C7971F8ECD06A433C10B6A96F4C4C3714F0624C56DA",
  "swap-fee": "0.01",
  "exit-fee": "0",
  "weights": "10uosmo,1ibc/C3E53D20BC7A4CC993B17C7971F8ECD06A433C10B6A96F4C4C3714F0624C56DA"
}' > pool.json

wait_tx osmosisd tx gamm create-pool --pool-file ./pool.json \
   --from validator1 --keyring-backend=test --home="$HOME"/.osmosisd/validator1 \
   "$OSMOSIS_TX_PARAMS"

# add liquidity
# osmosisd tx gamm join-pool --pool-id 1 --max-amounts-in 1000000$NATIVE_TOKEN_DENOM,1000000uosmo --share-amount-out 1000 --from test_master --keyring-backend test -y \
#     --node http://localhost:26657 -y -b block \
#     --gas-prices 0.025stake --gas-adjustment 1.7 --gas auto  \
#     --chain-id osmosis-dev-1
