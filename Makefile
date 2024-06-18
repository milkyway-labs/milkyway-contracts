SHELL := /bin/bash

check:
	cargo check --target wasm32-unknown-unknown

build:
	cargo build

test:
	cargo test --locked --workspace

# copied from DAO DAO:
# https://github.com/DA0-DA0/polytone/blob/main/devtools/optimize.sh
optimize:
	if [[ $(shell uname -m) =~ (arm64)|(aarch64) ]]; then \
	docker run --rm -v "$(CURDIR)":/code \
		--mount type=volume,source="$(notdir $(CURDIR))_cache",target=/target \
		--mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
		--platform linux/arm64 \
		cosmwasm/workspace-optimizer-arm64:0.14.0; else \
	docker run --rm -v "$(CURDIR)":/code \
		--mount type=volume,source="$(notdir $(CURDIR))_cache",target=/target \
		--mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
		--platform linux/amd64 \
		cosmwasm/workspace-optimizer:0.14.0; fi

initlocal: optimize _initlocal
_initlocal:
	bash scripts/testnet/init-stake-contract.sh
