use anyhow::{Context, Result};
use git2::{Repository, Status, StatusOptions};

/// Dump changes using native Rust git2 implementation
pub fn dump_changes(modified_only: bool, porcelain: bool) -> Result<()> {
    let repo = Repository::open(".")
        .context("Failed to open git repository. Are you in a git repository?")?;

    let mut opts = StatusOptions::new();
    opts.include_untracked(!modified_only);
    opts.include_ignored(false);

    let statuses = repo.statuses(Some(&mut opts))
        .context("Failed to get repository status")?;

    if statuses.is_empty() {
        println!("No changes detected in the working directory.");
        return Ok(());
    }

    if !porcelain {
        println!("Changes in current working directory:");
        println!("=====================================");
    }

    for entry in statuses.iter() {
        let status = entry.status();
        let path = entry.path().unwrap_or("<unknown>");

        if porcelain {
            // Porcelain format: XY filename
            let index_status = get_index_status_char(status);
            let worktree_status = get_worktree_status_char(status);
            println!("{}{} {}", index_status, worktree_status, path);
        } else {
            // Human-readable format
            let status_str = format_status(status);
            println!("{:3} {}", status_str, path);
        }
    }

    if !porcelain {
        print_legend();
    }

    Ok(())
}

fn print_legend() {
    println!("\nLegend:");
    println!("  M  = Modified");
    println!("  A  = Added");
    println!("  D  = Deleted");
    println!("  R  = Renamed");
    println!("  C  = Copied");
    println!("  T  = Type changed");
    println!("  ?? = Untracked");
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

fn format_status(status: Status) -> String {
    let mut result = String::new();

    // Check index status first
    if status.contains(Status::INDEX_NEW) {
        result.push('A');
    } else if status.contains(Status::INDEX_MODIFIED) {
        result.push('M');
    } else if status.contains(Status::INDEX_DELETED) {
        result.push('D');
    } else if status.contains(Status::INDEX_RENAMED) {
        result.push('R');
    } else if status.contains(Status::INDEX_TYPECHANGE) {
        result.push('T');
    } else {
        result.push(' ');
    }

    // Check worktree status
    if status.contains(Status::WT_NEW) {
        result.push('?');
    } else if status.contains(Status::WT_MODIFIED) {
        result.push('M');
    } else if status.contains(Status::WT_DELETED) {
        result.push('D');
    } else if status.contains(Status::WT_RENAMED) {
        result.push('R');
    } else if status.contains(Status::WT_TYPECHANGE) {
        result.push('T');
    } else {
        result.push(' ');
    }

    // Handle untracked files specially
    if result == "  " && status.contains(Status::WT_NEW) {
        result = "??".to_string();
    }

    result
}