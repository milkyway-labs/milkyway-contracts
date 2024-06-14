#!/usr/bin/env bash

set -e

DEFAULT_OSMOSISD_VERSION="25.0.0"
DEFAULT_CELESTIA_VERSION="1.11.0"
DEFAULT_HERMES_VERSION="1.9.0"

SCRIPT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)
BINS_DIR="$SCRIPT_DIR/bins"


function download_osmosisd() {
  local download_version="$DEFAULT_OSMOSISD_VERSION"
  if [[ -n "$OSMOSISD_VERSION" ]]; then
    download_version="$OSMOSISD_VERSION"
  fi

  local download_url="https://github.com/osmosis-labs/osmosis/releases/download/v${download_version}/osmosisd-${download_version}-linux-amd64"
  echo "Downloading osmosisd v${download_version}"
  wget -q -O "$BINS_DIR/osmosisd" "$download_url" || {
      echo "Failed to download osmosisd."
      return 1
  }
  chmod +x "$BINS_DIR/osmosisd"
}

function download_celestia-appd() {
  local download_version="$DEFAULT_CELESTIA_VERSION"
  if [[ -n "$CELESTIA_APP_VERSION" ]]; then
    download_version="$CELESTIA_APP_VERSION"
  fi

  local download_url="https://github.com/celestiaorg/celestia-app/releases/download/v${download_version}/celestia-app_Linux_x86_64.tar.gz"
  echo "Downloading celestia-appd v${download_version}"
  wget -q -O "$BINS_DIR/celestia.tar.gz" "$download_url" || {
      echo "Failed to download celestia-appd."
      return 1
  }


  tar --extract --file "$BINS_DIR/celestia.tar.gz" celestia-appd
  rm -f "$BINS_DIR/celestia.tar.gz"
  mv celestia-appd "$BINS_DIR"
  chmod +x "$BINS_DIR/celestia-appd"
}

function download_hermes() {
  local download_version=$DEFAULT_HERMES_VERSION
  if [[ -n "$HERMES_VERSION" ]]; then
    download_version="$HERMES_VERSION"
  fi

  local download_url="https://github.com/informalsystems/hermes/releases/download/v${download_version}/hermes-v${download_version}-x86_64-unknown-linux-gnu.zip"
  echo "Downloading hermes v${download_version}"
  wget -q -O "$BINS_DIR/hermes.zip" "$download_url" || {
      echo "Failed to download hermes."
      return 1
  }


  unzip -j "$BINS_DIR/hermes.zip" "hermes" -d "$BINS_DIR"
  chmod +x "$BINS_DIR/hermes"
  rm -f "$BINS_DIR/hermes.zip"
}

function ensure_installed() {
  mkdir -p "$BINS_DIR"

  local application=$1
  if [[ ! -f "$BINS_DIR/$application" ]]; then
    download_"$application" || {
      echo "Failed to download $application"
      exit 1
    }
  fi
}
