docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/workspace-optimizer-arm64:0.13.0

RES=$(osmosisd tx wasm store ../artifacts/staking-aarch64.wasm --from validator1 --output json --node http://localhost:26657 -y -b block --gas-prices 0.025stake --gas-adjustment 1.7 --gas auto  --home ~/.osmosisd/validator1 --keyring-backend test --chain-id testing)
CODE_ID=$(echo $RES | jq -r '.logs[0].events[-1].attributes[-1].value')
ADMIN=$(osmosisd keys show validator1 --keyring-backend=test --home=~/.osmosisd/validator1 --output json | jq -r '.address')
OSMOSIS_VALIDATOR_1=$(osmosisd keys show validator1 --keyring-backend=test --home=~/.osmosisd/validator1 --output json | jq -r '.address')
OSMOSIS_VALIDATOR_2=$(osmosisd keys show validator2 --keyring-backend=test --home=~/.osmosisd/validator2 --output json | jq -r '.address')
OSMOSIS_VALIDATOR_3=$(osmosisd keys show validator3 --keyring-backend=test --home=~/.osmosisd/validator3 --output json | jq -r '.address')
CELESTIA_VALIDATOR_1=$(celestia-appd keys show validator --keyring-backend=test --home=~/.celestia-app --output json | jq -r '.address')
INIT={\"native_token_denom\":\"osmoTIA\",\"liquid_stake_token_denom\":\"mlk\",\"treasury_address\":\"$OSMOSIS_VALIDATOR_1\",\"node_operators\":[\"$OSMOSIS_VALIDATOR_1\",\"$OSMOSIS_VALIDATOR_2\",\"$OSMOSIS_VALIDATOR_3\"],\"validators\":[\"$OSMOSIS_VALIDATOR_1\"],\"batch_period\":86400,\"unbonding_period\":1209600,\"protocol_fee_config\":{\"dao_treasury_fee\":\"10\"},\"multisig_address_config\":{\"controller_address\":\"$OSMOSIS_VALIDATOR_1\",\"staker_address\":\"$OSMOSIS_VALIDATOR_1\",\"reward_collector_address\":\"$OSMOSIS_VALIDATOR_1\"},\"minimum_liquid_stake_amount\":\"100\",\"minimum_rewards_to_collect\":\"10\"}
osmosisd tx wasm instantiate $CODE_ID $INIT \
    --from validator1 --label "milkyway test" -y \
    --admin "$ADMIN" --node http://localhost:26657 -y -b block \
    --gas-prices 0.025stake --gas-adjustment 1.7 --gas auto  \
    --home ~/.osmosisd/validator1 --keyring-backend test --chain-id testing \
    --amount 10000000uosmo --output json
CONTRACT=$(osmosisd query wasm list-contract-by-code $CODE_ID --node http://localhost:26657 --output json | jq -r '.contracts[-1]')
echo $CONTRACT