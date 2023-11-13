#!/bin/bash
set -e

# always returns true so set -e doesn't exit if it is not running.
killall osmosisd || true

# make four osmosis directories
mkdir -p $HOME/.osmosisd
mkdir $HOME/.osmosisd/validator1
mkdir $HOME/.osmosisd/validator2
mkdir $HOME/.osmosisd/validator3

# init all three validators
osmosisd init --chain-id=osmosis-dev-1 validator1 --home=$HOME/.osmosisd/validator1
osmosisd init --chain-id=osmosis-dev-1 validator2 --home=$HOME/.osmosisd/validator2
osmosisd init --chain-id=osmosis-dev-1 validator3 --home=$HOME/.osmosisd/validator3
# create keys for all three validators
osmosisd keys add validator1 --keyring-backend=test --home=$HOME/.osmosisd/validator1
osmosisd keys add validator2 --keyring-backend=test --home=$HOME/.osmosisd/validator2
osmosisd keys add validator3 --keyring-backend=test --home=$HOME/.osmosisd/validator3

update_genesis () {    
    cat $HOME/.osmosisd/validator1/config/genesis.json | jq "$1" > $HOME/.osmosisd/validator1/config/tmp_genesis.json && mv $HOME/.osmosisd/validator1/config/tmp_genesis.json $HOME/.osmosisd/validator1/config/genesis.json
}

# change staking denom to uosmo
update_genesis '.app_state["staking"]["params"]["bond_denom"]="uosmo"'

# create validator node with tokens to transfer to the three other nodes
osmosisd add-genesis-account $(osmosisd keys show validator1 -a --keyring-backend=test --home=$HOME/.osmosisd/validator1) 100000000000uosmo,100000000000stake --home=$HOME/.osmosisd/validator1
osmosisd gentx validator1 500000000uosmo --keyring-backend=test --home=$HOME/.osmosisd/validator1 --chain-id=osmosis-dev-1
osmosisd collect-gentxs --home=$HOME/.osmosisd/validator1


# port key (validator1 uses default ports)
# validator1 1317, 9090, 9091, 26658, 26657, 26656, 6060
# validator2 1316, 9088, 9089, 26655, 26654, 26653, 6061
# validator3 1315, 9086, 9087, 26652, 26651, 26650, 6062


# change app.toml values
VALIDATOR2_APP_TOML=$HOME/.osmosisd/validator2/config/app.toml
VALIDATOR3_APP_TOML=$HOME/.osmosisd/validator3/config/app.toml

# validator2
sed -i -E 's|tcp://0.0.0.0:1317|tcp://0.0.0.0:1316|g' $VALIDATOR2_APP_TOML
sed -i -E 's|0.0.0.0:9090|0.0.0.0:9088|g' $VALIDATOR2_APP_TOML
sed -i -E 's|0.0.0.0:9091|0.0.0.0:9089|g' $VALIDATOR2_APP_TOML

# validator3
sed -i -E 's|tcp://0.0.0.0:1317|tcp://0.0.0.0:1315|g' $VALIDATOR3_APP_TOML
sed -i -E 's|0.0.0.0:9090|0.0.0.0:9086|g' $VALIDATOR3_APP_TOML
sed -i -E 's|0.0.0.0:9091|0.0.0.0:9087|g' $VALIDATOR3_APP_TOML


# change config.toml values
VALIDATOR1_CONFIG=$HOME/.osmosisd/validator1/config/config.toml
VALIDATOR2_CONFIG=$HOME/.osmosisd/validator2/config/config.toml
VALIDATOR3_CONFIG=$HOME/.osmosisd/validator3/config/config.toml

# validator1
sed -i -E 's|allow_duplicate_ip = false|allow_duplicate_ip = true|g' $VALIDATOR1_CONFIG
# validator2
sed -i -E 's|tcp://127.0.0.1:26658|tcp://0.0.0.0:26655|g' $VALIDATOR2_CONFIG
sed -i -E 's|tcp://127.0.0.1:26657|tcp://0.0.0.0:26654|g' $VALIDATOR2_CONFIG
sed -i -E 's|tcp://0.0.0.0:26656|tcp://0.0.0.0:26653|g' $VALIDATOR2_CONFIG
sed -i -E 's|tcp://0.0.0.0:26656|tcp://0.0.0.0:26650|g' $VALIDATOR2_CONFIG
sed -i -E 's|allow_duplicate_ip = false|allow_duplicate_ip = true|g' $VALIDATOR2_CONFIG
# validator3
sed -i -E 's|tcp://127.0.0.1:26658|tcp://0.0.0.0:26652|g' $VALIDATOR3_CONFIG
sed -i -E 's|tcp://127.0.0.1:26657|tcp://0.0.0.0:26651|g' $VALIDATOR3_CONFIG
sed -i -E 's|tcp://0.0.0.0:26656|tcp://0.0.0.0:26650|g' $VALIDATOR3_CONFIG
sed -i -E 's|tcp://0.0.0.0:26656|tcp://0.0.0.0:26650|g' $VALIDATOR3_CONFIG
sed -i -E 's|allow_duplicate_ip = false|allow_duplicate_ip = true|g' $VALIDATOR3_CONFIG


