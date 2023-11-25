import { CosmWasmClient } from "cosmwasm";

const networks = {
  // ["osmosis-testnet"]: {
  //   id: "osmosis-testnet",
  //   contract: "osmo1h6d53zdzp4dwqr742qvzlucafghuhus7653su0f8cfdfzzkww4as9389xs",
  //   rpc: "https://rpc.testnet.osmosis.zone:443",
  // },
  // ["canary"]: {
  //   id: "canary",
  //   contract: "osmo1nc5tatafv6eyq7llkr2gv50ff9e22mnf70qgjlv737ktmt4eswrqvlx82r",
  //   rpc: "https://osmosis-rpc.milkyway.hanjun.kim",
  // },
  ["local"]: {
    id: "local",
    contract: "osmo153r9tg33had5c5s54sqzn879xww2q2egektyqnpj6nwxt8wls70qxukxqg",
    rpc: "http://localhost:26657",
  },
};

export const connect = async (network) => {
  try {
    const client = await CosmWasmClient.connect(network.rpc);
    network.client = client;
    console.log("Connected to", network.id);
  } catch (err) {
    console.error(err);
    setTimeout(() => {
      console.log("Reconnecting...");
      connect(network);
    }, 3000);
  }
};

Object.values(networks).forEach(async (network) => {
  network.ready = new Promise(async (resolve) => {
    await connect(network);
    resolve();
  });
});

export default networks;
