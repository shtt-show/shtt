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
# We expect this to fail at the push stage since there's no origin remote
OUTPUT=$("$SHTT_BINARY" save --message "Test commit message" 2>&1 || true)

# Check that it created a commit (even if push failed)
if echo "$OUTPUT" | grep -q "Created commit:"; then
    # Verify the commit was actually created by checking git log
    if git log --oneline | head -1 | grep -q "Test commit message"; then
        exit 0
    else
        echo "Commit was not found in git log"
        exit 1
    fi
else
    echo "Expected to see 'Created commit:' in output:"
    echo "$OUTPUT"
    exit 1
fi