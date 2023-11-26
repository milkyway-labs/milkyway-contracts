ADMIN_OSMOSIS=osmo1sfhy3emrgp26wnzuu64p06kpkxd9phel8ym0ge
ADMIN_CELESTIA=celestia1sfhy3emrgp26wnzuu64p06kpkxd9phel74e0yx
CODE_ID=$(osmosisd query wasm list-code --output json | jq -r '.code_infos[-1].code_id')
CONTRACT=$(osmosisd query wasm list-contract-by-code $CODE_ID --output json | jq -r '.contracts[-1]')

# send some tokens
celestia-appd tx ibc-transfer transfer transfer channel-0 --from test_master --keyring-backend test --keyring-backend test --node http://localhost:26661 --chain-id celestia-dev-1 --fees 21000utia --output json -y osmo1sfhy3emrgp26wnzuu64p06kpkxd9phel8ym0ge 10000000utia  --broadcast-mode block | jq -r '.raw_log'

# check ibc token denom and if tokens have arrived
osmosisd query bank balances osmo1sfhy3emrgp26wnzuu64p06kpkxd9phel8ym0ge --node https://rpc.testnet.osmosis.zone:443 
RESERVE_TOKEN="ibc/0E22FFB61DB307FE01D3D0DFF4A8EBEB6CC4997DCF9E901AE0751A2FFF4B90DD"

# liquid stake
osmosisd tx wasm execute $CONTRACT '{"liquid_stake":{}}' \
    --amount 1000$RESERVE_TOKEN \
    --from test_master --keyring-backend test --keyring-backend test -y \
    --node https://rpc.testnet.osmosis.zone:443 -y -b block \
    --gas-prices 0.025uosmo --gas-adjustment 1.7 --gas auto  \
    --chain-id osmo-test-5

# check balances
osmosisd query bank balances $ADMIN_OSMOSIS
osmosisd query bank balances $CONTRACT
celestia-appd query bank balances $ADMIN_CELESTIA --node http://localhost:26661

# check packets (inside docker)
hermes query packet pending --chain osmosis-dev-1 --port transfer --channel channel-0

osmosisd query wasm contract-state smart $CONTRACT '{"ibc_queue":{}}' --output json | jq -r '.ibc_queue'
osmosisd query wasm contract-state smart osmo14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9sq2r9g9 '{"batches":{}}' --output json --node https://osmosis-rpc.milkyway.hanjun.kim:443
