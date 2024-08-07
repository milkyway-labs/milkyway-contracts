FROM --platform=linux/amd64 ubuntu:latest

RUN apt-get update && \
  apt-get install -y tmux git make golang-go wget unzip psmisc jq

# Binaries versions
# Remember to update the hermes compatibility mode when bumping the versions
# https://hermes.informal.systems/documentation/configuration/comet-compat-mode.html
ENV OSMOSISD_VERSION="25.0.0" \
  CELESTIA_APP_VERSION="1.11.0" \
  HERMES_VERSION="1.9.0"

# Prepare the container environment
RUN mkdir /milkiway
COPY ./* /milkiway/
COPY ./bins /milkiway/bins
COPY ./utils /milkiway/utils
WORKDIR /milkiway

# Install binaries
RUN bash -c "source /milkiway/bins/download.sh && \
  ensure_installed osmosisd && \
  ensure_installed celestia-appd && \
  ensure_installed hermes"

RUN ./local-osmosis-testnet-new.sh
RUN ./local-celestia-testnet-multi-new.sh
RUN ./local-celestia-testnet-multi-continue.sh && \
  ./local-osmosis-testnet-continue.sh && \
  ./check-node-running.sh osmosis1 && \
  ./check-node-running.sh celestia1 && \
  ./fund-accounts.sh && \
  ./local-hermes-new.sh && \
  ./tia-osmo-pool.sh

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
