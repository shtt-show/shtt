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

# Create and commit an initial file
echo "Initial content" > initial.txt
git add initial.txt
git commit -m "Initial commit" > /dev/null 2>&1

# Create some changes
echo "Modified content" > initial.txt
echo "New file content" > new_file.txt

# Test save with message provided via command line
OUTPUT=$("$SHTT_BINARY" save --message "Test commit message" 2>&1 || echo "FAILED")

# Check if the command succeeded (we expect it to fail in CI because there's no remote)
# But we can check that it at least tried to commit
if echo "$OUTPUT" | grep -q "Created commit:" || echo "$OUTPUT" | grep -q "Failed to find 'origin' remote"; then
    # Either it succeeded in committing or failed at the push stage (which is expected in tests)
    exit 0
else
    echo "Expected to see commit creation or origin remote error in output:"
    echo "$OUTPUT"
    exit 1
fi