# copy validator1 genesis file to validator2-3
cp $HOME/.osmosisd/validator1/config/genesis.json $HOME/.osmosisd/validator2/config/genesis.json
cp $HOME/.osmosisd/validator1/config/genesis.json $HOME/.osmosisd/validator3/config/genesis.json


# copy tendermint node id of validator1 to persistent peers of validator2-3
sed -i -E "s|persistent_peers = \"\"|persistent_peers = \"$(osmosisd tendermint show-node-id --home=$HOME/.osmosisd/validator1)@localhost:26656\"|g" $HOME/.osmosisd/validator2/config/config.toml
sed -i -E "s|persistent_peers = \"\"|persistent_peers = \"$(osmosisd tendermint show-node-id --home=$HOME/.osmosisd/validator1)@localhost:26656\"|g" $HOME/.osmosisd/validator3/config/config.toml

# start all three validators
tmux new -s osmosis1 -d osmosisd start --home=$HOME/.osmosisd/validator1
tmux new -s osmosis2 -d osmosisd start --home=$HOME/.osmosisd/validator2
tmux new -s osmosis3 -d osmosisd start --home=$HOME/.osmosisd/validator3


# send uosmo from first validator to second validator
echo "Waiting 10 seconds to send funds to validators 2 and 3..."
sh ./check-node-running.sh osmosis1
sh ./check-node-running.sh osmosis2
sh ./check-node-running.sh osmosis3
osmosisd tx bank send validator1 $(osmosisd keys show validator2 -a --keyring-backend=test --home=$HOME/.osmosisd/validator2) 500000000uosmo --keyring-backend=test --home=$HOME/.osmosisd/validator1 --chain-id=osmosis-dev-1 --broadcast-mode block --node http://localhost:26657 --yes --fees 875stake
osmosisd tx bank send validator1 $(osmosisd keys show validator2 -a --keyring-backend=test --home=$HOME/.osmosisd/validator2) 100000000stake --keyring-backend=test --home=$HOME/.osmosisd/validator1 --chain-id=osmosis-dev-1 --broadcast-mode block --node http://localhost:26657 --yes --fees 875stake
osmosisd tx bank send validator1 $(osmosisd keys show validator3 -a --keyring-backend=test --home=$HOME/.osmosisd/validator3) 400000000uosmo --keyring-backend=test --home=$HOME/.osmosisd/validator1 --chain-id=osmosis-dev-1 --broadcast-mode block --node http://localhost:26657 --yes --fees 875stake
osmosisd tx bank send validator1 $(osmosisd keys show validator3 -a --keyring-backend=test --home=$HOME/.osmosisd/validator3) 100000000stake --keyring-backend=test --home=$HOME/.osmosisd/validator1 --chain-id=osmosis-dev-1 --broadcast-mode block --node http://localhost:26657 --yes --fees 875stake

# create second & third validator
osmosisd tx staking create-validator --amount=500000000uosmo --from=validator2 --pubkey=$(osmosisd tendermint show-validator --home=$HOME/.osmosisd/validator2) --moniker="validator2" --chain-id="osmosis-dev-1" --commission-rate="0.1" --commission-max-rate="0.2" --commission-max-change-rate="0.05" --min-self-delegation="500000000" --keyring-backend=test --home=$HOME/.osmosisd/validator2 --broadcast-mode block --node http://localhost:26657 --yes --fees 875stake
osmosisd tx staking create-validator --amount=400000000uosmo --from=validator3 --pubkey=$(osmosisd tendermint show-validator --home=$HOME/.osmosisd/validator3) --moniker="validator3" --chain-id="osmosis-dev-1" --commission-rate="0.1" --commission-max-rate="0.2" --commission-max-change-rate="0.05" --min-self-delegation="400000000" --keyring-backend=test --home=$HOME/.osmosisd/validator3 --broadcast-mode block --node http://localhost:26657 --yes --fees 875stake

echo "All 3 Validators are up and running!"