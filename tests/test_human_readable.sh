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

# Create an untracked file
echo "Hello, World!" > test.txt

# Run shtt dump (now always porcelain format)
OUTPUT=$("$SHTT_BINARY" dump)

# Check that output contains the porcelain format
if echo "$OUTPUT" | grep -q "? test.txt"; then
    exit 0
else
    echo "Expected porcelain format '? test.txt' in output:"
    echo "$OUTPUT"
    exit 1
fi
