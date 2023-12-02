RES=$(osmosisd tx wasm store ./artifacts/staking-aarch64.wasm --from test_master --keyring-backend --output json --node https://rpc.testnet.osmosis.zone:443 -y -b block --gas-prices 0.025uosmo --gas-adjustment 1.7 --gas auto --chain-id osmo-test-5)
CODE_ID=$(echo $RES | jq -r '.raw_log | fromjson | .[0].events[] | select(.type == "store_code") | .attributes[] | select(.key == "code_id") | .value')
ADMIN_OSMOSIS=osmo1sfhy3emrgp26wnzuu64p06kpkxd9phel8ym0ge
ADMIN_CELESTIA=celestia1sfhy3emrgp26wnzuu64p06kpkxd9phel74e0yx

CHANNEL_CELESTIA=channel-25
CHANNEL=channel-4370

# token depends on channel it was send over
# find the token by sending it to you and then reading it
celestia-appd tx ibc-transfer transfer transfer $CHANNEL_CELESTIA --from test_master --keyring-backend test --node https://rpc.celestia-mocha.com:443 --chain-id mocha-4 osmo1sfhy3emrgp26wnzuu64p06kpkxd9phel8ym0ge 1utia --fees 21000utia -y
osmosisd query bank balances osmo1sfhy3emrgp26wnzuu64p06kpkxd9phel8ym0ge --node https://rpc.testnet.osmosis.zone:443

RESERVE_TOKEN="ibc/0E22FFB61DB307FE01D3D0DFF4A8EBEB6CC4997DCF9E901AE0751A2FFF4B90DD"
CELESTIA_VALIDATOR_1=$(celestia-appd query staking validators --node https://rpc.celestia-mocha.com:443 --output json | jq -r '.validators | map(.operator_address) | join(",")' | cut -d',' -f1 | bech32 --decode | bech32 --prefix celestiavaloper)
CELESTIA_VALIDATOR_2=$(celestia-appd query staking validators --node https://rpc.celestia-mocha.com:443 --output json | jq -r '.validators | map(.operator_address) | join(",")' | cut -d',' -f2 | bech32 --decode | bech32 --prefix celestiavaloper)
CELESTIA_VALIDATOR_3=$(celestia-appd query staking validators --node https://rpc.celestia-mocha.com:443 --output json | jq -r '.validators | map(.operator_address) | join(",")' | cut -d',' -f3 | bech32 --decode | bech32 --prefix celestiavaloper)
UNBONDING_PERIOD=$(celestia-appd query staking params --node https://rpc.celestia-mocha.com:443 --output json | jq -r '.unbonding_time | .[:-1]')
BATCH_PERIOD=$(echo "scale=2; ($UNBONDING_PERIOD + 6) / 7" | bc)
BATCH_PERIOD=${BATCH_PERIOD%.*}
INIT={\"native_token_denom\":\"$RESERVE_TOKEN\",\"liquid_stake_token_denom\":\"stTIA\",\"treasury_address\":\"$ADMIN_OSMOSIS\",\"operators\":[\"$ADMIN_OSMOSIS\"],\"validators\":[\"$CELESTIA_VALIDATOR_1\",\"$CELESTIA_VALIDATOR_2\",\"$CELESTIA_VALIDATOR_3\"],\"batch_period\":86400,\"unbonding_period\":1209600,\"protocol_fee_config\":{\"dao_treasury_fee\":\"10\"},\"multisig_address_config\":{\"staker_address\":\"$ADMIN_CELESTIA\",\"reward_collector_address\":\"$ADMIN_CELESTIA\"},\"minimum_liquid_stake_amount\":\"100\",\"ibc_channel_id\":\"$CHANNEL\",\"pool_id\":1}
RES=$(osmosisd tx wasm instantiate $CODE_ID $INIT \
    --from test_master --keyring-backend test --label "milkyway test" -y \
    --admin "$ADMIN_OSMOSIS" --node https://rpc.testnet.osmosis.zone:443 -y -b block \
    --gas-prices 0.025uosmo --gas-adjustment 1.7 --gas auto  \
    --chain-id osmo-test-5 --output json)
CONTRACT=$(echo $RES | jq -r '.raw_log | fromjson | .[0].events[] | select(.type == "instantiate") | .attributes[] | select(.key == "_contract_address") | .value')
echo $CONTRACT

# current deployment
# osmo10nghhpea7rc78n4h3vcjy5rsq8m8supwpyrd3avgp7dtlh7zl4xqrnkhs4