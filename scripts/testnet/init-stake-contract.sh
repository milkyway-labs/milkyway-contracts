#!/bin/bash
# cargo install --git https://github.com/cmoog/bech32

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
ARTIFACTS_DIR="${SCRIPT_DIR}/../../artifacts"
BINS_DIR="${SCRIPT_DIR}/bins"

# extend PATH to include the bin dirs.
PATH="${BINS_DIR}:${PATH}"

# Include testenet params
source "${SCRIPT_DIR}/params.sh"
# Include tx utils
source "${SCRIPT_DIR}/utils/tx.sh"

set -e

function store_contract() {
  local contract_file=$1
  if [ ! -f "$contract_file" ]; then
    return 1
  fi

  wait_tx osmosisd tx wasm store $1 --from test_master --keyring-backend test $OSMOSIS_TX_PARAMS
  echo $(osmosisd query wasm list-code --node $OSMOSIS_NODE --output json | jq -r '.code_infos[-1].code_id')
}

function init_contract() {
  local contract_code=$1
  local init_message=$2
  local label=$3

  wait_tx osmosisd tx wasm instantiate $contract_code $init_message \
     --label "$label" --admin "$OSMOSIS_ACCOUNT" \
     --from test_master --keyring-backend test \
    $OSMOSIS_TX_PARAMS

  echo $(osmosisd query wasm list-contract-by-code $contract_code --node $OSMOSIS_NODE --output json | jq -r '.contracts[-1]')
}

# token depends on channel it was send over
# find the token by sending it to you and then reading it
# celestia-appd tx ibc-transfer transfer transfer channel-0 --from test_master --node http://localhost:26661 --chain-id celestia-dev-1 --fees 21000utia --output json -y osmo1sfhy3emrgp26wnzuu64p06kpkxd9phel8ym0ge 10000000utia  --broadcast-mode block | jq -r '.raw_log'
# osmosisd query bank balances osmo1sfhy3emrgp26wnzuu64p06kpkxd9phel8ym0ge
NATIVE_TOKEN_DENOM="ibc/C3E53D20BC7A4CC993B17C7971F8ECD06A433C10B6A96F4C4C3714F0624C56DA"
VALIDATORS=$(osmosisd query staking validators --output json | jq -r '.validators | map(.operator_address) | join(",")')
CELESTIA_VALIDATOR_1=$(celestia-appd query staking validators --node http://localhost:26661 --output json | jq -r '.validators[] | select(.description.moniker == "validator1") | .operator_address')
CELESTIA_VALIDATOR_2=$(celestia-appd query staking validators --node http://localhost:26661 --output json | jq -r '.validators[] | select(.description.moniker == "validator2") | .operator_address')
CELESTIA_VALIDATOR_3=$(celestia-appd query staking validators --node http://localhost:26661 --output json | jq -r '.validators[] | select(.description.moniker == "validator3") | .operator_address')
UNBONDING_PERIOD=$(celestia-appd query staking params --node http://localhost:26661 --output json | jq -r '.unbonding_time | .[:-1]')
BATCH_PERIOD=$(echo "scale=2; ($UNBONDING_PERIOD + 6) / 7" | bc)
BATCH_PERIOD=${BATCH_PERIOD%.*}

# Contracts paths.
STAKING_CONTRACT_PATH="${ARTIFACTS_DIR}/staking.wasm"
ORACLE_CONTRACT_PATH="${ARTIFACTS_DIR}/oracle.wasm"
TREASURY_CONTRACT_PATH="${ARTIFACTS_DIR}/treasury.wasm"

# Store the staking contract
echo "Storing staking contract..."
STAKING_CONTRACT_CODE_ID=$(store_contract "$STAKING_CONTRACT_PATH")
echo "Store MilkyWay oracle contract"
ORACLE_CODE_ID=$(store_contract "$ORACLE_CONTRACT_PATH")
echo "Store MilkyWay treasury contract"
TREASURY_CODE_ID=$(store_contract "$TREASURY_CONTRACT_PATH")

echo ""
echo "All contracts stored!"
echo "Staking contract code: $STAKING_CONTRACT_CODE_ID"
echo "Oracle contract code: $ORACLE_CODE_ID"
echo "Treasury contract code: $TREASURY_CODE_ID"
echo ""

# Contracts initialization

echo "Init treasury contract"
INIT={\"trader\":\"$OSMOSIS_TRADER\",\"allowed_swap_routes\":[[{\"pool_id\":1,\"token_in_denom\":\"$NATIVE_TOKEN_DENOM\",\"token_out_denom\":\"uosmo\"}]]} \
TREASURY_CONTRACT=$(init_contract "$TREASURY_CODE_ID" $INIT "Treasury")

echo "Init staking contract"
INIT={\"native_token_denom\":\"$NATIVE_TOKEN_DENOM\",\"liquid_stake_token_denom\":\"milkTIA\",\"treasury_address\":\"$TREASURY_CONTRACT\",\"monitors\":[\"$OSMOSIS_ACCOUNT\"],\"validators\":[\"$CELESTIA_VALIDATOR_1\"],\"batch_period\":60,\"unbonding_period\":$UNBONDING_PERIOD,\"protocol_fee_config\":{\"dao_treasury_fee\":\"10\"},\"multisig_address_config\":{\"staker_address\":\"$CELESTIA_STAKER\",\"reward_collector_address\":\"$CELESTIA_REWARDS_COLLECTOR\"},\"minimum_liquid_stake_amount\":\"100\",\"ibc_channel_id\":\"channel-0\",\"send_fees_to_treasury\":true}
STAKE_CONTRACT=$(init_contract $STAKING_CONTRACT_CODE_ID $INIT "Staking")

# Init our oracle contract
echo "Init oracle contract"
INIT={\"admin_address\":\"$STAKE_CONTRACT\"}
ORACLE_CONTRACT=$(init_contract $ORACLE_CODE_ID $INIT "Oracle")

# Start the staking contract
echo "Starting the staking contract..."
INIT={\"update_config\":{\"oracle_address\":\"$ORACLE_CONTRACT\"}}
wait_tx osmosisd tx wasm execute $STAKE_CONTRACT $INIT \
    --from test_master --keyring-backend test \
    $OSMOSIS_TX_PARAMS

wait_tx osmosisd tx wasm execute $STAKE_CONTRACT '{"resume_contract":{"total_native_token":"0","total_liquid_stake_token":"0","total_reward_amount":"0"}}' \
    --from test_master --keyring-backend test \
    $OSMOSIS_TX_PARAMS

echo ""
echo "Contracts initialized!"
echo "Staking contract address: $STAKE_CONTRACT"
echo "Oracle contract address: $ORACLE_CONTRACT"
echo "Treasury contract address: $TREASURY_CONTRACT"
echo ""

