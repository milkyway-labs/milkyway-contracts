import Redis from "ioredis";
import { subscribe } from "./websocket.js";
import networks from "./networks.js";

const client = new Redis(process.env.REDIS);

const handleUpdate = (network) => {
  let lastHeight = 0;
  subscribe(network, async (height) => {
    try {
      if (height > lastHeight) {
        await network.ready;
        const state = await network.client.queryContractSmart(
          network.contract,
          {
            state: {},
          }
        );
        await client.set(network.id + "-state", JSON.stringify(state));

        const batches = await network.client.queryContractSmart(
          network.contract,
          {
            batches: {},
          }
        );
        await client.set(network.id + "-batches", JSON.stringify(batches));

        lastHeight = height;

        console.log("Updated", network.id);

        await client.set(network.id + "-updated", Date.now().toString());
      }
    } catch (err) {
      console.error(err);
    }
  });
};

Object.values(networks).forEach(handleUpdate);
