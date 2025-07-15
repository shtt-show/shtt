use anyhow::{Context, Result};
use git2::{Repository};
use git2::Status;

/// Count the total number of commits in the repository
pub fn count_commits(repo: &Repository) -> Result<usize> {
    // Try to get the current HEAD
    let head = match repo.head() {
        Ok(head) => head,
        Err(_) => {
            // No HEAD means no commits yet
            return Ok(0);
        }
    };

    // Get the commit that HEAD points to
    let commit = head.peel_to_commit()
        .context("Failed to get commit from HEAD")?;

    // Walk through all commits from HEAD to count them
    let mut revwalk = repo.revwalk()
        .context("Failed to create revision walker")?;
    
    revwalk.push(commit.id())
        .context("Failed to push HEAD commit to revwalk")?;

    // Count all commits
    let count = revwalk.count();
    
    Ok(count)
}

/// Check if repository has any uncommitted changes
pub fn has_uncommitted_changes(repo: &Repository) -> Result<bool> {
    let mut opts = git2::StatusOptions::new();
    opts.include_untracked(true);
    opts.include_ignored(false);
    
    let statuses = repo.statuses(Some(&mut opts))
        .context("Failed to get repository status")?;

    // Check if there are any actual file changes
    let has_changes = statuses.iter().any(|entry| {
        let status = entry.status();
        // Check for any working tree or index changes
        status.intersects(
            Status::INDEX_NEW |
            Status::INDEX_MODIFIED |
            Status::INDEX_DELETED |
            Status::INDEX_RENAMED |
            Status::INDEX_TYPECHANGE |
            Status::WT_NEW |
            Status::WT_MODIFIED |
            Status::WT_DELETED |
            Status::WT_RENAMED |
            Status::WT_TYPECHANGE
        )
    });

    Ok(has_changes)
}
