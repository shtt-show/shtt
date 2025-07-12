#!/bin/bash

set -e

# Create a temporary directory for the "remote" repo
REMOTE_DIR=$(mktemp -d)
# Create a temporary directory for the "local" repo
LOCAL_DIR=$(mktemp -d)

# Cleanup function
cleanup() {
    cd /
    rm -rf "$REMOTE_DIR" "$LOCAL_DIR"
}
trap cleanup EXIT

# Set up a "remote" repository
cd "$REMOTE_DIR"
git init --bare > /dev/null 2>&1
# Set the default branch to trunk in the bare repo
git symbolic-ref HEAD refs/heads/trunk > /dev/null 2>&1

# Set up local repository and push initial state
cd "$LOCAL_DIR"
git init > /dev/null 2>&1
git config user.email "test@example.com"
git config user.name "Test User"
# Ensure we're using trunk as the default branch
git config init.defaultBranch trunk
git checkout -b trunk > /dev/null 2>&1
git remote add origin "$REMOTE_DIR"

# Create and commit initial files
echo "Initial content 1" > file1.txt
echo "Initial content 2" > file2.txt
mkdir subdir
echo "Initial nested" > subdir/file3.txt
git add .
git commit -m "Initial commit" > /dev/null 2>&1
git push -u origin trunk > /dev/null 2>&1

# Make additional commits (these should be wiped)
echo "Local change 1" > file1.txt
echo "New local file" > local_file.txt
git add .
git commit -m "Local commit 1" > /dev/null 2>&1

echo "Local change 2" > file2.txt
git add .
git commit -m "Local commit 2" > /dev/null 2>&1

# Create untracked files (these should be removed)
echo "Untracked content" > untracked.txt
echo "Another untracked" > another_untracked.txt

# Create ignored file
echo "untracked.txt" > .gitignore
git add .gitignore
git commit -m "Add gitignore" > /dev/null 2>&1

# Verify initial state before wipe
if [ ! -f "file1.txt" ] || [ ! -f "file2.txt" ] || [ ! -f "subdir/file3.txt" ] || [ ! -f "untracked.txt" ]; then
    echo "Initial setup failed"
    exit 1
fi

# Check that we have local commits ahead of origin
COMMITS_AHEAD=$(git rev-list --count origin/trunk..HEAD)
if [ "$COMMITS_AHEAD" -eq "0" ]; then
    echo "ERROR: Expected to have commits ahead of origin"
    exit 1
fi

# Run shtt wipe
"$SHTT_BINARY" wipe

# Check that we're now at the same commit as origin/trunk
if ! git diff --quiet origin/trunk; then
    echo "ERROR: Repository is not at the same state as origin/trunk"
    exit 1
fi

# Check that local commits are gone
COMMITS_AHEAD_AFTER=$(git rev-list --count origin/trunk..HEAD)
if [ "$COMMITS_AHEAD_AFTER" -ne "0" ]; then
    echo "ERROR: Still have commits ahead of origin after wipe"
    exit 1
fi

# Check that files match origin state
if [ "$(cat file1.txt)" != "Initial content 1" ]; then
    echo "ERROR: file1.txt was not reset to origin state"
    exit 1
fi

if [ "$(cat file2.txt)" != "Initial content 2" ]; then
    echo "ERROR: file2.txt was not reset to origin state"
    exit 1
fi

if [ "$(cat subdir/file3.txt)" != "Initial nested" ]; then
    echo "ERROR: subdir/file3.txt was not reset to origin state"
    exit 1
fi

# Check that untracked files are gone
if [ -f "untracked.txt" ] || [ -f "another_untracked.txt" ]; then
    echo "ERROR: Untracked files still exist"
    exit 1
fi

# Check that local_file.txt (from local commits) is gone
if [ -f "local_file.txt" ]; then
    echo "ERROR: File from local commits still exists"
    exit 1
fi

# Check that .gitignore is back to origin state (shouldn't contain untracked.txt)
if [ -f ".gitignore" ] && grep -q "untracked.txt" .gitignore; then
    echo "ERROR: .gitignore was not reset to origin state"
    exit 1
fi
