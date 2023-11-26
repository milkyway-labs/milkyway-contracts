# utils
function test_failure() {
    while [ "$(osmosisd query txs --events "recv_packet.packet_sequence=$PACKET_SEQUENCE" --output json | jq -r '.count')" = "0" ]; do    
        sleep 1
    done
    if [ "$(osmosisd query txs --events "recv_packet.packet_sequence=$PACKET_SEQUENCE" --output json | jq -r '.txs[-1].code')" = "0" ]; then
        return $(osmosisd query txs --events "recv_packet.packet_sequence=$PACKET_SEQUENCE" --output json | jq -r '.txs[-1].raw_log' | grep -Eq  "ABCI code:")
    fi
    return 1
}
function test_success() {
    test_failure && false || true
}

# receive rewards
MEMO='{"wasm":{"contract":"'$CONTRACT'","msg":{"receive_rewards":{}}}}'
PACKET_SEQUENCE=$(celestia-appd tx ibc-transfer transfer transfer channel-0 --from test_master --keyring-backend test --node http://localhost:26661 --chain-id celestia-dev-1 --fees 21000utia --output json -y $CONTRACT 10utia  --broadcast-mode block --memo "$MEMO" | jq -r '.raw_log | fromjson | .[0].events[] | select(.type == "send_packet") | .attributes[] | select(.key == "packet_sequence") | .value')
test_success && echo "success" || echo "failure"

# receive unstaked tokens
MEMO='{"wasm":{"contract":"'$CONTRACT'","msg":{"receive_unstaked_tokens":{}}}}'
PACKET_SEQUENCE=$(celestia-appd tx ibc-transfer transfer transfer channel-0 --from test_master --keyring-backend test --node http://localhost:26661 --chain-id celestia-dev-1 --fees 21000utia --output json -y $CONTRACT 10utia  --broadcast-mode block --memo "$MEMO" | jq -r '.raw_log | fromjson | .[0].events[] | select(.type == "send_packet") | .attributes[] | select(.key == "packet_sequence") | .value')
test_success && echo "success" || echo "failure"

# test failure (fails because of wrong message)
MEMO='{"wasm":{"contract":"'$CONTRACT'","msg":{"submit_batch":{}}}}'
PACKET_SEQUENCE=$(celestia-appd tx ibc-transfer transfer transfer channel-0 --from test_master --keyring-backend test --node http://localhost:26661 --chain-id celestia-dev-1 --fees 21000utia --output json -y $CONTRACT 10utia  --broadcast-mode block --memo "$MEMO" | jq -r '.raw_log | fromjson | .[0].events[] | select(.type == "send_packet") | .attributes[] | select(.key == "packet_sequence") | .value')
test_failure && echo "success" || echo "failure"

# test failure (fails because of wrong sender)
MEMO='{"wasm":{"contract":"'$CONTRACT'","msg":{"receive_rewards":{}}}'
PACKET_SEQUENCE=$(celestia-appd tx ibc-transfer transfer transfer channel-0 --from validator --node http://localhost:26661 --chain-id celestia-dev-1 --fees 21000utia --output json -y $CONTRACT 10utia  --broadcast-mode block --memo "$MEMO" | jq -r '.raw_log | fromjson | .[0].events[] | select(.type == "send_packet") | .attributes[] | select(.key == "packet_sequence") | .value')
test_failure && echo "success" || echo "failure"

# test failure (fails because of wrong sender)
MEMO='{"wasm":{"contract":"'$CONTRACT'","msg":{"receive_unstaked_tokens":{}}}'
PACKET_SEQUENCE=$(celestia-appd tx ibc-transfer transfer transfer channel-0 --from test_master --keyring-backend test --node http://localhost:26661 --chain-id celestia-dev-1 --fees 21000utia --output json -y $CONTRACT 10utia  --broadcast-mode block --memo "$MEMO" | jq -r '.raw_log | fromjson | .[0].events[] | select(.type == "send_packet") | .attributes[] | select(.key == "packet_sequence") | .value')
test_failure && echo "success" || echo "failure"

# circuit breaker
MEMO='{"wasm":{"contract":"'$CONTRACT'","msg":{"circuit_breaker":{}}}'
PACKET_SEQUENCE=$(celestia-appd tx ibc-transfer transfer transfer channel-0 --from test_master --keyring-backend test --node http://localhost:26661 --chain-id celestia-dev-1 --fees 21000utia --output json -y $CONTRACT 10utia  --broadcast-mode block --memo "$MEMO" | jq -r '.raw_log | fromjson | .[0].events[] | select(.type == "send_packet") | .attributes[] | select(.key == "packet_sequence") | .value')
test_success && echo "success" || echo "failure"

MEMO='{"wasm":{"contract":"'$CONTRACT'","msg":{"receive_unstaked_tokens":{}}}}'
PACKET_SEQUENCE=$(celestia-appd tx ibc-transfer transfer transfer channel-0 --from test_master --keyring-backend test --node http://localhost:26661 --chain-id celestia-dev-1 --fees 21000utia --output json -y $CONTRACT 10utia  --broadcast-mode block --memo "$MEMO" | jq -r '.raw_log | fromjson | .[0].events[] | select(.type == "send_packet") | .attributes[] | select(.key == "packet_sequence") | .value')
test_failure && echo "success" || echo "failure"

MEMO='{"wasm":{"contract":"'$CONTRACT'","msg":{"resume_contract":{}}}}'
if [ "$(osmosisd tx wasm execute $CONTRACT $PAYLOAD --from test_master --keyring-backend test --chain-id osmosis-dev-1 -y --broadcast-mode block --fees 900stake --output json | jq -r '.code')" = "0" ]; then
    echo "success"
else
    echo "failure"
fi

MEMO='{"wasm":{"contract":"'$CONTRACT'","msg":{"receive_unstaked_tokens":{}}}}'
PACKET_SEQUENCE=$(celestia-appd tx ibc-transfer transfer transfer channel-0 --from test_master --keyring-backend test --node http://localhost:26661 --chain-id celestia-dev-1 --fees 21000utia --output json -y $CONTRACT 10utia  --broadcast-mode block --memo "$MEMO" | jq -r '.raw_log | fromjson | .[0].events[] | select(.type == "send_packet") | .attributes[] | select(.key == "packet_sequence") | .value')
test_success && echo "success" || echo "failure"
