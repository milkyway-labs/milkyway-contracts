# Data Proxy

Purpose is to use a proper data separation for the frontend and not query nodes directly as they are often unreliable and slow.

This listenes to the node and persists the data in Upstash (Redis).

## Keys

NETWORK=osmosis-testnet,canary

NETWORK-state
NETWORK-config
NETWORK-batches
NETWORK-claimable-ADDRESS
NETWORK-updated

## Run

Go to console of server
Make sure to have ssh rights for Github
Checkout repo
Install node,npm,nodemon
Run proxy/runs.sh
