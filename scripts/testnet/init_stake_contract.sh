#!/bin/bash
# cargo install --git https://github.com/cmoog/bech32

TXHASH=$(osmosisd tx wasm store ./artifacts/staking-aarch64.wasm --from test_master --keyring-backend test --output json --node http://localhost:26657 -y -b sync --gas-prices 0.025stake --gas-adjustment 1.7 --gas auto --chain-id osmosis-dev-1 | jq -r '.txhash')
# wait

CODE_ID=$(osmosisd query wasm list-code --output json | jq -r '.code_infos[-1].code_id')
ADMIN_OSMOSIS=osmo1sfhy3emrgp26wnzuu64p06kpkxd9phel8ym0ge
ADMIN_CELESTIA=celestia1sfhy3emrgp26wnzuu64p06kpkxd9phel74e0yx
# token depends on channel it was send over
# find the token by sending it to you and then reading it
# celestia-appd tx ibc-transfer transfer transfer channel-0 --from test_master --node http://localhost:26661 --chain-id celestia-dev-1 --fees 21000utia --output json -y osmo1sfhy3emrgp26wnzuu64p06kpkxd9phel8ym0ge 10000000utia  --broadcast-mode block | jq -r '.raw_log'
# osmosisd query bank balances osmo1sfhy3emrgp26wnzuu64p06kpkxd9phel8ym0ge
NATIVE_TOKEN_DENOM="ibc/C3E53D20BC7A4CC993B17C7971F8ECD06A433C10B6A96F4C4C3714F0624C56DA"
VALIDATORS=$(osmosisd query staking validators --output json | jq -r '.validators | map(.operator_address) | join(",")')
OSMOSIS_VALIDATOR_1=$(echo $VALIDATORS | cut -d',' -f1 | bech32 --decode | bech32 --prefix osmo)
CELESTIA_VALIDATOR_1=$(celestia-appd query staking validators --node http://localhost:26661 --output json | jq -r '.validators[] | select(.description.moniker == "validator1") | .operator_address')
CELESTIA_VALIDATOR_2=$(celestia-appd query staking validators --node http://localhost:26661 --output json | jq -r '.validators[] | select(.description.moniker == "validator2") | .operator_address')
CELESTIA_VALIDATOR_3=$(celestia-appd query staking validators --node http://localhost:26661 --output json | jq -r '.validators[] | select(.description.moniker == "validator3") | .operator_address')
UNBONDING_PERIOD=$(celestia-appd query staking params --node http://localhost:26661 --output json | jq -r '.unbonding_time | .[:-1]')
BATCH_PERIOD=$(echo "scale=2; ($UNBONDING_PERIOD + 6) / 7" | bc)
BATCH_PERIOD=${BATCH_PERIOD%.*}
INIT={\"native_token_denom\":\"$NATIVE_TOKEN_DENOM\",\"liquid_stake_token_denom\":\"milkTIA\",\"treasury_address\":\"$ADMIN_OSMOSIS\",\"monitors\":[\"$ADMIN_OSMOSIS\"],\"validators\":[\"$CELESTIA_VALIDATOR_1\",\"$CELESTIA_VALIDATOR_2\",\"$CELESTIA_VALIDATOR_3\"],\"batch_period\":86400,\"unbonding_period\":$UNBONDING_PERIOD,\"protocol_fee_config\":{\"dao_treasury_fee\":\"10\"},\"multisig_address_config\":{\"staker_address\":\"$ADMIN_CELESTIA\",\"reward_collector_address\":\"$ADMIN_CELESTIA\"},\"minimum_liquid_stake_amount\":\"100\",\"ibc_channel_id\":\"channel-0\"}
osmosisd tx wasm instantiate $CODE_ID $INIT \
  --from test_master --keyring-backend test --label "milkyway test" -y \
  --admin "$ADMIN_OSMOSIS" --node http://localhost:26657 -y -b sync \
  --gas-prices 0.025stake --gas-adjustment 1.7 --gas auto \
  --chain-id osmosis-dev-1

# wait

STAKE_CONTRACT=$(osmosisd query wasm list-contract-by-code $CODE_ID --node http://localhost:26657 --output json | jq -r '.contracts[-1]')
echo $STAKE_CONTRACT

# deploy oracle contract

# ORACLE_CONTRACT=osmo1nc5tatafv6eyq7llkr2gv50ff9e22mnf70qgjlv737ktmt4eswrqvlx82r
osmosisd tx wasm execute $STAKE_CONTRACT '{"update_config":{"oracle_contract_address":"'$ORACLE_CONTRACT'"}}' \
  --from test_master --keyring-backend test \
  -y -b sync \
  --gas-prices 0.025stake --gas-adjustment 1.7 --gas auto \
  --chain-id osmosis-dev-1

# wait

osmosisd tx wasm execute $STAKE_CONTRACT '{"resume_contract":{"total_native_token":"0","total_liquid_stake_token":"0","total_reward_amount":"0"}}' \
  --from test_master --keyring-backend test -y \
  --node http://localhost:26657 -y -b sync \
  --gas-prices 0.025stake --gas-adjustment 1.7 --gas auto \
  --chain-id osmosis-dev-1
