#!/bin/bash
set -e

# always returns true so set -e doesn't exit if it is not running.
killall celestia-appd || true

# make four celestia directories
mkdir -p $HOME/.celestia-app
mkdir $HOME/.celestia-app/validator1
mkdir $HOME/.celestia-app/validator2
mkdir $HOME/.celestia-app/validator3

# init all three validators
celestia-appd init --chain-id=celestia-dev-1 validator1 --home=$HOME/.celestia-app/validator1
celestia-appd init --chain-id=celestia-dev-1 validator2 --home=$HOME/.celestia-app/validator2
celestia-appd init --chain-id=celestia-dev-1 validator3 --home=$HOME/.celestia-app/validator3
# create keys for all three validators
celestia-appd keys add validator1 --keyring-backend=test --home=$HOME/.celestia-app/validator1
celestia-appd keys add validator2 --keyring-backend=test --home=$HOME/.celestia-app/validator2
celestia-appd keys add validator3 --keyring-backend=test --home=$HOME/.celestia-app/validator3

# update_genesis () {    
#     cat $HOME/.celestia-app/validator1/config/genesis.json | jq "$1" > $HOME/.celestia-app/validator1/config/tmp_genesis.json && mv $HOME/.celestia-app/validator1/config/tmp_genesis.json $HOME/.celestia-app/validator1/config/genesis.json
# }

# create validator node with tokens to transfer to the three other nodes
celestia-appd add-genesis-account $(celestia-appd keys show validator1 -a --keyring-backend=test --home=$HOME/.celestia-app/validator1) 100000000000utia --home=$HOME/.celestia-app/validator1
celestia-appd gentx validator1 10000000000utia --keyring-backend=test --home=$HOME/.celestia-app/validator1 --chain-id=celestia-dev-1
celestia-appd collect-gentxs --home=$HOME/.celestia-app/validator1

# port key (osmosis uses default ports)
# validator1 1314, 9084, 9085, 26661, 26661, 26660, 6060
# validator2 1316, 9088, 9089, 26655, 26654, 26653, 6061
# validator3 1315, 9086, 9087, 26652, 26651, 26650, 6062

# change app.toml values
VALIDATOR1_APP_TOML=$HOME/.celestia-app/validator1/config/app.toml
VALIDATOR2_APP_TOML=$HOME/.celestia-app/validator2/config/app.toml
VALIDATOR3_APP_TOML=$HOME/.celestia-app/validator3/config/app.toml

# validator1
sed -i -E 's|tcp://0.0.0.0:1317|tcp://0.0.0.0:1314|g' $VALIDATOR1_APP_TOML
sed -i -E 's|0.0.0.0:9090|0.0.0.0:9084|g' $VALIDATOR1_APP_TOML
sed -i -E 's|0.0.0.0:9091|0.0.0.0:9085|g' $VALIDATOR1_APP_TOML
sed -i'.bak' 's#"null"#"kv"#g' $VALIDATOR1_APP_TOML

# validator2
sed -i -E 's|tcp://0.0.0.0:1317|tcp://0.0.0.0:1316|g' $VALIDATOR2_APP_TOML
sed -i -E 's|0.0.0.0:9090|0.0.0.0:9088|g' $VALIDATOR2_APP_TOML
sed -i -E 's|0.0.0.0:9091|0.0.0.0:9089|g' $VALIDATOR2_APP_TOML

# validator3
sed -i -E 's|tcp://0.0.0.0:1317|tcp://0.0.0.0:1315|g' $VALIDATOR3_APP_TOML
sed -i -E 's|0.0.0.0:9090|0.0.0.0:9086|g' $VALIDATOR3_APP_TOML
sed -i -E 's|0.0.0.0:9091|0.0.0.0:9087|g' $VALIDATOR3_APP_TOML

# change config.toml values
VALIDATOR1_CONFIG=$HOME/.celestia-app/validator1/config/config.toml
VALIDATOR2_CONFIG=$HOME/.celestia-app/validator2/config/config.toml
VALIDATOR3_CONFIG=$HOME/.celestia-app/validator3/config/config.toml

