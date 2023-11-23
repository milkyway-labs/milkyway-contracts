# Data Proxy

Purpose is to use a proper data separation for the frontend and not query nodes directly as they are often unreliable and slow.

This listenes to the node and persists the data in Upstash (Redis).

This script listens on github and pulls if new content. Nodemon will restart the node instance if any change detected.

## Keys

NETWORK=osmosis-testnet,canary

NETWORK-state
NETWORK-config
NETWORK-batches
NETWORK-claimable-ADDRESS
NETWORK-updated

Check also in the browser: `https://console.upstash.com/redis/b2de01b9-b4f5-47ec-aa56-9d6c73554bc3?tab=data-browser`

## Run

Go to console of the server
Make sure to have ssh rights for Github
Checkout repo

```
cd proxy
tmux new -s checker -d bash run.sh

sudo npm i env-cmd nodemon -g
tmux new -s nodemon -d env-cmd nodemon --watch "." --exec "node index.js"
```
