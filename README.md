## shtt
Simple History Tracking Tool

## Usage

### Dump Command

The `dump` subcommand lists all changes in the current working
directory using the native Rust `git2` library in porcelain format.

#### Basic Usage

```bash
## Show all changes (modified + untracked files) in porcelain format
shtt dump
```

#### Output Format

The output is always in porcelain (machine-readable) format:
```
 M src/main.rs
A  src/new_file.rs
?? untracked_file.txt
```

### Save Command

The `save` subcommand commits all changes (both tracked and untracked files) and pushes them to the origin remote.

#### Basic Usage

```bash
## Save with a commit message
shtt save --message "Your commit message"

## Save and be prompted for a commit message
shtt save
```

#### Command Options

- `--message` / `-m`: Provide the commit message directly. If not provided, you'll be prompted to enter one interactively.

#### Examples

```bash
## Save with a specific message
$ shtt save -m "Add new feature"
Created commit: a1b2c3d4e5f6789...
Pushed changes to origin/main

## Save and be prompted for message
$ shtt save
Enter commit message: Fix bug in validation
Created commit: f6e5d4c3b2a1987...
Pushed changes to origin/main

## No changes to save
$ shtt save
No changes to save.
```

#### Error Handling

- If not in a git repository: Returns error message "Failed to open git repository. Are you in a git repository?"
- If git user.name or user.email not configured: Returns helpful error message with configuration commands
- If no origin remote exists: Returns error message about missing origin remote
- If push fails due to permissions: Returns error about push access
- If commit message is empty when prompted: Returns error about empty commit message

### Wipe Command

The `wipe` subcommand resets the repository to match the state of `origin/<current_branch>` and removes all untracked files. This is equivalent to deleting the repository and cloning it fresh from origin.

#### Basic Usage

```bash
## Reset to origin state and remove all untracked files
shtt wipe
```

#### What it does

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

#### Examples

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

#### Error Handling

- If not in a git repository: Returns error message "Failed to open git repository. Are you in a git repository?"
- If not on a branch: Returns error message about HEAD reference
- If no corresponding origin branch exists: Returns error message about missing remote branch
- If git2 operations fail: Returns appropriate error message with context

#### Build Instructions

```bash
## Build the project
cargo build

## Build in release mode
cargo build --release

## Run directly with cargo
cargo run -- dump
cargo run -- save -m "commit message"
cargo run -- wipe
```

#### Examples

```bash
## Show all changes in porcelain format
$ shtt dump
 M README.md
?? new_feature.rs

## Save changes
$ shtt save -m "Update documentation"
Created commit: 1a2b3c4d5e6f789...
Pushed changes to origin/main

## Reset to origin state
$ shtt wipe
Reset repository to match origin/main
Removed all untracked files and directories

## No changes after wipe
$ shtt dump
No changes detected in the working directory.
```

#### Status Code Legend

The output uses Git's standard status codes:

- **Index Status** (first character):
  - `A` = Added to index
  - `M` = Modified in index  
  - `D` = Deleted from index
  - `R` = Renamed in index
  - `T` = Type changed in index
  - ` ` = No change in index

- **Working Tree Status** (second character):
  - `M` = Modified in working tree
  - `D` = Deleted in working tree
  - `R` = Renamed in working tree
  - `T` = Type changed in working tree
  - `?` = Untracked file
  - ` ` = No change in working tree

- **Special Cases**:
  - `??` = Untracked file (both characters)