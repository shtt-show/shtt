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

# Create and commit an initial file to establish the repository
echo "Initial content" > initial.txt
git add initial.txt
git commit -m "Initial commit" > /dev/null 2>&1

# Test first auto-numbered commit (should be "2nd Commit" since we have 1 existing commit)
echo "First change" > file1.txt
OUTPUT=$("$SHTT_BINARY" save 2>&1 || echo "FAILED")

# Check if the command succeeded and created a commit
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
    echo "✗ First save command failed unexpectedly:"
    echo "$OUTPUT"
    exit 1
fi

# Test second auto-numbered commit (should be "3rd Commit")
echo "Second change" > file2.txt
OUTPUT=$("$SHTT_BINARY" save 2>&1 || echo "FAILED")

if echo "$OUTPUT" | grep -q "Created commit:" || echo "$OUTPUT" | grep -q "Failed to find 'origin' remote"; then
    # Verify the commit message is "3rd Commit"
    LAST_COMMIT_MSG=$(git log -1 --pretty=format:%s)
    if [ "$LAST_COMMIT_MSG" = "3rd Commit" ]; then
        true
    else
        echo "✗ Expected '3rd Commit', got: '$LAST_COMMIT_MSG'"
        exit 1
    fi
else
    echo "✗ Second save command failed unexpectedly:"
    echo "$OUTPUT"
    exit 1
fi

# Test third auto-numbered commit (should be "4th Commit")
echo "Third change" > file3.txt
OUTPUT=$("$SHTT_BINARY" save 2>&1 || echo "FAILED")

if echo "$OUTPUT" | grep -q "Created commit:" || echo "$OUTPUT" | grep -q "Failed to find 'origin' remote"; then
    # Verify the commit message is "4th Commit"
    LAST_COMMIT_MSG=$(git log -1 --pretty=format:%s)
    if [ "$LAST_COMMIT_MSG" = "4th Commit" ]; then
        true
    else
        echo "✗ Expected '4th Commit', got: '$LAST_COMMIT_MSG'"
        exit 1
    fi
else
    echo "✗ Third save command failed unexpectedly:"
    echo "$OUTPUT"
    exit 1
fi

# Test that providing a manual message still works
echo "Manual change" > manual.txt
OUTPUT=$("$SHTT_BINARY" save --message "Custom commit message" 2>&1 || echo "FAILED")

if echo "$OUTPUT" | grep -q "Created commit:" || echo "$OUTPUT" | grep -q "Failed to find 'origin' remote"; then
    # Verify the commit message is the custom one
    LAST_COMMIT_MSG=$(git log -1 --pretty=format:%s)
    if [ "$LAST_COMMIT_MSG" = "Custom commit message" ]; then
        true
    else
        echo "✗ Expected 'Custom commit message', got: '$LAST_COMMIT_MSG'"
        exit 1
    fi
else
    echo "✗ Manual message save command failed unexpectedly:"
    echo "$OUTPUT"
    exit 1
fi

# Test that the next auto-numbered commit continues the sequence (should be "6th Commit")
echo "After manual change" > after_manual.txt
OUTPUT=$("$SHTT_BINARY" save 2>&1 || echo "FAILED")

if echo "$OUTPUT" | grep -q "Created commit:" || echo "$OUTPUT" | grep -q "Failed to find 'origin' remote"; then
    # Verify the commit message continues the sequence
    LAST_COMMIT_MSG=$(git log -1 --pretty=format:%s)
    if [ "$LAST_COMMIT_MSG" = "6th Commit" ]; then
        true
    else
        echo "✗ Expected '6th Commit', got: '$LAST_COMMIT_MSG'"
        exit 1
    fi
else
    echo "✗ Final save command failed unexpectedly:"
    echo "$OUTPUT"
    exit 1
fi
