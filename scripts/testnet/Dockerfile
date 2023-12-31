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

RUN ARCH=$(arch | sed s/aarch64/arm64/) && \
    wget https://github.com/celestiaorg/celestia-app/releases/download/v1.3.0/celestia-app_Linux_$ARCH.tar.gz && \
    tar -xvf celestia-app_Linux_$ARCH.tar.gz
RUN mv celestia-appd /usr/local/bin
RUN chmod +x /usr/local/bin/celestia-appd

RUN ARCH=$(arch) && \
    wget https://github.com/informalsystems/hermes/releases/download/v1.7.0/hermes-v1.7.0-$ARCH-unknown-linux-gnu.zip && \
    unzip hermes-v1.7.0-$ARCH-unknown-linux-gnu.zip
RUN mv hermes /usr/local/bin
RUN chmod +x /usr/local/bin/hermes

RUN apt-get install -y psmisc
RUN apt-get install -y jq


COPY ./check-node-running.sh ./check-node-running.sh
COPY ./local-osmosis-testnet-new.sh ./local-osmosis-testnet-new.sh
RUN sh ./local-osmosis-testnet-new.sh
COPY ./local-celestia-testnet-multi-new.sh ./local-celestia-testnet-multi-new.sh
RUN sh ./local-celestia-testnet-multi-new.sh

COPY ./local-osmosis-testnet-continue.sh ./local-osmosis-testnet-continue.sh
COPY ./local-celestia-testnet-multi-continue.sh ./local-celestia-testnet-multi-continue.sh
COPY ./local-hermes-continue.sh ./local-hermes-continue.sh
COPY ./local-hermes-new.sh ./local-hermes-new.sh
COPY ./local-account.sh ./local-account.sh
COPY ./hermes-config.toml ./hermes-config.toml
COPY ./local-hermes-clear-packets.sh ./local-hermes-clear-packets.sh
RUN sh ./local-celestia-testnet-multi-continue.sh && \
    sh ./local-osmosis-testnet-continue.sh && \
    sh ./check-node-running.sh osmosis1 && \
    sh ./check-node-running.sh celestia1 && \
    sh ./local-hermes-new.sh && \
    sh ./local-account.sh

EXPOSE 26661
EXPOSE 26657
EXPOSE 1317
EXPOSE 1314
EXPOSE 9090

# last command needs to be persistant to keep container running
CMD sh ./local-celestia-testnet-multi-continue.sh && \
    sh ./local-osmosis-testnet-continue.sh && \
    sh ./check-node-running.sh osmosis1 && \
    sh ./check-node-running.sh celestia1 && \
    sh ./local-hermes-clear-packets.sh && \
    sh ./local-hermes-continue.sh && \
    tail -f /dev/null