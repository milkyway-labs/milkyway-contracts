#!/bin/bash
set -e

#
# This script initializes a single local Osmosis chain and run it in your local machine for testing.
#

# Always returns true so set -e doesn't exit if it is not running.
killall osmosisd || true

# Ask the user for confirmation before deleting the existing osmosis home
# directory.
read -p "Do you want to reset the network and wipe: $HOME/.osmosisd? [y/n] " response

MNEMONIC_VALIDATOR="guard cream sadness conduct invite crumble clock pudding hole grit liar hotel maid produce squeeze return argue turtle know drive eight casino maze host"

# Check the user's response
if [[ $response != "n" ]]; then
    echo "Deleting $HOME/.osmosisd..."

    rm -rf $HOME/.osmosisd/

    # make directories
    mkdir $HOME/.osmosisd

    # init chain
    osmosisd init --chain-id=osmosis-dev-1 validator

    # create keys for validator
    echo $MNEMONIC_VALIDATOR | osmosisd keys add validator --keyring-backend=test --recover

    update_genesis () {
        cat $HOME/.osmosisd/config/genesis.json | jq "$1" > $HOME/.osmosisd/config/tmp_genesis.json && mv $HOME/.osmosisd/config/tmp_genesis.json $HOME/.osmosisd/config/genesis.json
    }

    # change staking denom to uosmo
    update_genesis '.app_state["staking"]["params"]["bond_denom"]="uosmo"'

    # create validator node with tokens to transfer to the three other nodes
    osmosisd add-genesis-account $(osmosisd keys show validator -a --keyring-backend=test) 100000000000uosmo,100000000000stake
    osmosisd gentx validator 500000000uosmo --keyring-backend=test --chain-id=osmosis-dev-1
    osmosisd collect-gentxs

    # update staking genesis
    update_genesis '.app_state["staking"]["params"]["unbonding_time"]="1814400s"'

    # port key (validator uses default ports)
    # validator 1317, 9050, 9091, 26658, 26657, 26656, 6060, 26660

    # change config.toml values
    VALIDATOR_CONFIG=$HOME/.osmosisd/config/config.toml

    # validator
    sed -i -E 's|allow_duplicate_ip = false|allow_duplicate_ip = true|g' $VALIDATOR_CONFIG

    # start the validator
    tmux new -s osmosisvalidator -d osmosisd start

    echo "The validator is up and running!"
else
    # start the validator
    tmux new -s osmosisvalidator -d osmosisd start

    echo "The validator is up and running!"
fi

