#!/usr/bin/env bash

# This script serves as a wrapper for various applications
# needed to run a testnet for testing our smart contracts.
# The application to run is determined by the name of this file,
# allowing the creation of links to this script with the actual application names.
# This way, other software depending on these applications
# will think they are using the actual application.

set -e

DEFAULT_OSMOSISD_VERSION="25.0.0"
DEFAULT_CELESTIA_VERSION="1.11.0"
DEFAULT_HERMES_VERSION="1.9.0"

SCRIPT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)
BINS_DIR="$SCRIPT_DIR/bins"
SCRIPT_NAME=$(basename -- "$0")

if [ ! -d $BINS_DIR ]; then
  mkdir $BINS_DIR
fi

function download_osmosisd() {
  local download_version="$DEFAULT_OSMOSISD_VERSION"
  if [[ ! -z "$OSMOSISD_VERSION" ]]; then
    download_version="$OSMOSISD_VERSION"
  fi

  local download_url="https://github.com/osmosis-labs/osmosis/releases/download/v${download_version}/osmosisd-${download_version}-linux-amd64"
  wget -q -O "$BINS_DIR/osmosisd" $download_url
  chmod +x "$BINS_DIR/osmosisd"
}

function download_celestia-appd() {
  local download_version="$DEFAULT_CELESTIA_VERSION"
  if [[ ! -z "$CELESTIA_APP_VERSION" ]]; then
    download_version="$CELESTIA_APP_VERSION"
  fi

  local download_url="https://github.com/celestiaorg/celestia-app/releases/download/v${download_version}/celestia-app_Linux_x86_64.tar.gz"
  wget -q -O "$BINS_DIR/celestia.tar.gz" $download_url

  tar --extract --file "$BINS_DIR/celestia.tar.gz" celestia-appd
  rm -f "$BINS_DIR/celestia.tar.gz"
  mv celestia-appd "$BINS_DIR"
  chmod +x "$BINS_DIR/celestia-appd"
}

function download_hermes() {
  local download_version=$DEFAULT_HERMES_VERSION
  if [[ ! -z "$HERMES_VERSION" ]]; then
    download_version="$HERMES_VERSION"
  fi

  local download_url="https://github.com/informalsystems/hermes/releases/download/v${download_version}/hermes-v${download_version}-x86_64-unknown-linux-gnu.zip"
  wget -q -O "$BINS_DIR/hermes.zip" $download_url

  unzip -j "$BINS_DIR/hermes.zip" "hermes" -d "$BINS_DIR"
  chmod +x "$BINS_DIR/hermes"
  rm -f "$BINS_DIR/hermes.zip"
}

function ensure_installed() {
  local application=$1
  if [[ ! -f "$BINS_DIR/$application" ]]; then
    download_$application
  fi
}

# Ensure that we have the proper a application installed
ensure_installed $SCRIPT_NAME
# Run the application passing the arguments to it
$BINS_DIR/$SCRIPT_NAME $@
