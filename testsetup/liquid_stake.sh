# transfer tia
TXHASH=$(celestia-appd tx ibc-transfer transfer transfer channel-0 --from test_master --node http://localhost:26661 --chain-id celestia-dev-1 --fees 21000utia --output json -y osmo1sfhy3emrgp26wnzuu64p06kpkxd9phel8ym0ge 10000000utia  --broadcast-mode block | jq -r '.txhash')
celestia-appd --node http://localhost:26661 --chain-id celestia-dev-1 --output json query tx $TXHASH | jq -r '.raw_log'

# find denom of ibc TIA on osmosis
osmosisd query bank balances osmo1sfhy3emrgp26wnzuu64p06kpkxd9phel8ym0ge
RESERVE_TOKEN="ibc/C3E53D20BC7A4CC993B17C7971F8ECD06A433C10B6A96F4C4C3714F0624C56DA"

# init contract with reserve token
INIT={\"native_token_denom\":\"$RESERVE_TOKEN\",\"liquid_stake_token_denom\":\"mlk\",\"treasury_address\":\"$ADMIN_OSMOSIS\",\"node_operators\":[\"$ADMIN_OSMOSIS\"],\"validators\":[\"$ADMIN_OSMOSIS\"],\"batch_period\":86400,\"unbonding_period\":1209600,\"protocol_fee_config\":{\"dao_treasury_fee\":\"10\"},\"multisig_address_config\":{\"controller_address\":\"$ADMIN_CELESTIA\",\"staker_address\":\"$ADMIN_CELESTIA\",\"reward_collector_address\":\"$ADMIN_CELESTIA\"},\"minimum_liquid_stake_amount\":\"100\",\"minimum_rewards_to_collect\":\"10\",\"ibc_channel_id\":\"channel-0\"}
osmosisd tx wasm instantiate $CODE_ID $INIT \
    --from test_master --label "milkyway test" -y \
    --admin "$ADMIN_OSMOSIS" --node http://localhost:26657 -y -b block \
    --gas-prices 0.025stake --gas-adjustment 1.7 --gas auto  \
    --chain-id osmosis-dev-1 \
    --amount 10000000uosmo --output json
CONTRACT=$(osmosisd query wasm list-contract-by-code $CODE_ID --node http://localhost:26657 --output json | jq -r '.contracts[-1]')
echo $CONTRACT

# liquid stake
osmosisd tx wasm execute $CONTRACT '{"liquid_stake":{}}' \
    --amount 1000$RESERVE_TOKEN \
    --from test_master -y \
    --node http://localhost:26657 -y -b block \
    --gas-prices 0.025stake --gas-adjustment 1.7 --gas auto  \
    --chain-id osmosis-dev-1 \
    --output json