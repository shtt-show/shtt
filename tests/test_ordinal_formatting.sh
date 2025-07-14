#!/bin/bash

set -e

# Create temporary directories for remote and local repos
REMOTE_DIR=$(mktemp -d)
LOCAL_DIR=$(mktemp -d)

# Cleanup function
cleanup() {
    cd /
    rm -rf "$REMOTE_DIR" "$LOCAL_DIR"
}
trap cleanup EXIT

# Set up a "remote" bare repository
cd "$REMOTE_DIR"
git init --bare > /dev/null 2>&1
git symbolic-ref HEAD refs/heads/trunk > /dev/null 2>&1

# Set up local repository with origin remote
cd "$LOCAL_DIR"
git init > /dev/null 2>&1
git config user.email "test@example.com"
git config user.name "Test User"
git config init.defaultBranch trunk
git checkout -b trunk > /dev/null 2>&1
git remote add origin "$REMOTE_DIR"

# Create a helper function to test ordinal formatting by making commits
test_ordinal() {
    local expected="$1"
    echo "Test commit $expected" > "file_$expected.txt"
    local output=$("$SHTT_BINARY" save 2>&1)
    local actual=$(git log -1 --pretty=format:%s)
    
    if [ "$actual" = "$expected Commit" ]; then
        return 0
    else
        echo "✗ Expected '$expected Commit', got: '$actual'"
        echo "Save output: $output"
        return 1
    fi
}

# Test key ordinal numbers that often cause issues
# Only output when there are problems - silent on success

# Test 1st, 2nd, 3rd
test_ordinal "1st" || exit 1
test_ordinal "2nd" || exit 1  
test_ordinal "3rd" || exit 1
test_ordinal "4th" || exit 1
test_ordinal "5th" || exit 1
test_ordinal "6th" || exit 1
test_ordinal "7th" || exit 1
test_ordinal "8th" || exit 1
test_ordinal "9th" || exit 1
test_ordinal "10th" || exit 1

# Test the tricky teens (11th, 12th, 13th - not 11st, 12nd, 13rd!)
test_ordinal "11th" || exit 1
test_ordinal "12th" || exit 1
test_ordinal "13th" || exit 1
test_ordinal "14th" || exit 1
test_ordinal "15th" || exit 1
test_ordinal "16th" || exit 1
test_ordinal "17th" || exit 1
test_ordinal "18th" || exit 1
test_ordinal "19th" || exit 1

# Back to the normal pattern
test_ordinal "20th" || exit 1
test_ordinal "21st" || exit 1
test_ordinal "22nd" || exit 1
test_ordinal "23rd" || exit 1
