ADMIN_OSMOSIS=osmo1sfhy3emrgp26wnzuu64p06kpkxd9phel8ym0ge
ADMIN_CELESTIA=celestia1sfhy3emrgp26wnzuu64p06kpkxd9phel74e0yx

# send some tokens
celestia-appd tx ibc-transfer transfer transfer channel-0 --from test_master --node http://localhost:26661 --chain-id celestia-dev-1 --fees 21000utia --output json -y osmo1sfhy3emrgp26wnzuu64p06kpkxd9phel8ym0ge 10000000utia  --broadcast-mode block | jq -r '.raw_log'

# check ibc token denom and if tokens have arrived
osmosisd query bank balances osmo1sfhy3emrgp26wnzuu64p06kpkxd9phel8ym0ge
RESERVE_TOKEN="ibc/C3E53D20BC7A4CC993B17C7971F8ECD06A433C10B6A96F4C4C3714F0624C56DA"

# liquid stake
osmosisd tx wasm execute $CONTRACT '{"liquid_stake":{}}' \
    --amount 1000$RESERVE_TOKEN \
    --from test_master -y \
    --node http://localhost:26657 -y -b block \
    --gas-prices 0.025stake --gas-adjustment 1.7 --gas auto  \
    --chain-id osmosis-dev-1

# check balances
osmosisd query bank balances $ADMIN_OSMOSIS
osmosisd query bank balances $CONTRACT
celestia-appd query bank balances $ADMIN_CELESTIA --node http://localhost:26661

# check packets (inside docker)
hermes query packet pending --chain osmosis-dev-1 --port transfer --channel channel-0
