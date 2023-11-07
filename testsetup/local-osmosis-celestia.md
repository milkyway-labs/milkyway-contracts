# Setup Testing Environment for Osmosis and Celestia Local Chains

## Install Dependencies

```bash
# Install a terminal multiplexer; `tmux`
brew install tmux

# Move to the directory where you'd like to set up your testing environment
# Build an executable file `osmosisd`
git clone https://github.com/osmosis-labs/osmosis.git
cd osmosis
make install

# Build an executable file `celestia-appd`
cd ..
git clone https://github.com/celestiaorg/celestia-app.git
cd celestia-app
make install

# Build an executable file `hermes`
cd ..
git clone https://github.com/informalsystems/hermes
cd hermes
make install

# Install ibc-relayer-cli
pr `cargo install ibc-relayer-cli --bin hermes --locked`
```

## Start Networks

By running the following scripts, you will set up nodes for Osmosis and Celestia local networks.

```bash
# The following scripts adjust the Celestia ports to not conflict with the Osmosis node ports.
#
# Osmosis uses default port numbers
# 1317 (api), 26657(rpc), 26656(p2p), 9090(grpc), 9091(grpc-web)
#
# Celestia uses custom port numbers
# 1340 (api), 26651(rpc), 26660(p2p), 9190(grpc), 9191(grpc-web)
#

# Initializes a single local chain for Osmosis and run it in your local machine
# It enables Tx indexing in Celestia so tooling can confirm transactions done on the network.
#
# Validator address: osmo1zaavvzxez0elundtn32qnk9lkm8kmcsz2tlhe7
sh ./local-osmosis-node.sh

# Initializes a single local chain for Celestia and run it in your local machine
#
# Validator address: celestia1zaavvzxez0elundtn32qnk9lkm8kmcszn6ah4p
sh ./local-celestia-node.sh

# Verify that the chains are running well
# Osmosis: http://localhost:26667/status?
# Celestia: http://localhost:26661/status?
```

## Start Relayer

Recover the relayer key on Celestia from the `osmosis-relayer-key.json` file.

```bash
MNEMONIC="glimpse forward syrup road draft foot bottom rug gadget blind pyramid law discover cage buyer recycle ginger patch defense solid whisper jeans appear receive"
echo $MNEMONIC | osmosisd keys add relayer --recover
```

Recover the relayer key on Celestia from the `celestia-relayer-key.json` file.

```bash
# If you encounter `Error: aborted`, then delete the key name `relayer`
MNEMONIC="vivid hello oxygen clean want jewel scheme illegal hand sentence test orbit shrimp between van inmate vocal raw chaos capable volume traffic husband dial"
echo $MNEMONIC | celestia-appd keys add relayer --recover
```

Import the hermes config

```bash
mkdir ~/.hermes
cp ./hermes-config.toml ~/.hermes/config.toml
```

Import the keys into hermes

```bash
# This command should output `SUCCESS Added key 'wallet' (YOUR_ADDRESS) on chain osmosis-dev-1
hermes keys add --chain osmosis-dev-1 --key-file ./osmosis-relayer-key.json

# This command should output SUCCESS Added key 'wallet' (YOUR_ADDRESS) on chain celestia-dev-1
hermes keys add --chain celestia-dev-1 --key-file ./celestia-relayer-key.json
```

Let's fund the accounts

```bash
OSMOSIS_ADDR="osmo1wsjas75qy0gt8yarwpzs8z4qhckzs5627y7402"
CELESTIA_ADDR="celestia100l7jp5nu3m5pgeg5t5rkyx8slvl69p864kl8h"

# Send funds to the relayer account
osmosisd tx bank send validator $OSMOSIS_ADDR 5000000000stake \
--chain-id osmosis-dev-1 \
--keyring-backend=test \
--yes \
--output json | jq

# Query balances
osmosisd q bank balances $OSMOSIS_ADDR -o json | jq

# Send funds to the relayer account
celestia-appd tx bank send validator $CELESTIA_ADDR 5000000000utia \
--chain-id celestia-dev-1 \
--keyring-backend=test \
--node http://127.0.0.1:26661 \
--fees 21000utia \
--yes \
--output json | jq

# Query balances
celestia-appd q bank balances $CELESTIA_ADDR --node http://127.0.0.1:26661 -o json | jq
```

Create the connection between the local chains

```bash
hermes keys add --chain osmosis-dev-1 --key-file 'osmosis-validator-key.json'
hermes keys add --chain celestia-dev-1 --key-file 'celestia-validator-key.json'

# Create a new IBC client
hermes create client --host-chain celestia-dev-1 --reference-chain osmosis-dev-1
hermes create client --host-chain osmosis-dev-1 --reference-chain celestia-dev-1

# Create a new connection between two chains
hermes create connection --a-chain celestia-dev-1 --b-chain osmosis-dev-1

# Create a new channel between two chains
hermes create channel --a-chain celestia-dev-1 --a-connection connection-0 --a-port transfer --b-port transfer
```

Start Hermes (subsequently you only need to run this command)

```bash
tmux new -s hermes -d hermes start
```

### Check Validator output

You can the output of the validators with Tmux

```bash
#
# To leave tmux, `Ctrl+B, D`
#

# Celestia Validator
tmux a -t celestiavalidator

# Osmosis Validator
tmux a -t osmosisvalidator

# Hermes
tmux a -t hermes
```
