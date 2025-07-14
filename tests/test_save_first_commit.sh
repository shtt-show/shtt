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

# Initialize git repo (but don't make any commits)
git init > /dev/null 2>&1
git config user.email "test@example.com"
git config user.name "Test User"

# Create a file for the very first commit
echo "First file ever" > first.txt

# Test that the first auto-numbered commit is "1st Commit"
OUTPUT=$("$SHTT_BINARY" save 2>&1 || echo "FAILED")

# Check if the command succeeded and created a commit
if echo "$OUTPUT" | grep -q "Created commit:" || echo "$OUTPUT" | grep -q "Failed to find 'origin' remote"; then
    # Verify the commit message is "1st Commit"
    LAST_COMMIT_MSG=$(git log -1 --pretty=format:%s)
    if [ "$LAST_COMMIT_MSG" = "1st Commit" ]; then
        true
    else
        echo "✗ Expected '1st Commit', got: '$LAST_COMMIT_MSG'"
        exit 1
    fi
else
    echo "✗ First commit save command failed unexpectedly:"
    echo "$OUTPUT"
    exit 1
fi

# Test that the second commit is "2nd Commit"
echo "Second file" > second.txt
OUTPUT=$("$SHTT_BINARY" save 2>&1 || echo "FAILED")

if echo "$OUTPUT" | grep -q "Created commit:" || echo "$OUTPUT" | grep -q "Failed to find 'origin' remote"; then
    # Verify the commit message is "2nd Commit"
    LAST_COMMIT_MSG=$(git log -1 --pretty=format:%s)
    if [ "$LAST_COMMIT_MSG" = "2nd Commit" ]; then
        true
    else
        echo "✗ Expected '2nd Commit', got: '$LAST_COMMIT_MSG'"
        exit 1
    fi
else
    echo "✗ Second commit save command failed unexpectedly:"
    echo "$OUTPUT"
    exit 1
fi
