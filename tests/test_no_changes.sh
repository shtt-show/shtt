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
echo "Hello, World!" > test.txt
git add test.txt
git commit -m "Initial commit" > /dev/null 2>&1

# Run shtt dump (should show no changes)
OUTPUT=$("$SHTT_BINARY" dump)

# Check that output shows no changes
if echo "$OUTPUT" | grep -q "No changes detected"; then
    exit 0
else
    echo "Expected to see 'No changes detected' in output:"
    echo "$OUTPUT"
    exit 1
fi