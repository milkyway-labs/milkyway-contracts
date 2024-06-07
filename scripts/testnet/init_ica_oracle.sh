#!/bin/bash
# init in stride folder

RES=$(osmosisd tx wasm store ./artifacts/ica_oracle.wasm --from test_master --keyring-backend test --output json --node http://localhost:26657 -y -b block --gas-prices 0.025stake --gas-adjustment 1.7 --gas auto --chain-id osmosis-dev-1)
CODE_ID=$(osmosisd query wasm list-code --output json | jq -r '.code_infos[-1].code_id')

STAKE_CONTRACT=osmo1lnfn4f9gpz8fmvquarmuph62wmss3qwh92j98z4y4q4x566stkmsnfuffg
ADMIN_OSMOSIS=osmo1sfhy3emrgp26wnzuu64p06kpkxd9phel8ym0ge

INIT={\"admin_address\":\"$STAKE_CONTRACT\"}
osmosisd tx wasm instantiate $CODE_ID $INIT \
  --from test_master --keyring-backend test --label "milkyway test" -y \
  --admin "$ADMIN_OSMOSIS" --node http://localhost:26657 -y -b block \
  --gas-prices 0.025stake --gas-adjustment 1.7 --gas auto \
  --chain-id osmosis-dev-1
CONTRACT=$(osmosisd query wasm list-contract-by-code $CODE_ID --node http://localhost:26657 --output json | jq -r '.contracts[-1]')
echo $CONTRACT
