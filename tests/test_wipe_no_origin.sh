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

# Initialize git repo but don't set up origin
git init > /dev/null 2>&1
git config user.email "test@example.com"
git config user.name "Test User"

# Create and commit a file
echo "Hello, World!" > test.txt
git add test.txt
git commit -m "Initial commit" > /dev/null 2>&1

# Run shtt wipe (should fail because there's no origin)
if "$SHTT_BINARY" wipe 2>&1 | grep -q "Failed to find remote branch"; then
    exit 0
else
    echo "Expected error message about missing remote branch"
    exit 1
fi