#!/bin/sh

# Stop script execution if an error is encountered
set -o errexit
# Stop script execution if an undefined variable is used
set -o nounset

killall celestia-appd || true

# Start celestia-app
echo "Starting celestia-app..."
tmux new -s celestia1 -d celestia-appd start --home=$HOME/.celestia-app/validator1
tmux new -s celestia2 -d celestia-appd start --home=$HOME/.celestia-app/validator2
tmux new -s celestia3 -d celestia-appd start --home=$HOME/.celestia-app/validator3