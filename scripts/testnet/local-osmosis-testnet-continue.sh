#!/bin/bash

SCRIPT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)
BINS_DIR=$SCRIPT_DIR/bins
PATH=$BINS_DIR:$PATH

set -e

# always returns true so set -e doesn't exit if it is not running.
killall osmosisd || true

# start all three validators
tmux new -s osmosis1 -d osmosisd start --home="$HOME"/.osmosisd/validator1 --rpc.laddr tcp://0.0.0.0:26657
