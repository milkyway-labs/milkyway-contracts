FROM ubuntu:latest

RUN apt-get update
RUN apt-get install -y tmux
RUN apt-get install -y git
RUN apt-get install -y make
RUN apt-get install -y golang-go
RUN apt-get install -y wget
RUN apt install unzip

RUN wget https://github.com/osmosis-labs/osmosis/releases/download/v20.1.1/osmosisd-20.1.1-linux-amd64
RUN mv osmosisd-20.1.1-linux-amd64 /usr/local/bin/osmosisd
RUN chmod +x /usr/local/bin/osmosisd

RUN wget https://github.com/celestiaorg/celestia-app/releases/download/v1.3.0/celestia-app_Linux_arm64.tar.gz
RUN tar -xvf celestia-app_Linux_arm64.tar.gz
RUN mv celestia-appd /usr/local/bin
RUN chmod +x /usr/local/bin/celestia-appd

RUN wget https://github.com/informalsystems/hermes/releases/download/v1.7.0/hermes-v1.7.0-aarch64-unknown-linux-gnu.zip
RUN unzip hermes-v1.7.0-aarch64-unknown-linux-gnu.zip
RUN mv hermes /usr/local/bin
RUN chmod +x /usr/local/bin/hermes

RUN apt-get install -y psmisc
RUN apt-get install -y jq

RUN celestia-appd keys add relayer --output=json --keyring-backend test 2> ./celestia-relayer-key.json
RUN osmosisd keys add relayer --output=json --keyring-backend test 2> ./osmosis-relayer-key.json

COPY . .

COPY ./hermes-config.toml /root/.hermes/config.toml

RUN hermes keys add --chain osmosis-dev-1 --key-file './osmosis-relayer-key.json'
RUN hermes keys add --chain celestia-dev-1 --key-file './celestia-relayer-key.json'

RUN sh ./local-osmosis-testnet-new.sh && sleep 15s && \
    OSMOSIS_ADDR=$(jq -r '.address' ./osmosis-relayer-key.json) && \
    osmosisd tx bank send validator1 $OSMOSIS_ADDR 50000000stake --keyring-backend=test --home=$HOME/.osmosisd/validator1 --chain-id osmosis-dev-1 --fees 875stake -y && \
    sleep 10s
RUN sh ./local-celestia-testnet-new.sh && sleep 15 && \
    CELESTIA_ADDR=$(jq -r '.address' ./celestia-relayer-key.json) && \
    celestia-appd tx bank send validator $CELESTIA_ADDR 5000000000utia --node http://127.0.0.1:26661 --fees 21000utia -y  --chain-id celestia-dev-1 && \
    sleep 10s

EXPOSE 26661
EXPOSE 26657

RUN sh ./local-celestia-testnet-continue.sh && \
    sh ./local-osmosis-testnet-continue.sh && sleep 15s && \
    hermes create client --host-chain celestia-dev-1 --reference-chain osmosis-dev-1 && \
    hermes create client --host-chain osmosis-dev-1 --reference-chain celestia-dev-1 && \
    hermes create connection --a-chain celestia-dev-1 --b-chain osmosis-dev-1 && \
    hermes create channel --a-chain celestia-dev-1 --a-connection connection-0 --a-port transfer --b-port transfer

CMD sh ./local-celestia-testnet-continue.sh && \
    sh ./local-osmosis-testnet-continue.sh && \
    sleep 15s && \
    hermes start 