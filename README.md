## shtt
Simple History Tracking Tool

## Usage

### Dump Command

The `dump` subcommand lists all changes in the current working
directory using the native Rust `git2` library.

#### Basic Usage

```bash
## Show all changes (modified + untracked files)
shtt dump

## Show only modified files (exclude untracked)
shtt dump --modified-only

## Machine-readable output format
shtt dump --porcelain
```

#### Command Options

- `--modified-only` / `-m`: Show only files that have been
  modified, added, deleted, or renamed. Excludes untracked
files.
- `--porcelain` / `-p`: Output in machine-readable format
  suitable for scripts.

#### Output Format

##### Human-readable format (default):
```
Changes in current working directory:
=====================================
 M  src/main.rs
A   src/new_file.rs
??  untracked_file.txt

Legend:
  M  = Modified
  A  = Added
  D  = Deleted
  R  = Renamed
  C  = Copied
  T  = Type changed
  ?? = Untracked
```

##### Porcelain format (`--porcelain`):
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

The `wipe` subcommand removes all tracked files and git history from the repository (DESTRUCTIVE operation).

#### Basic Usage

```bash
## Remove all tracked files and git history
shtt wipe

## Also remove untracked files (only ignored files remain)
shtt wipe --include-untracked
```

#### Command Options

- `--include-untracked` / `-i`: Also remove untracked files. Ignored files (those in .gitignore) will still remain.

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
## Basic usage - show all changes
$ shtt dump
Changes in current working directory:
=====================================
 M  README.md
??  new_feature.rs

## Only modified files
$ shtt dump --modified-only
Changes in current working directory:
=====================================
 M  README.md

## Script-friendly output
$ shtt dump --porcelain
 M README.md
?? new_feature.rs

## Save changes
$ shtt save -m "Update documentation"
Created commit: 1a2b3c4d5e6f789...
Pushed changes to origin/main

## No changes
$ shtt dump
No changes detected in the working directory.
```

#### Error Handling

- If not in a git repository: Returns error message "Failed to
  open git repository. Are you in a git repository?"
- If git2 operations fail: Returns appropriate error message
  with context
- Clean error messages help identify the issue quickly

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