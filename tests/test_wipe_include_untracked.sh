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

# Create and commit some files
echo "Hello, World!" > file1.txt
echo "Another file" > file2.txt
git add .
git commit -m "Initial commit" > /dev/null 2>&1

# Create an untracked file
echo "Untracked content" > untracked.txt

# Create a .gitignore file and an ignored file
echo "ignored.txt" > .gitignore
echo "This should remain" > ignored.txt
git add .gitignore
git commit -m "Add gitignore" > /dev/null 2>&1

# Verify initial state
if [ ! -d ".git" ] || [ ! -f "file1.txt" ] || [ ! -f "file2.txt" ] || [ ! -f "untracked.txt" ] || [ ! -f "ignored.txt" ]; then
    echo "Initial setup failed"
    exit 1
fi

# Run shtt wipe --include-untracked
"$SHTT_BINARY" wipe --include-untracked

# Check that .git directory is gone
if [ -d ".git" ]; then
    echo "ERROR: .git directory still exists"
    exit 1
fi

# Check that tracked files are gone
if [ -f "file1.txt" ] || [ -f "file2.txt" ] || [ -f ".gitignore" ]; then
    echo "ERROR: Tracked files still exist"
    exit 1
fi

# Check that untracked file is also gone
if [ -f "untracked.txt" ]; then
    echo "ERROR: Untracked file still exists when it should have been removed"
    exit 1
fi

# Check that ignored file remains
if [ ! -f "ignored.txt" ]; then
    echo "ERROR: Ignored file was removed when it should have been preserved"
    exit 1
fi
