import { CosmWasmClient } from "cosmwasm";

const networks = {
  ["osmo-test-5"]: {
    id: "osmo-test-5",
    contract: "osmo1ql4s7l8shevqf77guzh7q5qh54hplrhmjnn29kvcfw4ypyuanlzq0p47ys",
    rpc: "https://rpc.testnet.osmosis.zone:443",
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
