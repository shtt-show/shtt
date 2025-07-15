use anyhow::{Context, Result};
use git2::{IndexAddOption};
use shtt::git_utils::{get_signature, push_to_origin, get_current_branch, open_repository};
use shtt::format_utils::format_ordinal;
use shtt::repo_utils::count_commits;

/// Save changes by committing and pushing to origin
pub fn save_changes(message: Option<String>) -> Result<()> {
    let repo = open_repository()?;

    // Check if there are any changes to commit
    let statuses = repo.statuses(None)
        .context("Failed to get repository status")?;

    if statuses.is_empty() {
        println!("No changes to save.");
        return Ok(());
    }

    // Stage all changes (tracked and untracked files)
    let mut index = repo.index()
        .context("Failed to get repository index")?;
    
    index.add_all(["*"].iter(), IndexAddOption::DEFAULT, None)
        .context("Failed to stage changes")?;
    
    index.write()
        .context("Failed to write index")?;

    // Get commit message
    let commit_message = if let Some(msg) = message {
        msg
    } else {
        // Count existing commits and generate auto-numbered message
        let commit_count = count_commits(&repo)?;
        let next_commit_number = commit_count + 1;
        format!("{} Commit", format_ordinal(next_commit_number))
    };

    // Create commit - this will handle interactive git config setup if needed
    let signature = get_signature(&repo)?;
    let tree_id = index.write_tree()
        .context("Failed to write tree")?;
    let tree = repo.find_tree(tree_id)
        .context("Failed to find tree")?;

    let parent_commit = match repo.head() {
        Ok(head) => Some(head.peel_to_commit().context("Failed to get HEAD commit")?),
        Err(_) => None, // First commit
    };

    let parents: Vec<_> = parent_commit.iter().collect();
    
    let commit_id = repo.commit(
        Some("HEAD"),
        &signature,
        &signature,
        &commit_message,
        &tree,
        &parents,
    ).context("Failed to create commit")?;

    println!("Created commit: {}", commit_id);

    // Push to origin using the refactored function
    let branch_name = get_current_branch(&repo)?;
    push_to_origin(&repo, &branch_name)?;

    Ok(())
}