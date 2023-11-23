#!/bin/bash

# Set the path to your repository and the Node.js script
REPO_PATH="."
NODE_SCRIPT="index.js"
SPECIFIC_FOLDER="proxy" # relative to the repository root
REMOTE_BRANCH="fabo/proxy" # adjust the remote branch name as needed

sudo npm i nodemon -g 

# Function to check for updates and pull
check_and_pull() {
    echo "Checking for updates..."
    git fetch
    
    # Compare the local branch with the remote branch for the specific folder
    if git diff --quiet HEAD "$REMOTE_BRANCH" -- "$SPECIFIC_FOLDER"; then
        return 1
    else
        echo "Changes detected in $SPECIFIC_FOLDER."
        # Optionally, pull the changes
        git pull
        return 0
    fi
}

# Main loop
while true; do
    # Check for updates
    if check_and_pull; then
        # Run the Node.js script
        echo "Running Node.js script..."
        npm install
        nodemon "$NODE_SCRIPT"
    fi

    # Wait for a specified interval before checking again (e.g., 10 minutes)
    sleep 600
done
