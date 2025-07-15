# Drop Command

The `drop` subcommand creates and pushes semantic version tags. It automatically finds the highest existing semver tag and increments it according to the specified bump type.

## Basic Usage

```bash
## Create a patch version bump (e.g., v1.2.3 → v1.2.4)
shtt drop patch

## Create a minor version bump (e.g., v1.2.3 → v1.3.0)
shtt drop minor

## Create a major version bump (e.g., v1.2.3 → v2.0.0)
shtt drop major
```

## Examples

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
