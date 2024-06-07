FROM ubuntu:latest


RUN apt-get update && \
  apt-get install -y tmux git make golang-go wget unzip psmisc jq

# Binaries versions
# Remember to update the hermes compatibility mode when bumping the versions
# https://hermes.informal.systems/documentation/configuration/comet-compat-mode.html
ENV OSMOSISD_VERSION="25.0.0" \
  CELESTIA_APP_VERSION="1.11.0" \
  HERMES_VERSION="1.9.0"

RUN wget -O /usr/local/bin/osmosisd \
  https://github.com/osmosis-labs/osmosis/releases/download/v${OSMOSISD_VERSION}/osmosisd-${OSMOSISD_VERSION}-linux-amd64 && \
  chmod +x /usr/local/bin/osmosisd

RUN ARCH=$(arch | sed s/aarch64/arm64/) && \
    wget https://github.com/celestiaorg/celestia-app/releases/download/v${CELESTIA_APP_VERSION}/celestia-app_Linux_$ARCH.tar.gz && \
    tar -xvf celestia-app_Linux_$ARCH.tar.gz && \
    rm celestia-app_Linux_$ARCH.tar.gz && \
    mv celestia-appd /usr/local/bin && \
    chmod +x /usr/local/bin/celestia-appd

RUN ARCH=$(arch) && \
    wget https://github.com/informalsystems/hermes/releases/download/v${HERMES_VERSION}/hermes-v${HERMES_VERSION}-$ARCH-unknown-linux-gnu.zip && \
    unzip hermes-v${HERMES_VERSION}-$ARCH-unknown-linux-gnu.zip && \
    rm hermes-v${HERMES_VERSION}-$ARCH-unknown-linux-gnu.zip && \
    mv hermes /usr/local/bin && \
    chmod +x /usr/local/bin/hermes

COPY ./utils.sh ./utils.sh
COPY ./check-node-running.sh ./check-node-running.sh
COPY ./local-osmosis-testnet-new.sh ./local-osmosis-testnet-new.sh
RUN  ./local-osmosis-testnet-new.sh
COPY ./local-celestia-testnet-multi-new.sh ./local-celestia-testnet-multi-new.sh
RUN  ./local-celestia-testnet-multi-new.sh

COPY ./local-osmosis-testnet-continue.sh ./local-osmosis-testnet-continue.sh
COPY ./local-celestia-testnet-multi-continue.sh ./local-celestia-testnet-multi-continue.sh
COPY ./local-hermes-continue.sh ./local-hermes-continue.sh
COPY ./local-hermes-new.sh ./local-hermes-new.sh
COPY ./local-account.sh ./local-account.sh
COPY ./hermes-config.toml ./hermes-config.toml
COPY ./local-hermes-clear-packets.sh ./local-hermes-clear-packets.sh
RUN ./local-celestia-testnet-multi-continue.sh && \
    ./local-osmosis-testnet-continue.sh && \
    ./check-node-running.sh osmosis1 && \
    ./check-node-running.sh celestia1 && \
    ./local-hermes-new.sh && \
    ./local-account.sh

EXPOSE 26661
EXPOSE 26657
EXPOSE 1317
EXPOSE 1314
EXPOSE 9090

# last command needs to be persistant to keep container running
CMD ./local-celestia-testnet-multi-continue.sh && \
    ./local-osmosis-testnet-continue.sh && \
    ./check-node-running.sh osmosis1 && \
    ./check-node-running.sh celestia1 && \
    ./local-hermes-clear-packets.sh && \
    ./local-hermes-continue.sh && \
    tail -f /dev/null
