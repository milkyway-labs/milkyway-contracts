# Local Osmosis - Celestia Testnet

## Use Docker as a simple setup with an isolated environment

First build the image, this will take a while and leaves you with a set up testnet for both networks and a setup relayer

```
docker build . --tag mw-testnet
```

Now you can always boot into the clean config

```
docker run --name mw-testnet -d -p 26661:26661/udp -p 26657:26657/udp -p 26661:26661/tcp -p 26657:26657/tcp -p 1317:1317 -p 1314:1314 -p 9090:9090 docker.io/library/mw-testnet
```

Upon creation, the following accounts will be funded:
- `osmo1sfhy3emrgp26wnzuu64p06kpkxd9phel8ym0ge`: Osmosis account used to deploy the contracts on the chain.
- `osmo1lh0u9sug6qh922gjpal3frwtacaums4s7lkyl9`: Osmosis trader account used to swap the fee collected by the treasury contract.
- `celestia1sfhy3emrgp26wnzuu64p06kpkxd9phel74e0yx`: Celestia account that stakes the TIA on the Celestia chain (staker).
- `celestia1lh0u9sug6qh922gjpal3frwtacaums4s8w5yn6`: Celestia account that operates the staker account through the `x/authz` module (grantee).
- `celestia12rzczckgh8fqq533t0xqhqrrzdk76du3dxrx9q`: Celestia account where the staking reward will be withdrawn.

NOTE: The `grantee` account will have all the on chain permissions to control the staker account.

Those accounts can be imported into a test keyring through the `utils/import-accounts-keys.sh` script.

After the network is ready, you can configure the clients by using the `utils/configure-localnet.sh` script.

## Deploy Contracts

Before deploying the contracts, you need to:

1. Import the accounts using the `utils/import-accounts-keys.sh` script.
2. Compile the contracts by navigating to the project root directory and running the `make optimize` command.
3. Download the oracle contract from [here](https://github.com/milkyway-labs/milkyway-oracle/releases) and store it in the `artifacts/` folder.

After completing the above steps, you can deploy the contracts by running:

```sh
./init-stake-contract.sh
```

This script will initialize all the contracts and print out their addresses.

Next, you can IBC transfer some TIA using the following command:

```sh
celestia-appd tx ibc-transfer transfer transfer channel-0 osmo1sfhy3emrgp26wnzuu64p06kpkxd9phel8ym0ge 1000000utia \
  --from staker --fees 21000utia -b block -y
```

Upon reception, you can liquid stake the received tokens using the following command:

```sh
osmosisd tx wasm execute osmo1suhgf5svhu4usrurvxzlgn54ksxmn8gljarjtxqnapv8kjnp4nrsll0sqv '{"liquid_stake":{}}' \
  --amount 1000000ibc/C3E53D20BC7A4CC993B17C7971F8ECD06A433C10B6A96F4C4C3714F0624C56DA \
  --from test_master
```
NOTE: All the application binaries are available in the `bins` folder and will be lazily downloaded on first use.

### Fast testnet to test unbonding

This will stop working after an hour as the bridge will be out of sync

```
cd scripts/testnet
docker build . --file Dockerfile.fast --tag mw-testnet:fast
docker run --name mw-testnet-fast -d -p 26661:26661/udp -p 26657:26657/udp -p 26661:26661/tcp -p 26657:26657/tcp -p 1317:1317 -p 1314:1314 -p 9090:9090 docker.io/library/mw-testnet:fast
```

Check Docker logs to see when the bridge is ready ("--- HERMES READY ---")

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
pr `cargo install ibc-relayer-cli --bin hermes --locked`

```

## Start Networks

This will launch a 1 node Osmosis testnet and a 3 node Celestia testnet.
This will create keys for the validators and fund the validators.

```
sh ./local-celestia-testnet-multi-new.sh
sh ./local-osmosis-testnet-new.sh
```

Notes:

- This adjusts the Celestia ports to not conflict with the Osmosis node ports.
- This enables Tx indexing in Celestia so tooling can confirm transactions done on the network.

## Start Relayer

Create a key for the relayer on Celestia side and store in a file celestia-relayer-key.json

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
tmux a -t hermes
```

To leave Tmux `Ctrl+B, D`

## Start Again

To start the network again after setting it up run:

```
sh ./local-osmosis-testnet-continue.sh
sh ./local-celestia-testnet-multi-continue.sh
tmux new -s hermes -d hermes start
```
