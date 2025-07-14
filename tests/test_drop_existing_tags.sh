#!/bin/bash

set -e

# Create temporary directories for the "remote" repo and "local" repo
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

# Create and commit an initial file
echo "Initial content" > initial.txt
git add initial.txt
git commit -m "Initial commit" > /dev/null 2>&1

# Push initial commit to origin
git push -u origin trunk > /dev/null 2>&1

# Create some existing tags manually and push them
git tag v1.5.2 > /dev/null 2>&1
git tag v1.4.0 > /dev/null 2>&1
git tag v2.0.0 > /dev/null 2>&1
git tag v0.9.1 > /dev/null 2>&1

# Push all tags to origin
git push origin --tags > /dev/null 2>&1

# Test that drop finds the highest existing tag (v2.0.0) and increments correctly

# Test patch bump (should create v2.0.1)
OUTPUT=$("$SHTT_BINARY" drop patch 2>&1)

if echo "$OUTPUT" | grep -q "Created tag: v2.0.1" && echo "$OUTPUT" | grep -q "Pushed tag v2.0.1 to origin"; then
    # Verify tag exists locally
    if git tag -l | grep -q "v2.0.1"; then
        # Verify tag was pushed to origin by fetching and checking
        git fetch origin > /dev/null 2>&1
        if git tag -l | grep -q "v2.0.1"; then
            true
        else
            echo "✗ Tag v2.0.1 was not pushed to origin"
            exit 1
        fi
    else
        echo "✗ Tag v2.0.1 was not created locally"
        exit 1
    fi
else
    echo "✗ Expected to see 'Created tag: v2.0.1' and 'Pushed tag v2.0.1 to origin' in output:"
    echo "$OUTPUT"
    exit 1
fi

# Test minor bump (should create v2.1.0)
OUTPUT=$("$SHTT_BINARY" drop minor 2>&1)

if echo "$OUTPUT" | grep -q "Created tag: v2.1.0" && echo "$OUTPUT" | grep -q "Pushed tag v2.1.0 to origin"; then
    # Verify tag exists locally and was pushed
    if git tag -l | grep -q "v2.1.0"; then
        git fetch origin > /dev/null 2>&1
        if git tag -l | grep -q "v2.1.0"; then
            true
        else
            echo "✗ Tag v2.1.0 was not pushed to origin"
            exit 1
        fi
    else
        echo "✗ Tag v2.1.0 was not created locally"
        exit 1
    fi
else
    echo "✗ Expected to see 'Created tag: v2.1.0' and 'Pushed tag v2.1.0 to origin' in output:"
    echo "$OUTPUT"
    exit 1
fi

# Test major bump (should create v3.0.0)
OUTPUT=$("$SHTT_BINARY" drop major 2>&1)

if echo "$OUTPUT" | grep -q "Created tag: v3.0.0" && echo "$OUTPUT" | grep -q "Pushed tag v3.0.0 to origin"; then
    # Verify tag exists locally and was pushed
    if git tag -l | grep -q "v3.0.0"; then
        git fetch origin > /dev/null 2>&1
        if git tag -l | grep -q "v3.0.0"; then
            true
        else
            echo "✗ Tag v3.0.0 was not pushed to origin"
            exit 1
        fi
    else
        echo "✗ Tag v3.0.0 was not created locally"
        exit 1
    fi
else
    echo "✗ Expected to see 'Created tag: v3.0.0' and 'Pushed tag v3.0.0 to origin' in output:"
    echo "$OUTPUT"
    exit 1
fi

# Verify that non-semver tags are ignored
git tag random-tag > /dev/null 2>&1
git tag not-a-version > /dev/null 2>&1
git push origin --tags > /dev/null 2>&1

# Should still use v3.0.0 as the highest
OUTPUT=$("$SHTT_BINARY" drop patch 2>&1)

if echo "$OUTPUT" | grep -q "Created tag: v3.0.1" && echo "$OUTPUT" | grep -q "Pushed tag v3.0.1 to origin"; then
    true
else
    echo "✗ Expected to see 'Created tag: v3.0.1' and push confirmation (non-semver tags should be ignored):"
    echo "$OUTPUT"
    exit 1
fi