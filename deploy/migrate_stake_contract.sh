ADMIN_OSMOSIS=osmo1sfhy3emrgp26wnzuu64p06kpkxd9phel8ym0ge

PAGE=0
while true; do
  OUT=$(osmosisd query wasm list-code --page $PAGE --output json)
  INFOS=$(echo $OUT | jq -r '.code_infos')
  break
  for INFO in $(echo $INFOS | jq -c '.[]'); do
    CREATOR=$(echo $INFO | jq -r '.creator') 
    if [ "$CREATOR" == "$ADMIN_OSMOSIS" ]; then
      CODE_ID=$(echo $INFO | jq -r '.code_id')
      break
    fi
  done

  NEXT=$(echo "$OUT" | jq -r '.pagination.next_key')

  if [ -z "$NEXT" ]; then
    echo "No code found for creator $ADMIN_OSMOSIS"
    exit 1
  fi

  PAGE=$NEXT
done
CONTRACT=$(osmosisd query wasm list-contract-by-code $CODE_ID --output json | jq -r '.contracts[-1]')

RES=$(osmosisd tx wasm store ./artifacts/staking-aarch64.wasm --from test_master --output json -y -b block --gas-prices 0.025stake --gas-adjustment 1.7 --gas auto --chain-id osmosis-dev-1)
CODE_ID=$(echo $RES | jq -r '.logs[0].events[-1].attributes[-1].value')
osmosisd tx wasm migrate $CONTRACT $CODE_ID '{}' \
    --from test_master -y \
    --node https://rpc.testnet.osmosis.zone:443 -y -b block \
    --gas-prices 0.025uosmo --gas-adjustment 1.7 --gas auto  \
    --chain-id osmo-test-5 && echo "Success" || echo "Failure"
