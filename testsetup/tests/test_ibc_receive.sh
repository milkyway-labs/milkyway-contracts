# utils
function test_failure() {
    while [ "$(osmosisd query txs --events "recv_packet.packet_sequence=$PACKET_SEQUENCE" --output json | jq -r '.count')" = "0" ]; do    
        sleep 1
    done
    osmosisd query txs --events "recv_packet.packet_sequence=$PACKET_SEQUENCE" --output json | jq -r '.txs[-1].raw_log | fromjson | .[].events[] | select(.type == "write_acknowledgement") | .attributes[] | select(.key == "packet_ack") | .value | fromjson.error' | grep -Eq  "^ABCI code:"
}
function test_success() {
    test_failure && false || true
}

# find existing contract
CODE_ID=$(osmosisd query wasm list-code --output json | jq -r '.code_infos[-1].code_id')
CONTRACT=$(osmosisd query wasm list-contract-by-code $CODE_ID --output json | jq -r '.contracts[-1]')

ADMIN_OSMOSIS=osmo1sfhy3emrgp26wnzuu64p06kpkxd9phel8ym0ge
ADMIN_CELESTIA=celestia1sfhy3emrgp26wnzuu64p06kpkxd9phel74e0yx

# send tokens
MEMO='{"wasm":{"contract":"'$CONTRACT'","msg":{"receive_rewards":{}}}}'
PACKET_SEQUENCE=$(celestia-appd tx ibc-transfer transfer transfer channel-0 --from test_master --node http://localhost:26661 --chain-id celestia-dev-1 --fees 21000utia --output json -y $CONTRACT 10utia  --broadcast-mode block --memo "$MEMO" | jq -r '.raw_log | fromjson | .[0].events[] | select(.type == "send_packet") | .attributes[] | select(.key == "packet_sequence") | .value')
test_success && echo "success" || echo "failure"

# check logs (in docker)
tmux capture-pane -pS -1000 -t osmosis1 | grep -A 2 "error-context"

# check updates
osmosisd query txs --events "recv_packet.packet_sequence=$PACKET_SEQUENCE" --output json | jq -r '.txs[-1].raw_log'
osmosisd query bank balances $CONTRACT
celestia-appd query bank balances $ADMIN_CELESTIA --node http://localhost:26661

# stake via IBC
MEMO='{"wasm":{"contract":"'$CONTRACT'","msg":{"liquid_stake":{"original_sender":"'$ADMIN_CELESTIA'"}}}}'
PACKET_SEQUENCE=$(celestia-appd tx ibc-transfer transfer transfer channel-0 --from test_master --node http://localhost:26661 --chain-id celestia-dev-1 --fees 21000utia --output json -y $CONTRACT 1000utia  --broadcast-mode block --memo "$MEMO" | jq -r '.raw_log | fromjson | .[0].events[] | select(.type == "send_packet") | .attributes[] | select(.key == "packet_sequence") | .value')
test_success && echo "success" || echo "failure"