import { createClient } from "redis";
import { subscribe } from "./websocket.js";
import networks from "./networks.js";

const client = createClient({
  url: process.env.REDIS,
});

client.on("error", function (err) {
  throw err;
});
await client.connect();

let lastHeight = 0;
const network = networks["osmo-test-5"];
subscribe(network, async (height) => {
  try {
    if (height > lastHeight) {
      await network.ready;
      const state = await network.client.queryContractSmart(network.contract, {
        state: {},
      });
      await client.set("state", JSON.stringify(state));

      const batches = await network.client.queryContractSmart(
        network.contract,
        {
          batches: {},
        }
      );
      await client.set("batches", JSON.stringify(batches));

      lastHeight = height;

      console.log("Updated", network.id);
    }
  } catch (err) {
    console.error(err);
  }
});
