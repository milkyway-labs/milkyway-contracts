
docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/workspace-optimizer-arm64:0.14.0

# cargo install --git https://github.com/cmoog/bech32         

RES=$(osmosisd tx wasm store ../artifacts/staking-aarch64.wasm --from test_master --output json --node http://localhost:26657 -y -b block --gas-prices 0.025stake --gas-adjustment 1.7 --gas auto --chain-id osmosis-dev-1)
CODE_ID=$(osmosisd query wasm list-code --output json | jq -r '.code_infos[-1].code_id')
ADMIN_OSMOSIS=osmo1sfhy3emrgp26wnzuu64p06kpkxd9phel8ym0ge
ADMIN_CELESTIA=celestia1sfhy3emrgp26wnzuu64p06kpkxd9phel74e0yx
# token depends on channel it was send over
# find the token by sending it to you and then reading it
# celestia-appd tx ibc-transfer transfer transfer channel-0 --from test_master --node http://localhost:26661 --chain-id celestia-dev-1 --fees 21000utia --output json -y osmo1sfhy3emrgp26wnzuu64p06kpkxd9phel8ym0ge 10000000utia  --broadcast-mode block | jq -r '.raw_log'
# osmosisd query bank balances osmo1sfhy3emrgp26wnzuu64p06kpkxd9phel8ym0ge
RESERVE_TOKEN="ibc/C3E53D20BC7A4CC993B17C7971F8ECD06A433C10B6A96F4C4C3714F0624C56DA"
VALIDATORS=$(osmosisd query staking validators --output json | jq -r '.validators | map(.operator_address) | join(",")')
OSMOSIS_VALIDATOR_1=$(echo $VALIDATORS | cut -d',' -f1 | bech32 --decode | bech32 --prefix osmo)
CELESTIA_VALIDATOR_1=$(celestia-appd query staking validators --node http://localhost:26661 --output json | jq -r '.validators | map(.operator_address) | join(",")' | cut -d',' -f1 | bech32 --decode | bech32 --prefix celestia)
CELESTIA_VALIDATOR_2=$(celestia-appd query staking validators --node http://localhost:26661 --output json | jq -r '.validators | map(.operator_address) | join(",")' | cut -d',' -f2 | bech32 --decode | bech32 --prefix celestia)
CELESTIA_VALIDATOR_3=$(celestia-appd query staking validators --node http://localhost:26661 --output json | jq -r '.validators | map(.operator_address) | join(",")' | cut -d',' -f3 | bech32 --decode | bech32 --prefix celestia)
INIT={\"native_token_denom\":\"$RESERVE_TOKEN\",\"liquid_stake_token_denom\":\"mlk\",\"treasury_address\":\"$ADMIN_OSMOSIS\",\"operators\":[\"$ADMIN_OSMOSIS\"],\"validators\":[\"$CELESTIA_VALIDATOR_1\",\"$CELESTIA_VALIDATOR_2\",\"$CELESTIA_VALIDATOR_3\"],\"batch_period\":86400,\"unbonding_period\":1209600,\"protocol_fee_config\":{\"dao_treasury_fee\":\"10\"},\"multisig_address_config\":{\"controller_address\":\"$ADMIN_CELESTIA\",\"staker_address\":\"$ADMIN_CELESTIA\",\"reward_collector_address\":\"$ADMIN_CELESTIA\"},\"minimum_liquid_stake_amount\":\"100\",\"minimum_rewards_to_collect\":\"10\",\"ibc_channel_id\":\"channel-0\"}
osmosisd tx wasm instantiate $CODE_ID $INIT \
    --from test_master --label "milkyway test" -y \
    --admin "$ADMIN_OSMOSIS" --node http://localhost:26657 -y -b block \
    --gas-prices 0.025stake --gas-adjustment 1.7 --gas auto  \
    --chain-id osmosis-dev-1
CONTRACT=$(osmosisd query wasm list-contract-by-code $CODE_ID --node http://localhost:26657 --output json | jq -r '.contracts[-1]')
echo $CONTRACT