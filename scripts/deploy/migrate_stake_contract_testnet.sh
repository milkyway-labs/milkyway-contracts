docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/workspace-optimizer-arm64:0.14.0

# double check contract address https://decento-labs.slack.com/canvas/C04R92P76L9
CONTRACT="osmo1h3s40qhqztppucdfggzwykz7xgwhcnps952y86xc73eupqfu9jmqcaghze"
RES=$(osmosisd tx wasm store ./artifacts/staking-aarch64.wasm --from test_master --output json -y -b block --gas-prices 0.025stake --gas-adjustment 1.7 --gas auto --chain-id osmosis-dev-1)
CODE_ID=$(echo $RES | jq -r '.logs[0].events[-1].attributes[-1].value')
osmosisd tx wasm migrate $CONTRACT $CODE_ID '{}' \
    --from test_master -y \
    -b block --node https://rpc.testnet.osmosis.zone:443 \
    --gas-prices 0.025uosmo --gas-adjustment 1.7 --gas auto  \
    --chain-id osmo-test-5
