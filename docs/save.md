# Save Command

The `save` subcommand commits all changes (both tracked and untracked files) and pushes them to the origin remote.

## Basic Usage

```bash
## Save with a commit message
shtt save --message "Your commit message"

## Save with auto-generated commit message
shtt save
```

## Command Options

- `--message` / `-m`: Provide the commit message directly. If not provided, an auto-numbered commit message will be generated (e.g., "1st Commit", "2nd Commit", etc.)

## Examples

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

