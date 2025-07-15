use anyhow::{Context, Result};
use git2::{Repository, Status, StatusOptions, ResetType, BranchType};
use std::fs;
use std::path::Path;
use shtt::git_utils::open_repository;

/// Reset repository to match the state of origin/<current_branch>
pub fn wipe_repository(_include_untracked: bool) -> Result<()> {
    let repo = open_repository()?;

    // Get the current branch name
    let head = repo.head()
        .context("Failed to get HEAD reference. Make sure you're on a branch.")?;
    
    let current_branch = head.shorthand()
        .context("Failed to get current branch name")?;

    // Find the corresponding remote branch
    let remote_branch_name = format!("origin/{}", current_branch);
    let remote_branch = repo.find_branch(&remote_branch_name, BranchType::Remote)
        .with_context(|| format!("Failed to find remote branch '{}'. Make sure you have fetched from origin.", remote_branch_name))?;

    let remote_commit = remote_branch.get().peel_to_commit()
        .context("Failed to get commit from remote branch")?;

    // Perform a hard reset to the remote branch commit
    repo.reset(&remote_commit.as_object(), ResetType::Hard, None)
        .context("Failed to reset to remote branch")?;

    // Remove all untracked files and directories
    remove_all_untracked_files(&repo)?;

    // Remove empty directories
    let repo_path = repo.workdir()
        .context("Failed to get repository working directory")?;
    remove_empty_directories(repo_path)?;

    println!("Reset repository to match {}", remote_branch_name);
    println!("Removed all untracked files and directories");

    Ok(())
}

fn remove_all_untracked_files(repo: &Repository) -> Result<()> {
    let mut opts = StatusOptions::new();
    opts.include_untracked(true);
    opts.include_ignored(false); // Don't remove ignored files
    
    let statuses = repo.statuses(Some(&mut opts))?;
    let repo_path = repo.workdir()
        .context("Failed to get repository working directory")?;
    
    for entry in statuses.iter() {
        let status = entry.status();
        if status.contains(Status::WT_NEW) {
            if let Some(path) = entry.path() {
                let full_path = repo_path.join(path);
                if full_path.exists() {
                    if full_path.is_file() {
                        fs::remove_file(&full_path)
                            .with_context(|| format!("Failed to remove untracked file: {}", path))?;
                    } else if full_path.is_dir() {
                        fs::remove_dir_all(&full_path)
                            .with_context(|| format!("Failed to remove untracked directory: {}", path))?;
                    }
                }
            }
        }
    }
    
    Ok(())
}

fn remove_empty_directories(root: &Path) -> Result<()> {
    fn remove_empty_dirs_recursive(dir: &Path, root: &Path) -> Result<bool> {
        if !dir.is_dir() {
            return Ok(false);
        }
        
        let mut is_empty = true;
        
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.is_dir() {
                // Skip .git directory
                if path.file_name().and_then(|n| n.to_str()) == Some(".git") {
                    is_empty = false;
                    continue;
                }
                
                // Recursively check subdirectories
                if !remove_empty_dirs_recursive(&path, root)? {
                    is_empty = false;
                }
            } else {
                // Found a file, directory is not empty
                is_empty = false;
            }
        }
        
        // If directory is empty, remove it (but don't remove the root directory)
        if is_empty && dir != root {
            fs::remove_dir(dir)?;
            return Ok(true);
        }
        
        Ok(false)
    }
    
    remove_empty_dirs_recursive(root, root)?;
    Ok(())
}