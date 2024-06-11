#!/bin/bash

SCRIPT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)
source "$SCRIPT_DIR/params.sh"
source "$SCRIPT_DIR/../utils/tx.sh"

# Add bins to the path
BINS_DIR=$SCRIPT_DIR/../bins
PATH=$BINS_DIR:$PATH

wait_tx osmosisd tx bank send validator1 $OSMOSIS_ACCOUNT 50000000stake \
  $OSMOSIS_TX_PARAMS \
  --home=$HOME/.osmosisd/validator1 -y
wait_tx osmosisd tx bank send validator1 $OSMOSIS_ACCOUNT 50000000uosmo \
  $OSMOSIS_TX_PARAMS \
  --home=$HOME/.osmosisd/validator1 -y

wait_tx celestia-appd tx bank send validator1 $CELESTIA_ACCOUNT 500000000utia \
  $CELESTIA_TX_PARAMS \
  --home=$HOME/.celestia-app/validator1 -y
