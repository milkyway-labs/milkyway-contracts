sh ./local-celestia-testnet-multi-continue.sh
sh ./local-osmosis-testnet-continue.sh
sh ./tia-osmo-pool.sh
tmux new -s clear -d sh ./local-hermes-clear-packets.sh
sh ./local-hermes-continue.sh