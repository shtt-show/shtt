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
mkdir subdir
echo "Nested file" > subdir/file3.txt
git add .
git commit -m "Initial commit" > /dev/null 2>&1

# Create an untracked file
echo "Untracked content" > untracked.txt

# Verify initial state
if [ ! -d ".git" ] || [ ! -f "file1.txt" ] || [ ! -f "file2.txt" ] || [ ! -f "subdir/file3.txt" ]; then
    echo "Initial setup failed"
    exit 1
fi

# Run shtt wipe (should remove tracked files but keep untracked)
"$SHTT_BINARY" wipe

# Check that .git directory is gone
if [ -d ".git" ]; then
    echo "ERROR: .git directory still exists"
    exit 1
fi

# Check that tracked files are gone
if [ -f "file1.txt" ] || [ -f "file2.txt" ] || [ -f "subdir/file3.txt" ]; then
    echo "ERROR: Tracked files still exist"
    exit 1
fi

# Check that untracked file remains (since --include-untracked was not used)
if [ ! -f "untracked.txt" ]; then
    echo "ERROR: Untracked file was removed when it shouldn't have been"
    exit 1
fi
