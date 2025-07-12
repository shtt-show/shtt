#!/bin/bash

set -e

# Create a temporary directory for the "remote" repo
REMOTE_DIR=$(mktemp -d)
# Create a temporary directory for the "local" repo
LOCAL_DIR=$(mktemp -d)
# Create a directory for testing clone
CLONE_DIR=$(mktemp -d)

# Cleanup function
cleanup() {
    cd /
    rm -rf "$REMOTE_DIR" "$LOCAL_DIR" "$CLONE_DIR"
}
trap cleanup EXIT

# Set up a "remote" repository
cd "$REMOTE_DIR"
git init --bare > /dev/null 2>&1
git symbolic-ref HEAD refs/heads/trunk > /dev/null 2>&1

# Set up local repository and push initial state
cd "$LOCAL_DIR"
git init > /dev/null 2>&1
git config user.email "test@example.com"
git config user.name "Test User"
git config init.defaultBranch trunk
git checkout -b trunk > /dev/null 2>&1
git remote add origin "$REMOTE_DIR"

# Create and commit initial files
echo "Initial content" > file1.txt
git add .
git commit -m "Initial commit" > /dev/null 2>&1
git push -u origin trunk > /dev/null 2>&1

# Make a new commit and push it
echo "Updated content" > file1.txt
echo "New file" > file2.txt
git add .
git commit -m "Update files" > /dev/null 2>&1
git push origin trunk > /dev/null 2>&1

# Reset local repo to the first commit to simulate being behind
git reset --hard HEAD~1 > /dev/null 2>&1

# Verify we're behind origin
COMMITS_BEHIND=$(git rev-list --count HEAD..origin/trunk)
if [ "$COMMITS_BEHIND" -eq "0" ]; then
    echo "ERROR: Expected to be behind origin"
    exit 1
fi

# Test pull in existing repository
OUTPUT=$("$SHTT_BINARY" pull 2>&1)

if echo "$OUTPUT" | grep -q "Fast-forwarding to origin/trunk"; then
    # Verify we're now up to date
    if ! git diff --quiet origin/trunk; then
        echo "ERROR: Repository is not up to date after pull"
        exit 1
    fi
    
    # Verify file contents
    if [ "$(cat file1.txt)" != "Updated content" ] || [ "$(cat file2.txt)" != "New file" ]; then
        echo "ERROR: Files were not updated correctly"
        exit 1
    fi
    
else
    echo "ERROR: Expected to see fast-forward message in output:"
    echo "$OUTPUT"
    exit 1
fi

# Test that providing remote URL in existing repo fails
if "$SHTT_BINARY" pull "$REMOTE_DIR" 2>&1 | grep -q "Already in a git repository"; then
    true
else
    echo "ERROR: Should have rejected remote URL when already in a repository"
    exit 1
fi

# Test clone functionality
cd "$CLONE_DIR"

# Extract the repository name for the expected directory
REPO_NAME="$(basename "$REMOTE_DIR")"

OUTPUT=$("$SHTT_BINARY" pull "$REMOTE_DIR" 2>&1)

if echo "$OUTPUT" | grep -q "Repository cloned successfully"; then
    # Check that the directory was created
    if [ ! -d "$REPO_NAME" ]; then
        echo "ERROR: Repository directory was not created"
        exit 1
    fi
    
    # Check that it's a valid git repository with correct content
    cd "$REPO_NAME"
    if [ ! -d ".git" ]; then
        echo "ERROR: Cloned directory is not a git repository"
        exit 1
    fi
    
    # Verify file contents
    if [ "$(cat file1.txt)" != "Updated content" ] || [ "$(cat file2.txt)" != "New file" ]; then
        echo "ERROR: Cloned files do not have correct content"
        exit 1
    fi
    
else
    echo "ERROR: Expected to see clone success message in output:"
    echo "$OUTPUT"
    exit 1
fi

# Test that not providing remote URL outside repo fails
cd "$CLONE_DIR"
rm -rf "$REPO_NAME"

if "$SHTT_BINARY" pull 2>&1 | grep -q "Please provide a remote URL to clone"; then
    true
else
    echo "ERROR: Should have required remote URL when not in a repository"
    exit 1
fi
