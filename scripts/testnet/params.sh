# Osmosis config
OSMOSIS_NODE="http://localhost:26657"
OSMOSIS_CHAIN_ID="osmosis-dev-1"
OSMOSIS_GAS_PRICE="0.025stake"
OSMOSIS_GAS_ADJUSTMENT="1.7"
OSMOSIS_TX_PARAMS="--node $OSMOSIS_NODE --chain-id $OSMOSIS_CHAIN_ID --gas-prices $OSMOSIS_GAS_PRICE --gas-adjustment $OSMOSIS_GAS_ADJUSTMENT --gas auto"

# Celestia config
CELESTIA_NODE="http://localhost:26661"
CELESTIA_CHAIN_ID="celestia-dev-1"
CELESTIA_TX_PARAMS="--node $CELESTIA_NODE --chain-id $CELESTIA_CHAIN_ID --fees 21000utia"
