import { CosmWasmClient } from "cosmwasm";

const networks = {
  ["osmosis-testnet"]: {
    id: "osmosis-testnet",
    contract: "osmo1zemdpquegrkcenhav9z47jp0hpq4hmk7gflq66qxu7nmpx09tygqtssvhv",
    rpc: "https://rpc.testnet.osmosis.zone:443",
  },
  ["canary"]: {
    id: "canary",
    contract: "osmo14hj2tavq8fpesdwxxcu44rty3hh90vhujrvcmstl4zr3txmfvw9sq2r9g9",
    rpc: "https://osmosis-rpc.devnet.milkyway.zone",
  },
  // ["local"]: {
  //   id: "local",
  //   contract: "osmo153r9tg33had5c5s54sqzn879xww2q2egektyqnpj6nwxt8wls70qxukxqg",
  //   rpc: "http://localhost:26657",
  // },
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
