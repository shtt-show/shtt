# Dump Command

The `dump` subcommand lists all changes in the current working directory using the native Rust `git2` library in porcelain format.

## Basic Usage

```bash
 Show all changes (modified + untracked files) in porcelain format
shtt dump
```

## Output Format

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


