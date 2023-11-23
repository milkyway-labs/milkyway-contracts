#!/bin/bash

NODE_PROCESS_ID=$1
PATTERN="executed block"

# Function to capture and check the pane content
check_pane_output() {
  tmux capture-pane -pS -100 -t $NODE_PROCESS_ID > ${HOME}/$NODE_PROCESS_ID-tmux-buffer.txt
  # Search for the pattern in the buffer
  if grep -q "$PATTERN" ${HOME}/$NODE_PROCESS_ID-tmux-buffer.txt; then
    echo "Node $NODE_PROCESS_ID is running!"
    # Perform any action you need to do once the pattern is found
    # For example, you can kill the process or just break the loop
    return 0
  else
    return 1
  fi
}

# Loop until the pattern is found
until check_pane_output; do
  sleep 1 # Wait for some time before checking again
done

# Clean up if necessary
rm ${HOME}/$NODE_PROCESS_ID-tmux-buffer.txt