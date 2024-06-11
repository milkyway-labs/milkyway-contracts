#!/bin/bash

SCRIPT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)
source "$SCRIPT_DIR/params.sh"
source "$SCRIPT_DIR/../utils/tx.sh"

# Add bins to the path
BINS_DIR=$SCRIPT_DIR/../bins
PATH=$BINS_DIR:$PATH

echo "Funding $OSMOSIS_ACCOUNT"
wait_tx osmosisd tx bank send validator1 $OSMOSIS_ACCOUNT 50000000stake \
  $OSMOSIS_TX_PARAMS \
  --home=$HOME/.osmosisd/validator1 -y
wait_tx osmosisd tx bank send validator1 $OSMOSIS_ACCOUNT 50000000uosmo \
  $OSMOSIS_TX_PARAMS \
  --home=$HOME/.osmosisd/validator1 -y

# Funds Celestia accounts
echo "Funding $CELESTIA_STAKER (staker)"
wait_tx celestia-appd tx bank send validator1 $CELESTIA_STAKER 500000000utia \
  $CELESTIA_TX_PARAMS \
  --home=$HOME/.celestia-app/validator1 -y
echo "Funding $CELESTIA_GRANTEE (grantee)"
wait_tx celestia-appd tx bank send validator1 $CELESTIA_GRANTEE 10000000utia \
  $CELESTIA_TX_PARAMS \
  --home=$HOME/.celestia-app/validator1 -y
echo "Funding $CELESTIA_REWARDS_COLLECTOR (rewards_collector)"
wait_tx celestia-appd tx bank send validator1 $CELESTIA_REWARDS_COLLECTOR 1000000utia \
  $CELESTIA_TX_PARAMS \
  --home=$HOME/.celestia-app/validator1 -y

# Init Celestia accounts
echo "Adding accounts keys to Celestia"
echo $TESTNET_MNEMONIC | celestia-appd keys add staker --account $CELESTIA_STAKER_ACCOUNT_INDEX --recover
echo $TESTNET_MNEMONIC | celestia-appd keys add grantee --account $CELESTIA_GRANTEE_ACCOUNT_INDEX --recover
echo $TESTNET_MNEMONIC | celestia-appd keys add rewards_collector --account $CELESTIA_REWARDS_COLLECTOR_ACCOUNT_INDEX --recover

# Set rewards collector address
echo "Setting rewards collector address"
wait_tx celestia-appd tx distribution set-withdraw-addr $CELESTIA_REWARDS_COLLECTOR --from staker \
  $CELESTIA_TX_PARAMS

# Grant grants to the grantee
echo "Granting grants from staker to grantee"
wait_tx celestia-appd tx authz grant $CELESTIA_GRANTEE generic --msg-type=/cosmos.staking.v1beta1.MsgDelegate --from staker \
  $CELESTIA_TX_PARAMS
wait_tx celestia-appd tx authz grant $CELESTIA_GRANTEE generic --msg-type=/cosmos.staking.v1beta1.MsgUndelegate --from staker \
  $CELESTIA_TX_PARAMS
wait_tx celestia-appd tx authz grant $CELESTIA_GRANTEE generic --msg-type=/cosmos.staking.v1beta1.MsgBeginRedelegate --from staker \
  $CELESTIA_TX_PARAMS
wait_tx celestia-appd tx authz grant $CELESTIA_GRANTEE generic --msg-type=/cosmos.distribution.v1beta1.MsgWithdrawDelegatorReward --from staker \
  $CELESTIA_TX_PARAMS
wait_tx celestia-appd tx authz grant $CELESTIA_GRANTEE generic --msg-type=/ibc.applications.transfer.v1.MsgTransfer --from staker \
  $CELESTIA_TX_PARAMS
wait_tx celestia-appd tx authz grant $CELESTIA_GRANTEE generic --msg-type=/ibc.applications.transfer.v1.MsgTransfer --from rewards_collector \
  $CELESTIA_TX_PARAMS

# Grant fee grant
echo "Granting fee grant from staker to grantee"
wait_tx celestia-appd tx feegrant grant $CELESTIA_STAKER $CELESTIA_GRANTEE \
  --period 120 --period-limit 500000utia \
  --allowed-messages "/cosmos.authz.v1beta1.MsgExec,/ibc.applications.transfer.v1.MsgTransfer" \
  $CELESTIA_TX_PARAMS

