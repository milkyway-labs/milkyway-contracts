CONTRACT=osmo1h6d53zdzp4dwqr742qvzlucafghuhus7653su0f8cfdfzzkww4as9389xs
RES=$(osmosisd tx wasm store ./artifacts/staking-aarch64.wasm --from test_master --keyring-backend test --output json -y -b block --gas-prices 0.025uosmo --gas-adjustment 1.7 --gas auto --chain-id osmo-test-5)
CODE_ID=$(echo $RES | jq -r '.logs[0].events[-1].attributes[-1].value')
osmosisd tx wasm migrate $CONTRACT $CODE_ID '{}' \
    --from test_master --keyring-backend test -y \
    -b block \
    --gas-prices 0.025uosmo --gas-adjustment 1.7 --gas auto  \
    --chain-id osmo-test-5 --node https://rpc.testnet.osmosis.zone:443