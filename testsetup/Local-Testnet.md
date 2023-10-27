# Local Osmosis - Celestia Testnet

## Install deps

```
brew install tmux

cd YOUR_DEV_FOLDER

git clone https://github.com/osmosis-labs/osmosis.git
cd osmosis
make install

cd ..
git clone https://github.com/celestiaorg/celestia-app.git
cd celestia-app
make install

cd ..
git clone https://github.com/informalsystems/hermes
cd hermes
make install
```

## Start Networks

This will launch a 3 node Osmosis testnet and a 1 node Celestia testnet.
This will create keys for the validators and fund the validators.

```
sh ./local-celestia-testnet.sh
sh ./local-osmosis-testnet.sh
```

Notes:

- This adjusts the Celestia ports to not conflict with the Osmosis node ports.
- This enables Tx indexing in Celestia so tooling can confirm transactions done on the network.

## Start Relayer

Create a key for the relayer on Celestia side and store in in a file celestia-relayer-key.json

```
celestia-appd keys add relayer --output=json
```

Create a key for the relayer on Osmosis side and store in in a file osmosis-relayer-key.json

```
osmosisd keys add relayer --output=json
```

Import the Hermes config locally

```
mkdir ~/.hermes
cp ./hermes-config.toml ~/.hermes/config.toml
```

Import the keys into Hermes

```
hermes keys add --chain testing --key-file './osmosis-relayer-key.json'
hermes keys add --chain private --key-file './celestia-relayer-key.json'
```

Fund the accounts, replace the addresses with the addresses from the key files
// osmosis local testnet uses stake for fees

```
osmosisd tx bank send validator1 OSMOSIS_ADDR 50000000stake --keyring-backend=test --home=$HOME/.osmosisd/validator1 --chain-id testing
celestia-appd tx bank send validator CELESTIA_ADDR 5000000000utia --node http://127.0.0.1:26661 --fees 21000utia
```

Create the connection between the local chains

```
- hermes keys add --chain testing --key-file 'osmosis-validator-key.json'
- hermes keys add --chain private --key-file 'celestia-validator-key.json'
- hermes create client --host-chain private --reference-chain testing
- hermes create client --host-chain testing --reference-chain private
- hermes create connection --a-chain private --b-chain testing
- hermes create channel --a-chain private --a-connection connection-0 --a-port transfer --b-port transfer
```

Start Hermes (subsequently you only need to run this command)

```
tmux -s hermes -d hermes start
```

### Check Validator output

You can the output of the validators with Tmux

```
tmux a -t celestiavalidator1
tmux a -t osmosisvalidator1
tmux a -t osmosisvalidator2
tmux a -t osmosisvalidator3
tmux a -t hermes
```
