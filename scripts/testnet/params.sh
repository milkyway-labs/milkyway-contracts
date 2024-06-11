# Osmosis config
OSMOSIS_NODE="http://localhost:26657"
OSMOSIS_CHAIN_ID="osmosis-dev-1"
OSMOSIS_GAS_PRICE="0.025stake"
OSMOSIS_GAS_ADJUSTMENT="1.7"
OSMOSIS_TX_PARAMS="--keyring-backend test --node $OSMOSIS_NODE --chain-id $OSMOSIS_CHAIN_ID --gas-prices $OSMOSIS_GAS_PRICE --gas-adjustment $OSMOSIS_GAS_ADJUSTMENT --gas auto"

# Celestia config
CELESTIA_NODE="http://localhost:26661"
CELESTIA_CHAIN_ID="celestia-dev-1"
CELESTIA_TX_PARAMS="--keyring-backend test --node $CELESTIA_NODE --chain-id $CELESTIA_CHAIN_ID --fees 21000utia"

# Accounts info
# Mnemonic used to generate accounts
TESTNET_MNEMONIC="remember nice comfort manage mango shuffle deputy perfect similar police birth ecology wild intact ethics raw elder romance valid shadow retreat process invite goose"
# Account that have been funded in the testnet
OSMOSIS_ACCOUNT="osmo1zx5gh6f4yy3d0vp9k37lxcdfmz7fecqjdrkcm2"
CELESTIA_ACCOUNT="celestia1zx5gh6f4yy3d0vp9k37lxcdfmz7fecqj5j5ch4"
