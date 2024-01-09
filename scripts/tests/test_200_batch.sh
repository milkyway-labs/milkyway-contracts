#!/bin/zsh

function awaitTx() {
    while true; do
        TX=$(osmosisd q tx $TXHASH --chain-id osmosis-dev-1 --output json)
        if [ -z "$TX" ]; then
            echo "Waiting for tx $TXHASH"
            sleep 1
        else
            break
        fi
    done
}

# create 200 wallets in osmosis (just need to do once locally)
for i in {1..200}
do
  echo "Creating wallet $i"
  osmosisd keys add wallet$i --keyring-backend test --home ./osmosisd_test
done

# send stake for fees to each wallet
ADMIN_OSMOSIS=osmo1sfhy3emrgp26wnzuu64p06kpkxd9phel8ym0ge
ALL_ADDRESSES=()
for i in {1..200}
do
    ALL_ADDRESSES+=$(osmosisd keys show wallet$i --keyring-backend test --home ./osmosisd_test -a)
done
osmosisd tx bank multi-send $ADMIN_OSMOSIS "${ALL_ADDRESSES[@]}" 200000stake \
    --from test_master --keyring-backend test \
    -y --gas-prices 0.025stake --gas-adjustment 1.7 --gas auto  \
    --chain-id osmosis-dev-1 --broadcast-mode sync

# ibc send 100000 utia to each wallet from celestia
# need to do in docker currently
celestia-appd tx ibc-transfer transfer transfer channel-0 --from validator1 --keyring-backend test  --home=$HOME/.celestia-app/validator1 \
    --node http://localhost:26661 --chain-id celestia-dev-1 --fees 21000utia --output json -y \
    $ADMIN_OSMOSIS 20000000utia --broadcast-mode sync --output json

NATIVE_TOKEN_DENOM="ibc/C3E53D20BC7A4CC993B17C7971F8ECD06A433C10B6A96F4C4C3714F0624C56DA"
while [ -z "$NATIVE_TOKEN_DENOM" ]; do
    BALANCES=$(osmosisd query bank balances $ADMIN_OSMOSIS --output json)
    echo $BALANCES
    NATIVE_TOKEN_DENOM=$(echo $BALANCES | jq -r '.balances[].denom | select(. | contains("ibc/"))')
    sleep 3
done;
echo $NATIVE_TOKEN_DENOM

TXHASH=$(osmosisd tx bank multi-send $ADMIN_OSMOSIS "${ALL_ADDRESSES[@]}" 100000$NATIVE_TOKEN_DENOM \
    --from test_master --keyring-backend test \
    -y --gas-prices 0.025stake --gas-adjustment 1.7 --gas auto  \
    --chain-id osmosis-dev-1 --broadcast-mode sync --output json | jq -r '.txhash')
awaitTx

# STAKE_CONTRACT=osmo14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9sq2r9g9

# liquid stake 100000 utia from each wallet
for i in {1..200}
do
  echo "Liquid staking wallet $i"
# check if wallet has stake already
    BALANCES=$(osmosisd query bank balances $(osmosisd keys show wallet$i --keyring-backend test --home ./osmosisd_test -a) --output json)
    if [ $(echo $BALANCES | jq -r '.balances[].denom | select(. | contains("milkTIA"))') ]; then
        echo "Wallet $i already has stake"
        continue
    fi
  osmosisd tx wasm execute $STAKE_CONTRACT '{"liquid_stake":{}}' \
    --from wallet$i --keyring-backend test --home ./osmosisd_test \
    --amount 100000$NATIVE_TOKEN_DENOM \
    -y --gas-prices 0.025stake --gas-adjustment 1.7 --gas auto  \
    --broadcast-mode sync --chain-id osmosis-dev-1 \
    --generate-only > unsignedTx.json
  osmosisd tx sign unsignedTx.json \
    --from wallet$i --keyring-backend test --home ./osmosisd_test \
    --chain-id osmosis-dev-1 --output-document signedTx.json
  RES=$(osmosisd tx broadcast signedTx.json --output json)
  TXHASH=$(echo $RES | jq -r '.txhash')
  echo $TXHASH
# pause every 25
    if [ $(( $i % 25 )) -eq 0 ]; then
        echo "Pausing for 5 seconds"
        sleep 5
    fi
  SEQUENCE=$(expr $SEQUENCE + 1)
