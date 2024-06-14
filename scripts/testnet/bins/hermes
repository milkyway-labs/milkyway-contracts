#!/usr/bin/env bash

# This script serves as a wrapper for various applications
# needed to run a testnet for testing our smart contracts.
# The application to run is determined by the name of this file,
# allowing the creation of links to this script with the actual application names.
# This way, other software depending on these applications
# will think they are using the actual application.

set -e

SCRIPT_DIR=$(cd -- "$(dirname -- "${BASH_SOURCE[0]}")" &>/dev/null && pwd)
BINS_DIR="$SCRIPT_DIR/bins"
source "$SCRIPT_DIR/download.sh"
SCRIPT_NAME="$(basename "$0")"

# Ensure that we have the proper a application installed
ensure_installed "$SCRIPT_NAME"
# Run the application passing the arguments to it
"$BINS_DIR/$SCRIPT_NAME" "$@"