# validator1
sed -i -E 's|tcp://127.0.0.1:26658|tcp://0.0.0.0:26662|g' $VALIDATOR1_CONFIG
sed -i -E 's|tcp://127.0.0.1:26657|tcp://0.0.0.0:26661|g' $VALIDATOR1_CONFIG
sed -i -E 's|tcp://0.0.0.0:26656|tcp://0.0.0.0:26660|g' $VALIDATOR1_CONFIG
sed -i -E 's|allow_duplicate_ip = false|allow_duplicate_ip = true|g' $VALIDATOR1_CONFIG
# validator2
sed -i -E 's|tcp://127.0.0.1:26658|tcp://0.0.0.0:26655|g' $VALIDATOR2_CONFIG
sed -i -E 's|tcp://127.0.0.1:26657|tcp://0.0.0.0:26654|g' $VALIDATOR2_CONFIG
sed -i -E 's|tcp://0.0.0.0:26656|tcp://0.0.0.0:26653|g' $VALIDATOR2_CONFIG
sed -i -E 's|allow_duplicate_ip = false|allow_duplicate_ip = true|g' $VALIDATOR2_CONFIG
# validator3
sed -i -E 's|tcp://127.0.0.1:26658|tcp://0.0.0.0:26652|g' $VALIDATOR3_CONFIG
sed -i -E 's|tcp://127.0.0.1:26657|tcp://0.0.0.0:26651|g' $VALIDATOR3_CONFIG
sed -i -E 's|tcp://0.0.0.0:26656|tcp://0.0.0.0:26650|g' $VALIDATOR3_CONFIG
sed -i -E 's|allow_duplicate_ip = false|allow_duplicate_ip = true|g' $VALIDATOR3_CONFIG


# copy validator1 genesis file to validator2-3
cp $HOME/.celestia-app/validator1/config/genesis.json $HOME/.celestia-app/validator2/config/genesis.json
cp $HOME/.celestia-app/validator1/config/genesis.json $HOME/.celestia-app/validator3/config/genesis.json


# copy tendermint node id of validator1 to persistent peers of validator2-3
sed -i -E "s|persistent_peers = \"\"|persistent_peers = \"$(celestia-appd tendermint show-node-id --home=$HOME/.celestia-app/validator1)@localhost:26660\"|g" $HOME/.celestia-app/validator2/config/config.toml
sed -i -E "s|persistent_peers = \"\"|persistent_peers = \"$(celestia-appd tendermint show-node-id --home=$HOME/.celestia-app/validator1)@localhost:26660\"|g" $HOME/.celestia-app/validator3/config/config.toml

# start all three validators
tmux new -s celestia1 -d celestia-appd start --home=$HOME/.celestia-app/validator1
tmux new -s celestia2 -d celestia-appd start --home=$HOME/.celestia-app/validator2
tmux new -s celestia3 -d celestia-appd start --home=$HOME/.celestia-app/validator3


# send utia from first validator to second validator
echo "Waiting 10 seconds to send funds to validators 2 and 3..."
sh ./check-node-running.sh celestia1
sh ./check-node-running.sh celestia2
sh ./check-node-running.sh celestia3
celestia-appd tx bank send validator1 $(celestia-appd keys show validator2 -a --keyring-backend=test --home=$HOME/.celestia-app/validator2) 100000000utia --keyring-backend=test --home=$HOME/.celestia-app/validator1 --chain-id=celestia-dev-1 --broadcast-mode block --node http://localhost:26661 --yes --fees 21000utia
celestia-appd tx bank send validator1 $(celestia-appd keys show validator3 -a --keyring-backend=test --home=$HOME/.celestia-app/validator3) 100000000utia --keyring-backend=test --home=$HOME/.celestia-app/validator1 --chain-id=celestia-dev-1 --broadcast-mode block --node http://localhost:26661 --yes --fees 21000utia

# create second & third validator
celestia-appd tx staking create-validator --amount=500000000utia --from=validator2 --pubkey=$(celestia-appd tendermint show-validator --home=$HOME/.celestia-app/validator2) --moniker="validator2" --chain-id="celestia-dev-1" --commission-rate="0.1" --commission-max-rate="0.2" --commission-max-change-rate="0.05" --min-self-delegation="500000000" --keyring-backend=test --home=$HOME/.celestia-app/validator2 --broadcast-mode block --node http://localhost:26661 --yes --fees 21000utia
celestia-appd tx staking create-validator --amount=400000000utia --from=validator3 --pubkey=$(celestia-appd tendermint show-validator --home=$HOME/.celestia-app/validator3) --moniker="validator3" --chain-id="celestia-dev-1" --commission-rate="0.1" --commission-max-rate="0.2" --commission-max-change-rate="0.05" --min-self-delegation="400000000" --keyring-backend=test --home=$HOME/.celestia-app/validator3 --broadcast-mode block --node http://localhost:26661 --yes --fees 21000utia

echo "All 3 Validators are up and running!"