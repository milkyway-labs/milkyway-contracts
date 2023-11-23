import { CosmWasmClient } from "cosmwasm";

const networks = {
  ["osmosis-testnet"]: {
    id: "osmosis-testnet",
    contract: "osmo1ql4s7l8shevqf77guzh7q5qh54hplrhmjnn29kvcfw4ypyuanlzq0p47ys",
    rpc: "https://rpc.testnet.osmosis.zone:443",
  },
  ["canary"]: {
    id: "canary",
    contract: "osmo1nc5tatafv6eyq7llkr2gv50ff9e22mnf70qgjlv737ktmt4eswrqvlx82r",
    rpc: "https://osmosis-rpc.milkyway.hanjun.kim/",
  },
};

Object.values(networks).forEach(async (network) => {
  network.ready = new Promise(async (resolve) => {
    const client = await CosmWasmClient.connect(network.rpc);
    network.client = client;
    resolve();
    console.log("Connected to", network.id);
  });
});

export default networks;
