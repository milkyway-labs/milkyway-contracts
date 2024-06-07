#!/bin/bash

SCRIPT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)
BINS_DIR=$SCRIPT_DIR/../bins
PATH=$BINS_DIR:$PATH

tmux new -s hermes -d hermes start
