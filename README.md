# shtt
*Simple History Tracking Tool*

## Installation

```bash
cargo install --git https://github.com/shtt-show/shtt
```

## Commands

### Dump Command

The `dump` subcommand lists all changes in the current working directory using the native Rust `git2` library in porcelain format.

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

The first column shows index status, the second column shows working tree status:
- `M` = Modified
- `A` = Added
- `D` = Deleted
- `R` = Renamed
- `??` = Untracked
- `  ` = No changes

### Save Command

The `save` subcommand commits all changes (both tracked and untracked files) and pushes them to the origin remote.

#### Basic Usage

```bash
## Save with a commit message
shtt save --message "Your commit message"

## Save with auto-generated commit message
shtt save
```

#### Command Options

- `--message` / `-m`: Provide the commit message directly. If not provided, an auto-numbered commit message will be generated (e.g., "1st Commit", "2nd Commit", etc.)

#### Examples

```bash
## Save with a specific message
$ shtt save -m "Add new feature"
Created commit: a1b2c3d4e5f6789...
Pushed changes to origin/main

## Save with auto-generated message
$ shtt save
Created commit: f6e5d4c3b2a1987...
Pushed changes to origin/main

## No changes to save
$ shtt save
No changes to save.
```

### Pull Command

The `pull` subcommand pulls latest changes from origin if you're in a git repository, or clones a repository if you provide a remote URL and you're not in a git repository.

#### Basic Usage

```bash
## Pull changes when in a git repository
shtt pull

## Clone a repository when not in a git repository
shtt pull <remote-url>
```

#### Examples

```bash
## Pull latest changes
$ shtt pull
Fetching from origin...
Fast-forwarding to origin/main...
Successfully updated to abc123...

## Clone a repository
$ shtt pull git@github.com:user/repo.git
Cloning git@github.com:user/repo.git into repo...
Repository cloned successfully into 'repo'
Switched to branch 'main'
```

### Drop Command

The `drop` subcommand creates and pushes semantic version tags. It automatically finds the highest existing semver tag and increments it according to the specified bump type.

#### Basic Usage

```bash
## Create a patch version bump (e.g., v1.2.3 → v1.2.4)
shtt drop patch

## Create a minor version bump (e.g., v1.2.3 → v1.3.0)
shtt drop minor

## Create a major version bump (e.g., v1.2.3 → v2.0.0)
shtt drop major
```

#### Examples

```bash
## First tag when no tags exist
$ shtt drop patch
Created tag: v0.0.1
Pushed tag v0.0.1 to origin

## Increment patch version
$ shtt drop patch
Created tag: v1.2.4
Pushed tag v1.2.4 to origin

## Increment minor version (resets patch to 0)
$ shtt drop minor
Created tag: v1.3.0
Pushed tag v1.3.0 to origin

## Increment major version (resets minor and patch to 0)
$ shtt drop major
Created tag: v2.0.0
Pushed tag v2.0.0 to origin
```

**Note**: Only semver-compatible tags (e.g., `v1.2.3`, `1.0.0`) are considered when finding the highest version. Non-semver tags are ignored.

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

## Configuration

SHTT uses your existing git configuration for user name, email, and authentication. Make sure you have these configured:

```bash
git config user.name "Your Name"
git config user.email "your.email@example.com"
```

For authentication, SHTT supports:
- SSH keys (via ssh-agent)
- Git credential helpers
- Default git credentials

## Error Handling

SHTT provides clear error messages for common issues:
- Not in a git repository
- Missing origin remote
- Uncommitted changes when pulling
- Authentication failures
- Missing git configuration

## License

This project is licensed under the Mozilla Public License 2.0. See the [LICENSE](LICENSE) file for details.