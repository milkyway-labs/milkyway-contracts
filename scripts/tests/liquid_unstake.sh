ADMIN_OSMOSIS=osmo1sfhy3emrgp26wnzuu64p06kpkxd9phel8ym0ge
ADMIN_CELESTIA=celestia1sfhy3emrgp26wnzuu64p06kpkxd9phel74e0yx
CODE_ID=$(osmosisd query wasm list-code --output json | jq -r '.code_infos[-1].code_id')
CONTRACT=$(osmosisd query wasm list-contract-by-code $CODE_ID --output json | jq -r '.contracts[-1]')

# assert that balance has stTIA
if [ -z "$(osmosisd query bank balances $ADMIN_OSMOSIS | grep "factory/$CONTRACT/mlk")" ]; then
    echo "stTIA not found"
fi

# liquid unstake
osmosisd tx wasm execute $CONTRACT '{"liquid_unstake":{}}' \
    --amount 1000factory/$CONTRACT/mlk \
    --from test_master --keyring-backend test -y \
    --node http://localhost:26657 -y -b block \
    --gas-prices 0.025stake --gas-adjustment 1.7 --gas auto  \
    --chain-id osmosis-dev-1

# check liquid unstake request
osmosisd query wasm contract-state smart $CONTRACT '{"batch":{"id":1}}'

# unstake on celestia
# CELESTIA_VALIDATOR_1=$(celestia-appd query staking validators --node http://localhost:26661 --output json | jq -r '.validators[] | select(.description.moniker == "validator1") | .operator_address')
# CELESTIA_VALIDATOR_2=$(celestia-appd query staking validators --node http://localhost:26661 --output json | jq -r '.validators[] | select(.description.moniker == "validator2") | .operator_address')
# CELESTIA_VALIDATOR_3=$(celestia-appd query staking validators --node http://localhost:26661 --output json | jq -r '.validators[] | select(.description.moniker == "validator3") | .operator_address')
# CELESTIA_VALIDATOR_2_OPERATOR=$(celestia-appd keys show validator2 --keyring-backend=test --home=$HOME/.celestia-app/validator2 --output json | jq -r '.address')
# celestia-appd tx staking unbond $CELESTIA_VALIDATOR_2 10utia --from validator2 --chain-id="celestia-dev-1" --broadcast-mode block --node http://localhost:26661 --yes --keyring-backend=test --home=$HOME/.celestia-app/validator2 --fees 21000utia
# celestia-appd query staking --node http://localhost:26661 unbonding-delegation $CELESTIA_VALIDATOR_2_OPERATOR $CELESTIA_VALIDATOR_2

osmosisd tx wasm execute $CONTRACT '{"submit_batch":{}}' \
    --from test_master --keyring-backend test \
    -y -b block \
    --gas-prices 0.025stake --gas-adjustment 1.7 --gas auto  \
    --chain-id osmosis-dev-1
osmosisd query wasm contract-state smart $CONTRACT '{"batch":{"id":1}}'

RESERVE_TOKEN="ibc/C3E53D20BC7A4CC993B17C7971F8ECD06A433C10B6A96F4C4C3714F0624C56DA"
osmosisd tx wasm execute $CONTRACT '{"receive_unstaked_tokens":{}}' \
    --from test_master --keyring-backend test -y \
    --node http://localhost:26657 -y -b block \
    --gas-prices 0.025stake --gas-adjustment 1.7 --gas auto  \
    --chain-id osmosis-dev-1 \
    --amount 100000$RESERVE_TOKEN

MEMO='{"wasm":{"contract":"'$CONTRACT'","msg":{"receive_rewards":{}}}}'
celestia-appd tx ibc-transfer transfer transfer channel-0 --from test_master --keyring-backend test --node http://localhost:26661 --chain-id celestia-dev-1 --fees 21000utia --output json -y $CONTRACT 1000utia  --broadcast-mode block --memo "$MEMO"

osmosisd query wasm contract-state smart $CONTRACT '{"batch":{"id":1}}'