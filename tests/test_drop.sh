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

# Test creating first tag (should be v0.0.1 when no tags exist and we bump patch)
OUTPUT=$("$SHTT_BINARY" drop patch 2>&1)

if echo "$OUTPUT" | grep -q "Created tag: v0.0.1" && echo "$OUTPUT" | grep -q "Pushed tag v0.0.1 to origin"; then
    # Verify tag exists locally
    if git tag -l | grep -q "v0.0.1"; then
        # Verify tag was pushed to origin by fetching and checking
        git fetch origin > /dev/null 2>&1
        if git tag -l | grep -q "v0.0.1"; then
            true
        else
            echo "✗ Tag v0.0.1 was not pushed to origin"
            exit 1
        fi
    else
        echo "✗ Tag v0.0.1 was not created locally"
        exit 1
    fi
else
    echo "✗ Expected to see 'Created tag: v0.0.1' and 'Pushed tag v0.0.1 to origin' in output:"
    echo "$OUTPUT"
    exit 1
fi

# Test patch bump (should create v0.0.2)
OUTPUT=$("$SHTT_BINARY" drop patch 2>&1)

if echo "$OUTPUT" | grep -q "Created tag: v0.0.2" && echo "$OUTPUT" | grep -q "Pushed tag v0.0.2 to origin"; then
    # Verify tag exists locally and was pushed
    if git tag -l | grep -q "v0.0.2"; then
        git fetch origin > /dev/null 2>&1
        if git tag -l | grep -q "v0.0.2"; then
            true
        else
            echo "✗ Tag v0.0.2 was not pushed to origin"
            exit 1
        fi
    else
        echo "✗ Tag v0.0.2 was not created locally"
        exit 1
    fi
else
    echo "✗ Expected to see 'Created tag: v0.0.2' and 'Pushed tag v0.0.2 to origin' in output:"
    echo "$OUTPUT"
    exit 1
fi

# Test minor bump (should create v0.1.0)
OUTPUT=$("$SHTT_BINARY" drop minor 2>&1)

if echo "$OUTPUT" | grep -q "Created tag: v0.1.0" && echo "$OUTPUT" | grep -q "Pushed tag v0.1.0 to origin"; then
    # Verify tag exists and patch is reset
    if git tag -l | grep -q "v0.1.0"; then
        git fetch origin > /dev/null 2>&1
        if git tag -l | grep -q "v0.1.0"; then
            true
        else
            echo "✗ Tag v0.1.0 was not pushed to origin"
            exit 1
        fi
    else
        echo "✗ Tag v0.1.0 was not created locally"
        exit 1
    fi
else
    echo "✗ Expected to see 'Created tag: v0.1.0' and 'Pushed tag v0.1.0 to origin' in output:"
    echo "$OUTPUT"
    exit 1
fi

# Test major bump (should create v1.0.0)
OUTPUT=$("$SHTT_BINARY" drop major 2>&1)

if echo "$OUTPUT" | grep -q "Created tag: v1.0.0" && echo "$OUTPUT" | grep -q "Pushed tag v1.0.0 to origin"; then
    # Verify tag exists and minor/patch are reset
    if git tag -l | grep -q "v1.0.0"; then
        git fetch origin > /dev/null 2>&1
        if git tag -l | grep -q "v1.0.0"; then
            true
        else
            echo "✗ Tag v1.0.0 was not pushed to origin"
            exit 1
        fi
    else
        echo "✗ Tag v1.0.0 was not created locally"
        exit 1
    fi
else
    echo "✗ Expected to see 'Created tag: v1.0.0' and 'Pushed tag v1.0.0 to origin' in output:"
    echo "$OUTPUT"
    exit 1
fi

# Test invalid bump type
OUTPUT=$("$SHTT_BINARY" drop invalid 2>&1 || true)

if echo "$OUTPUT" | grep -q "error\|invalid"; then
    true
else
    echo "✗ Expected error for invalid bump type:"
    echo "$OUTPUT"
    exit 1
fi

# Verify all tags are in correct order
git fetch origin > /dev/null 2>&1
TAGS=$(git tag -l | sort -V)
EXPECTED="v0.0.1
v0.0.2
v0.1.0
v1.0.0"

if [ "$TAGS" = "$EXPECTED" ]; then
    true
else
    echo "✗ Tags not in expected order:"
    echo "Expected:"
    echo "$EXPECTED"
    echo "Actual:"
    echo "$TAGS"
    exit 1
fi
