#!/bin/bash

set -e

# Create a temporary directory
TEST_DIR=$(mktemp -d)
cd "$TEST_DIR"

# Cleanup function
cleanup() {
    cd /
    rm -rf "$TEST_DIR"
}
trap cleanup EXIT

# Initialize git repo
git init > /dev/null 2>&1
git config user.email "test@example.com"
git config user.name "Test User"

# Create and commit a file
echo "Original content" > committed.txt
git add committed.txt
git commit -m "Initial commit" > /dev/null 2>&1

# Modify the committed file
echo "Modified content" > committed.txt

# Create an untracked file
echo "Untracked content" > untracked.txt

# Run shtt dump (now always shows all changes including untracked)
OUTPUT=$("$SHTT_BINARY" dump)

# Check that output shows both the modified file and the untracked file
if echo "$OUTPUT" | grep -q " M committed.txt" && echo "$OUTPUT" | grep -q "? untracked.txt"; then
    exit 0
else
    echo "Expected to see ' M committed.txt' and '? untracked.txt' in output:"
    echo "$OUTPUT"
    exit 1
fi
