import Redis from "ioredis";
// import { subscribe } from "./websocket.js";
import { subscribe } from "./polling.js";
import networks from "./networks.js";

console.log("Starting proxy...", new Date().toISOString());

const initRedis = () => {
  let client = new Redis(process.env.REDIS);

  // Listen to 'reconnecting' event to Redis
  client.on("reconnecting", (err) => {
    if (client.status === "reconnecting")
      console.log("Reconnecting to Redis Session Store...");
    else console.log("Error reconnecting to Redis Session Store.");
  });

  // Listen to 'error' events to the Redis connection
  client.on("error", (error) => {
    if (error.code === "ECONNRESET") {
      console.log("Connection to Redis Session Store timed out.");
    } else if (error.code === "ECONNREFUSED") {
      console.log("Connection to Redis Session Store refused!");
    } else console.log(error);
  });

  // Listen to the 'connect' event to Redis
  client.on("connect", (err) => {
    if (!err) console.log("Connected to Redis Session Store!");
  });
  return client;
};

const handleUpdate = async (network) => {
  let client;
  try {
    client = initRedis();
    await network.ready;

    const state = await network.client.queryContractSmart(network.contract, {
      state: {},
    });
    await client.set(network.id + "-state", JSON.stringify(state));

    const batches = await network.client.queryContractSmart(network.contract, {
      batches: {},
    });
    await client.set(network.id + "-batches", JSON.stringify(batches.batches));

    let users = {};
    batches.batches.forEach((batch) => {
      batch.requests.forEach((request) => {
        if (!request.redeemed) {
          const user = request.user;
          if (!users[user]) {
            users[user] = [];
          }
          users[user].push(batch.id);
        }
      });
    });
    // TODO optimize
    Object.entries(users).forEach(([user, batches]) => {
      client.set(network.id + "-claimable-" + user, JSON.stringify(batches));
    });

    console.log("Updated", network.id, Date.now().toString());

    await client.set(network.id + "-updated", Date.now().toString());
  } catch (err) {
    console.error(err);
  } finally {
    client?.quit();
  }
};

const handleUpdates = (network) => {
  let lastHeight = 0;
  subscribe(network, async (height) => {
    if (height > lastHeight) {
      lastHeight = height;
      await handleUpdate(network);
    }
  });
};

Object.values(networks).forEach(handleUpdates);
setTimeout(() => {
  Object.values(networks).forEach(handleUpdate);
}, 200);
