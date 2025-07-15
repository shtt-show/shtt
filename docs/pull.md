# Pull Command

The `pull` subcommand pulls latest changes from origin if you're in a git repository, or clones a repository if you provide a remote URL and you're not in a git repository.

## Basic Usage

```bash
## Pull changes when in a git repository
shtt pull

## Clone a repository when not in a git repository
shtt pull <remote-url>
```

## Examples

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
