# this create a tia osmo pool on osmosis testnet

# send tia
celestia-appd tx ibc-transfer transfer transfer channel-0 --from test_master --keyring-backend test --node http://localhost:26661 --chain-id celestia-dev-1 --fees 21000utia --output json -y osmo1sfhy3emrgp26wnzuu64p06kpkxd9phel8ym0ge 10000000utia  --broadcast-mode block | jq -r '.raw_log'

# get tia ibc token name
osmosisd query bank balances osmo1sfhy3emrgp26wnzuu64p06kpkxd9phel8ym0ge
RESERVE_TOKEN="ibc/0E22FFB61DB307FE01D3D0DFF4A8EBEB6CC4997DCF9E901AE0751A2FFF4B90DD"

osmosisd tx gamm create-pool --pool-file ./pool.json \
    --from test_master --keyring-backend test -y \
    --node https://rpc.testnet.osmosis.zone:443 -b block \
    --gas-prices 0.025uosmo --gas-adjustment 1.7 --gas auto  \
    --chain-id osmo-test-5 --output json