done

# unstake 10000 utia from each wallet
SEQUENCE=$(osmosisd query account $(osmosisd keys show test_master --keyring-backend test -a) \
    --chain-id osmosis-dev-1 --output json | jq -r '.sequence')
ACCOUNT_NUMBER=$(osmosisd query account $(osmosisd keys show test_master --keyring-backend test -a) \
    --chain-id osmosis-dev-1 --output json | jq -r '.account_number')
LS_DENOM=factory/$STAKE_CONTRACT/milkTIA
for i in {1..200}
do
    echo "Unstaking wallet $i"
# check if wallet has stake already
        BALANCES=$(osmosisd query bank balances $(osmosisd keys show wallet$i --keyring-backend test --home ./osmosisd_test -a) --output json)
        BALANCE=$(echo $BALANCES | jq -r '.balances[] | select(.denom | contains("milkTIA"))')
        if [ -z $BALANCE ]; then
            echo "Wallet $i has no stake"
            continue
        fi
# continue if staked token balance is not still 100000
        if [ -z $(echo $BALANCE | jq -r '.amount' | grep 100000) ]; then
            echo "Wallet $i has already unstaked"
            continue
        fi
    osmosisd tx wasm execute $STAKE_CONTRACT '{"liquid_unstake":{}}' \
        --amount 10000$LS_DENOM \
        --from wallet$i --keyring-backend test --home ./osmosisd_test \
        -y --gas-prices 0.025stake --gas-adjustment 1.7 --gas auto  \
        --broadcast-mode sync --chain-id osmosis-dev-1 \
        --generate-only > unsignedTx.json
    osmosisd tx sign unsignedTx.json \
        --from wallet$i --keyring-backend test --home ./osmosisd_test \
        --chain-id osmosis-dev-1 --account-number $ACCOUNT_NUMBER --sequence $SEQUENCE --output-document signedTx.json
    RES=$(osmosisd tx broadcast signedTx.json --output json)
    TXHASH=$(echo $RES | jq -r '.txhash')
    # awaitTx
    # RES=$(osmosisd query tx $TXHASH --output json)
    # GAS=$(echo $RES | jq -r '.gas_used')
    echo $TXHASH
    # echo $GAS
# pause every 25
        if [ $(( $i % 25 )) -eq 0 ]; then
            echo "Pausing for 5 seconds"
            sleep 5
        fi
    SEQUENCE=$(expr $SEQUENCE + 1)
done

# count requests in batch
osmosisd query wasm contract-state smart $STAKE_CONTRACT '{"batches":{}}' --output json | jq -r '.data.batches[0].unstake_request_count'
osmosisd query wasm contract-state smart $STAKE_CONTRACT '{"unstake_requests":{"user":"osmo1z0um7jczylh6n7mh5s26grtydf02d8254kmaf6"}}'

# check queue
osmosisd query wasm contract-state smart $STAKE_CONTRACT '{"ibc_queue":{}}' --output json | jq -r '.data.ibc_queue'

# execute batch
osmosisd tx wasm execute $STAKE_CONTRACT '{"submit_batch":{}}' \
  --from test_master --keyring-backend test \
  -y -b sync \
  --gas-prices 0.025stake --gas-adjustment 1.7 --gas auto  \
  --chain-id osmosis-dev-1

# receive rewards
MEMO='{"wasm":{"contract":"'$STAKE_CONTRACT'","msg":{"receive_unstaked_tokens":{"batch_id":1}}}}'
PACKET_SEQUENCE=$(celestia-appd tx ibc-transfer transfer transfer channel-0 --from test_master --node http://localhost:26661 --chain-id celestia-dev-1 --fees 21000utia --output json -y $STAKE_CONTRACT 10utia  --broadcast-mode block --memo "$MEMO" | jq -r '.raw_log | fromjson | .[0].events[] | select(.type == "send_packet") | .attributes[] | select(.key == "packet_sequence") | .value')

# withdraw tokens
osmosisd tx wasm execute $STAKE_CONTRACT '{"withdraw":{"batch_id":1}}' \
  --from wallet1 --keyring-backend test --home ./osmosisd_test \
  -y -b sync \
  --gas-prices 0.025stake --gas-adjustment 1.7 --gas auto  \
  --chain-id osmosis-dev-1