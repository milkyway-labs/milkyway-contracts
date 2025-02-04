SHELL := /bin/bash

check:
	cargo check --target wasm32-unknown-unknown

build:
	cargo build

test:
	cargo test --locked --workspace

optimize:
	if [[ $(shell uname -m) =~ (arm64)|(aarch64) ]]; then \
	docker run --rm -v "$(CURDIR)":/code \
		--mount type=volume,source="$(notdir $(CURDIR))_cache",target=/target \
		--mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
		--platform linux/arm64 \
		cosmwasm/optimizer:0.16.1; else \
  docker run --rm -v "$(CURDIR)":/code \
		--mount type=volume,source="$(notdir $(CURDIR))_cache",target=/target \
		--mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
    cosmwasm/optimizer:0.16.1; fi

initlocal: optimize _initlocal
_initlocal:
	bash scripts/testnet/init-stake-contract.sh
