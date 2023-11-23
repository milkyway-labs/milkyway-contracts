#!/bin/bash

# Set the path to your repository and the Node.js script
REPO_PATH="."
NODE_SCRIPT="index.js"
SPECIFIC_FOLDER="proxy" # relative to the repository root
REMOTE_BRANCH="fabo/proxy" # adjust the remote branch name as needed

# Function to check for updates and pull
check_and_pull() {
    echo "Checking for updates..."
    git fetch

    LOCAL=$(git rev-parse @)
    REMOTE=$(git rev-parse "@{u}")
    
    # Compare the local branch with the remote branch for the specific folder
    if [ "$LOCAL" != "$REMOTE" ]; then
        echo "Changes detected in $SPECIFIC_FOLDER."
        # Optionally, pull the changes
        git pull
        npm install
    else
        echo "No changes detected in $SPECIFIC_FOLDER."
    fi
}

# Main loop
while true; do
    # Check for updates
    check_and_pull

    # Wait for a specified interval before checking again (e.g., 1 minutes)
    sleep 60
done
