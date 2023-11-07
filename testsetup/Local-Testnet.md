# Local Osmosis - Celestia Testnet

## Use Docker as a simple setup with an isolated environment

First build the image, this will take a while and leaves you with a setup testnet for both networks and a setup relayer

```
docker build ./testsetup --tag mw-testnet
```

Now you can always boot into the clean config

```
docker run --name mw-testnet -d -p 26661:26661/udp -p 26657:26657/udp -p 26661:26661/tcp -p 26657:26657/tcp docker.io/library/mw-testnet
```

Test accounts are funded, check out `./local-accounts.sh`
But you need to import the mnemonic:

```
boy view flame close solar robust crunch slot govern false jungle dirt blade minor shield bounce rent expand anxiety busy pull inject grace require
```

```
osmosisd keys add test_master --recover
celestia-appd keys add test_master --recover
```

Now you can deploy the contract:

```
sh ./init_stake_contract.sh
```

After this you can liquid stake (currently you need to wait a couple of seconds after ibc transfer TIA to Osmosis):

```
sh ./liquid_stake.sh
```

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
pr `cargo install ibc-relayer-cli --bin hermes --locked`

```

## Start Networks

This will launch a 3 node Osmosis testnet and a 1 node Celestia testnet.
This will create keys for the validators and fund the validators.

```
sh ./local-celestia-testnet-new.sh
sh ./local-osmosis-testnet-new.sh
```

Notes:

- This adjusts the Celestia ports to not conflict with the Osmosis node ports.
- This enables Tx indexing in Celestia so tooling can confirm transactions done on the network.

## Start Relayer

Create a key for the relayer on Celestia side and store in in a file celestia-relayer-key.json

```
celestia-appd keys add relayer --output=json > celestia-relayer-key.json
```

Create a key for the relayer on Osmosis side and store in in a file osmosis-relayer-key.json

```
osmosisd keys add relayer --output=json > osmosis-relayer-key.json
```

Import the Hermes config locally

```
mkdir ~/.hermes
cp ./hermes-config.toml ~/.hermes/config.toml
```

Import the keys into Hermes

```
hermes keys add --chain osmosis-dev-1 --key-file './osmosis-relayer-key.json'
hermes keys add --chain celestia-dev-1 --key-file './celestia-relayer-key.json'
```

Fund the accounts, replace the addresses with the addresses from the key files
// osmosis local testnet uses stake for fees

```
osmosisd tx bank send validator1 $OSMOSIS_ADDR 50000000stake --keyring-backend=test --home=$HOME/.osmosisd/validator1 --chain-id osmosis-dev-1
celestia-appd tx bank send validator $CELESTIA_ADDR 5000000000utia --node http://127.0.0.1:26661 --fees 21000utia
```

Create the connection between the local chains

```
hermes create client --host-chain celestia-dev-1 --reference-chain osmosis-dev-1
hermes create client --host-chain osmosis-dev-1 --reference-chain celestia-dev-1
hermes create connection --a-chain celestia-dev-1 --b-chain osmosis-dev-1
hermes create channel --a-chain celestia-dev-1 --a-connection connection-0 --a-port transfer --b-port transfer
```

Start Hermes (subsequently you only need to run this command)

```
tmux new -s hermes -d hermes start
```

### Check Validator output

You can the output of the validators with Tmux

```
tmux a -t celestia1
tmux a -t osmosis1
tmux a -t osmosis2
tmux a -t osmosis3
tmux a -t hermes
```

To leave Tmux `Ctrl+B, D`

## Start Again

To start the network again after setting it up run:

```
sh ./local-osmosis-testnet-continue.sh
sh ./local-celestia-testnet-continue.sh
tmux new -s hermes -d hermes start
```
