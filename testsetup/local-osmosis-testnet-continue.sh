#!/bin/bash
set -e

# always returns true so set -e doesn't exit if it is not running.
killall osmosisd || true

# start all three validators
tmux new -s osmosisvalidator1 -d osmosisd start --home=$HOME/.osmosisd/validator1 --rpc.laddr tcp://0.0.0.0:26657
tmux new -s osmosisvalidator2 -d osmosisd start --home=$HOME/.osmosisd/validator2
tmux new -s osmosisvalidator3 -d osmosisd start --home=$HOME/.osmosisd/validator3
