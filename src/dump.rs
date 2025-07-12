use anyhow::{Context, Result};
use git2::{Repository, Status, StatusOptions};

/// Dump changes using native Rust git2 implementation (porcelain format only)
pub fn dump_changes() -> Result<()> {
    let repo = Repository::open(".")
        .context("Failed to open git repository. Are you in a git repository?")?;

    let mut opts = StatusOptions::new();
    opts.include_untracked(true);
    opts.include_ignored(false);

    let statuses = repo.statuses(Some(&mut opts))
        .context("Failed to get repository status")?;

    if statuses.is_empty() {
        println!("No changes detected in the working directory.");
        return Ok(());
    }

    for entry in statuses.iter() {
        let status = entry.status();
        let path = entry.path().unwrap_or("<unknown>");

        // Porcelain format: XY filename
        let index_status = get_index_status_char(status);
        let worktree_status = get_worktree_status_char(status);
        println!("{}{} {}", index_status, worktree_status, path);
    }

    Ok(())
}

fn get_index_status_char(status: Status) -> char {
    if status.contains(Status::INDEX_NEW) { 'A' }
    else if status.contains(Status::INDEX_MODIFIED) { 'M' }
    else if status.contains(Status::INDEX_DELETED) { 'D' }
    else if status.contains(Status::INDEX_RENAMED) { 'R' }
    else if status.contains(Status::INDEX_TYPECHANGE) { 'T' }
    else { ' ' }
}

fn get_worktree_status_char(status: Status) -> char {
    if status.contains(Status::WT_NEW) { '?' }
    else if status.contains(Status::WT_MODIFIED) { 'M' }
    else if status.contains(Status::WT_DELETED) { 'D' }
    else if status.contains(Status::WT_RENAMED) { 'R' }
    else if status.contains(Status::WT_TYPECHANGE) { 'T' }
    else { ' ' }
}