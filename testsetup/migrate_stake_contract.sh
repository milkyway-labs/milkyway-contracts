docker run --rm -v "$(pwd)":/code \
  --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
  --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
  cosmwasm/workspace-optimizer-arm64:0.13.0

RES=$(osmosisd tx wasm store ../artifacts/staking-aarch64.wasm --from validator1 --output json --node http://localhost:26657 -y -b block --gas-prices 0.025stake --gas-adjustment 1.7 --gas auto  --home ~/.osmosisd/validator1 --keyring-backend test --chain-id testing)
CODE_ID=$(echo $RES | jq -r '.logs[0].events[-1].attributes[-1].value')
ADMIN=$(osmosisd keys show validator1 --keyring-backend=test --home=~/.osmosisd/validator1 --output json | jq -r '.address')
osmosisd tx wasm migrate $CONTRACT $CODE_ID '{}' \
    --from validator1 -y \
    --node http://localhost:26657 -y -b block \
    --gas-prices 0.025stake --gas-adjustment 1.7 --gas auto  \
    --home ~/.osmosisd/validator1 --keyring-backend test --chain-id testing \
    --output json