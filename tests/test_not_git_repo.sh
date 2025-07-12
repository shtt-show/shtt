#!/bin/bash

set -e

# Create a temporary directory (but don't initialize git)
TEST_DIR=$(mktemp -d)
cd "$TEST_DIR"

# Cleanup function
cleanup() {
    cd /
    rm -rf "$TEST_DIR"
}
trap cleanup EXIT

# Run shtt dump (should fail)
if "$SHTT_BINARY" dump 2>&1 | grep -q "git repository"; then
    exit 0
else
    echo "Expected error message about git repository"
    exit 1
fi