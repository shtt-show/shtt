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
echo "Original content" > test.txt
git add test.txt
git commit -m "Initial commit" > /dev/null 2>&1

# Modify the file
echo "Modified content" > test.txt

# Run shtt dump (now always porcelain format)
OUTPUT=$("$SHTT_BINARY" dump)

# Check that output shows the modified file
if echo "$OUTPUT" | grep -q " M test.txt"; then
    exit 0
else
    echo "Expected to see ' M test.txt' in output:"
    echo "$OUTPUT"
    exit 1
fi