// Import the WebSocket library for real-time bidirectional communication.
import WebSocket from "ws";
// Import the UUID library to generate a unique ID for each subscription request.
import { v4 as uuidv4 } from "uuid";

// Initialize websocket and wsQuery variables.
let websocket;
let wsQuery;
// This function initiates a WebSocket connection and sends a subscription request to track transactions that fulfill certain conditions.
export const subscribe = (network, cb) => {
  try {
    // Open a new WebSocket connection to the specified URL.
    const wsUrl = network.rpc.replace("https://", "wss://") + "/websocket",
      websocket = new WebSocket(wsUrl);
    // Define the subscription request. It asks for transactions where the recipient address, and checks for transactions to be published.
    wsQuery = {
      jsonrpc: "2.0",
      method: "subscribe",
      id: uuidv4().toString(),
      params: {
        query: `wasm._contract_address = '${network.contract}'`,
      },
    };
    // When the WebSocket connection is established, send the subscription request.
    websocket.on("open", () => {
      console.log("Connected to", network.id);
      websocket.send(JSON.stringify(wsQuery));
    });
    // When a message (i.e., a matching transaction) is received, log the transaction and close the WebSocket connection.
    websocket.on("message", (event) => {
      const eventData = JSON.parse(event);
      if (eventData && eventData.result && eventData.result.data) {
        const data = eventData.result.data.value.TxResult;
        const height = data.height;
        cb(height);
      }
    });
    // If an error occurs with the WebSocket, log the error and close the WebSocket connection.
    websocket.on("error", (error) => {
      console.error(error);
      disconnectFromWebsocket();
      setTimeout(() => {
        console.log("Reconnecting...");
        subscribe(network, cb);
      }, 3000);
    });
  } catch (err) {
    // If an error occurs when trying to connect or subscribe, log the error and close the WebSocket connection.
    console.error(err);
    disconnectFromWebsocket();
    setTimeout(() => {
      console.log("Reconnecting...");
      subscribe(network, cb);
    }, 3000);
  }
};
// This function closes the WebSocket connection and resets the websocket and wsQuery variables.
export const disconnectFromWebsocket = () => {
  // If the WebSocket isn't open, exit the function.
  if (!websocket || websocket.readyState !== WebSocket.OPEN) return;
  // Send an 'unsubscribe' message to the server.
  websocket.send(JSON.stringify({ ...wsQuery, method: "unsubscribe" }));
  // Close the WebSocket connection.
  websocket.close();
  // Reset the websocket and wsQuery variables.
  websocket = null;
  wsQuery = null;
};
// When the process is exiting, close the WebSocket connection if it's still open.
process.on("exit", () => {
  disconnectFromWebsocket();
});
