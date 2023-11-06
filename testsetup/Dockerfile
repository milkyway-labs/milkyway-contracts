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

COPY ./local-osmosis-testnet-new.sh ./local-osmosis-testnet-new.sh
COPY ./local-osmosis-testnet-continue.sh ./local-osmosis-testnet-continue.sh
COPY ./local-celestia-testnet-new.sh ./local-celestia-testnet-new.sh
COPY ./local-celestia-testnet-continue.sh ./local-celestia-testnet-continue.sh
COPY ./local-hermes-continue.sh ./local-hermes-continue.sh
COPY ./local-hermes-new.sh ./local-hermes-new.sh
COPY ./local-account.sh ./local-account.sh
COPY ./hermes-config.toml /root/.hermes/config.toml

RUN sh ./local-osmosis-testnet-new.sh && \
    sh ./local-celestia-testnet-new.sh && \
    sleep 15s && \
    sh ./local-hermes-new.sh && \
    sh ./local-account.sh

EXPOSE 26661
EXPOSE 26657

CMD sh ./local-celestia-testnet-continue.sh && \
    sh ./local-osmosis-testnet-continue.sh && \
    sh ./local-hermes-continue.sh