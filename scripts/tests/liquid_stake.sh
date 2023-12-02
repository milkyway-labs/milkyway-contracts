ADMIN_OSMOSIS=osmo1sfhy3emrgp26wnzuu64p06kpkxd9phel8ym0ge
ADMIN_CELESTIA=celestia1sfhy3emrgp26wnzuu64p06kpkxd9phel74e0yx
CODE_ID=$(osmosisd query wasm list-code --output json | jq -r '.code_infos[-1].code_id')
CONTRACT=$(osmosisd query wasm list-contract-by-code $CODE_ID --output json | jq -r '.contracts[-1]')

# send some tokens
celestia-appd tx ibc-transfer transfer transfer channel-0 --from test_master --keyring-backend test --node http://localhost:26661 --chain-id celestia-dev-1 --fees 21000utia --output json -y osmo1sfhy3emrgp26wnzuu64p06kpkxd9phel8ym0ge 10000000utia  --broadcast-mode block | jq -r '.raw_log'

# check ibc token denom and if tokens have arrived
RESERVE_TOKEN=""
while [ -z "$RESERVE_TOKEN" ]; do
    BALANCES=$(osmosisd query bank balances $ADMIN_OSMOSIS --output json)
    echo $BALANCES
    RESERVE_TOKEN=$(echo $BALANCES | jq -r '.balances[].denom | select(. | contains("ibc/"))')
    sleep 3
done;

# liquid stake
osmosisd tx wasm execute $CONTRACT '{"liquid_stake":{}}' \
    --amount 1000$RESERVE_TOKEN \
    --from test_master --keyring-backend test \
    -y -b block \
    --gas-prices 0.025stake --gas-adjustment 1.7 --gas auto  \
    --chain-id osmosis-dev-1 --output json | jq -r '.raw_log'

# check balances
# osmosisd query bank balances $ADMIN_OSMOSIS
# osmosisd query bank balances $CONTRACT
# celestia-appd query bank balances $ADMIN_CELESTIA --node http://localhost:26661
# osmosisd query wasm contract-state smart $CONTRACT '{"state":{}}'
# osmosisd query wasm contract-state smart $CONTRACT '{"ibc_queue":{}}' --output json | jq -r '.ibc_queue'
# osmosisd query wasm contract-state smart $CONTRACT '{"batches":{}}'

# check packets (inside docker)
# hermes query packet pending --chain osmosis-dev-1 --port transfer --channel channel-0

# recover packets
osmosisd tx wasm execute $CONTRACT '{"recover_pending_ibc_transfers":{}}' \
    --from test_master -y \
    --node http://localhost:26657 -y -b block \
    --gas-prices 0.025stake --gas-adjustment 1.7 --gas auto  \
    --chain-id osmosis-dev-1