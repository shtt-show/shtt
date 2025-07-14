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
    # Verify the commit was actually created by checking git log
    if git log --oneline | head -1 | grep -q "Test commit message"; then
        true
    else
        echo "✗ Commit with manual message was not found in git log"
        exit 1
    fi
else
    echo "✗ Expected to see commit creation or origin remote error in output:"
    echo "$OUTPUT"
    exit 1
fi

# Test save without message (should auto-generate "3rd Commit")
echo "Another change" > another_file.txt
OUTPUT=$("$SHTT_BINARY" save 2>&1 || echo "FAILED")

if echo "$OUTPUT" | grep -q "Created commit:" || echo "$OUTPUT" | grep -q "Failed to find 'origin' remote"; then
    # Verify the commit was created with auto-generated message
    LAST_COMMIT_MSG=$(git log -1 --pretty=format:%s)
    if [ "$LAST_COMMIT_MSG" = "3rd Commit" ]; then
        true
    else
        echo "✗ Expected auto-generated '3rd Commit', got: '$LAST_COMMIT_MSG'"
        exit 1
    fi
else
    echo "✗ Expected to see commit creation or origin remote error in output:"
    echo "$OUTPUT"
    exit 1
fi
