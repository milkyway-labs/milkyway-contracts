import axios from "axios";

// This function initiates a WebSocket connection and sends a subscription request to track transactions that fulfill certain conditions.
export const subscribe = (network, cb) => {
  let lastHeight = 0;
  setInterval(async () => {
    try {
      let rpc = network.rpc;
      let {
        data: {
          result: { block },
        },
      } = await axios.get(rpc + "/block");
      let height = block.header.height;
      if (height > lastHeight) {
        lastHeight = height;

        let {
          data: {
            result: { txs },
          },
        } = await axios.get(
          rpc +
            `/tx_search?query="wasm._contract_address='${network.contract}' AND tx.height=${height}"`
        );

        if (txs.length > 0) {
          console.log("Relevant block detected", network.id);
          cb(height);
        }
      }
    } catch (err) {
      console.error(err);
    }
  }, 5000);
};
