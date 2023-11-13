#!/bin/bash
set -e

# always returns true so set -e doesn't exit if it is not running.
killall osmosisd || true

# make four osmosis directories
mkdir -p $HOME/.osmosisd
mkdir $HOME/.osmosisd/validator1

# init validator
osmosisd init --chain-id=osmosis-dev-1 validator1 --home=$HOME/.osmosisd/validator1
osmosisd keys add validator1 --keyring-backend=test --home=$HOME/.osmosisd/validator1

update_genesis () {    
    cat $HOME/.osmosisd/validator1/config/genesis.json | jq "$1" > $HOME/.osmosisd/validator1/config/tmp_genesis.json && mv $HOME/.osmosisd/validator1/config/tmp_genesis.json $HOME/.osmosisd/validator1/config/genesis.json
}

# change staking denom to uosmo
update_genesis '.app_state["staking"]["params"]["bond_denom"]="uosmo"'

sed -i -E 's|tcp://127.0.0.1:26657|tcp://0.0.0.0:26657|g' $HOME/.osmosisd/validator1/config/config.toml

# create validator node with tokens
osmosisd add-genesis-account $(osmosisd keys show validator1 -a --keyring-backend=test --home=$HOME/.osmosisd/validator1) 100000000000uosmo,100000000000stake --home=$HOME/.osmosisd/validator1
osmosisd gentx validator1 500000000uosmo --keyring-backend=test --home=$HOME/.osmosisd/validator1 --chain-id=osmosis-dev-1
osmosisd collect-gentxs --home=$HOME/.osmosisd/validator1

# change config.toml values
sed -i -E 's|allow_duplicate_ip = false|allow_duplicate_ip = true|g' $HOME/.osmosisd/validator1/config/config.toml

# start all three validators
tmux new -s osmosis1 -d osmosisd start --home=$HOME/.osmosisd/validator1
sh ./check-node-running.sh osmosis1

echo "Osmosis validator is up and running!"