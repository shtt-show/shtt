# Wipe Command

The `wipe` subcommand resets the repository to match the state of `origin/<current_branch>` and removes all untracked files. This is equivalent to deleting the repository and cloning it fresh from origin.

## Basic Usage

```bash
## Reset to origin state and remove all untracked files
shtt wipe
```

## What it does

1. **Hard reset** to `origin/<current_branch>` - removes any local commits ahead of origin
2. **Removes all untracked files** - any files not tracked by git (except ignored files)
3. **Removes empty directories** - cleans up any empty folders left behind
4. **Preserves ignored files** - files listed in `.gitignore` are not removed

This is equivalent to:
```bash
rm -rf my-repo
git clone <origin-url> my-repo
cd my-repo
```

## Examples

```bash
## Reset everything to match origin
$ shtt wipe
Reset repository to match origin/main
Removed all untracked files and directories

## When you have local commits ahead of origin
$ git log --oneline origin/main..HEAD
abc1234 Local commit 2
def5678 Local commit 1

$ shtt wipe
Reset repository to match origin/main
Removed all untracked files and directories

$ git log --oneline origin/main..HEAD
# (no output - local commits are gone)
```
