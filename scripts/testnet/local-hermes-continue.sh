tmux new -s hermes -d hermes start

# start a loop to clear packets
tmux new -s hermes-clear-lopp -d sh ./local-hermes-clear-packets.sh