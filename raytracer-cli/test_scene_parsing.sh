#!/bin/bash

# Path to the scenes directory
SCENES_DIR="../scenes/scenes"

# Loop through all *.json files in the directory
for file in "$SCENES_DIR"/*.json; do
    # Run the command with the current file
    cargo run -- -i "$file" >/dev/null 2>&1
    
    # Check the exit code of the last command
    if [ $? -ne 0 ]; then
        echo "Error parsing $file"
    fi
done