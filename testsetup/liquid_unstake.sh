ADMIN_OSMOSIS=osmo1sfhy3emrgp26wnzuu64p06kpkxd9phel8ym0ge
ADMIN_CELESTIA=celestia1sfhy3emrgp26wnzuu64p06kpkxd9phel74e0yx
CODE_ID=$(osmosisd query wasm list-code --output json | jq -r '.code_infos[-1].code_id')
CONTRACT=$(osmosisd query wasm list-contract-by-code $CODE_ID --output json | jq -r '.contracts[-1]')

# assert that balance has stTIA
if [ -z "$(osmosisd query bank balances $ADMIN_OSMOSIS | grep "factory/$CONTRACT/mlk")" ]; then
    echo "stTIA not found"
fi

# liquid unstake
BATCH_ID=$(osmosisd tx wasm execute $CONTRACT '{"liquid_unstake":{}}' \
    --amount 1000factory/$CONTRACT/mlk \
    --from test_master --keyring-backend test --keyring-backend test -y \
    --node http://localhost:26657 -y -b block \
    --gas-prices 0.025stake --gas-adjustment 1.7 --gas auto  \
    --chain-id osmosis-dev-1 | jq -r '.[].events[] | select(.type == "wasm") | .attributes[] | select(.key == "batch") | .value')

# check liquid unstake request
osmosisd query wasm contract-state smart osmo1nc5tatafv6eyq7llkr2gv50ff9e22mnf70qgjlv737ktmt4eswrqvlx82r '{"batches":{}}' --node https://osmosis-rpc.milkyway.hanjun.kim:443