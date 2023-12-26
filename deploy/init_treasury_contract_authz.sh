ADMIN_ACCOUNT=osmo1zmptm0xcrrhmje5tgrppjktkskcnnmkr6v5h3t

osmosisd tx wasm store ./artifacts/treasury-aarch64.wasm --from $ADMIN_ACCOUNT \
    -y -b block --gas-prices 0.025uosmo --gas-adjustment 1.7 --gas auto --chain-id osmosis-1 \
    --node https://rpc.osmosis.zone:443 --generate-only --sequence 12 \
    > tx.json

osmosisd tx authz exec tx.json --from contract --node https://rpc.osmosis.zone:443 --gas-prices 0.025uosmo --gas-adjustment 1.7 --gas auto --chain-id osmosis-1

CODE_ID=378

INIT={}
osmosisd tx wasm instantiate $CODE_ID $INIT --label "Milkyway Treasury" \
    --from $ADMIN_ACCOUNT --admin $ADMIN_ACCOUNT \
    -y -b block --gas-prices 0.025uosmo --gas-adjustment 1.7 --gas auto --chain-id osmosis-1 \
    --node https://rpc.osmosis.zone:443 --generate-only --sequence 12 \
    > tx.json
    
osmosisd tx authz exec tx.json --from contract --node https://rpc.osmosis.zone:443 --gas-prices 0.025uosmo --gas-adjustment 1.7 --gas auto --chain-id osmosis-1
