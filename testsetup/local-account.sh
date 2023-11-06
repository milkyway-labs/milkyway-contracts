# test mnemonic, so in tests you have a funded account
# boy view flame close solar robust crunch slot govern false jungle dirt blade minor shield bounce rent expand anxiety busy pull inject grace require
# addresses
# celestia1sfhy3emrgp26wnzuu64p06kpkxd9phel74e0yx
# osmo1sfhy3emrgp26wnzuu64p06kpkxd9phel8ym0ge

OSMOSIS_ADDR=osmo1sfhy3emrgp26wnzuu64p06kpkxd9phel8ym0ge
osmosisd tx bank send validator1 $OSMOSIS_ADDR 50000000stake --keyring-backend=test --home=$HOME/.osmosisd/validator1 --chain-id osmosis-dev-1 --fees 875stake -y --broadcast-mode block
osmosisd tx bank send validator1 $OSMOSIS_ADDR 50000000uosmo --keyring-backend=test --home=$HOME/.osmosisd/validator1 --chain-id osmosis-dev-1 --fees 875stake -y --broadcast-mode block

CELESTIA_ADDR=celestia1sfhy3emrgp26wnzuu64p06kpkxd9phel74e0yx
celestia-appd tx bank send validator $CELESTIA_ADDR 5000000000utia --node http://0.0.0.0:26661 --fees 21000utia -y  --chain-id celestia-dev-1 --broadcast-mode block

# osmosisd keys add test_master --recover
# celestia-appd keys add test_master --recover