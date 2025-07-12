use anyhow::{Context, Result};
use git2::{Repository, Status, StatusOptions};
use std::fs;
use std::path::Path;

/// Wipe all git history and tracked files from the current repository
pub fn wipe_repository(include_untracked: bool) -> Result<()> {
    // Check if we're in a git repository
    let repo = Repository::open(".")
        .context("Failed to open git repository. Are you in a git repository?")?;

    // Get the repository root path
    let repo_path = repo.workdir()
        .context("Failed to get repository working directory")?;

    // Get all tracked files before we remove .git
    let tracked_files = get_tracked_files(&repo)?;
    
    // Get untracked files if requested
    let untracked_files = if include_untracked {
        get_untracked_files(&repo)?
    } else {
        Vec::new()
    };

    // Remove all tracked files
    for file_path in &tracked_files {
        let full_path = repo_path.join(file_path);
        if full_path.exists() {
            if full_path.is_file() {
                fs::remove_file(&full_path)
                    .with_context(|| format!("Failed to remove file: {}", file_path))?;
            } else if full_path.is_dir() {
                fs::remove_dir_all(&full_path)
                    .with_context(|| format!("Failed to remove directory: {}", file_path))?;
            }
        }
    }

    // Remove untracked files if requested
    if include_untracked {
        for file_path in &untracked_files {
            let full_path = repo_path.join(file_path);
            if full_path.exists() {
                if full_path.is_file() {
                    fs::remove_file(&full_path)
                        .with_context(|| format!("Failed to remove untracked file: {}", file_path))?;
                } else if full_path.is_dir() {
                    fs::remove_dir_all(&full_path)
                        .with_context(|| format!("Failed to remove untracked directory: {}", file_path))?;
                }
            }
        }
    }

    // Remove .git directory (this removes all git history)
    let git_dir = repo_path.join(".git");
    if git_dir.exists() {
        fs::remove_dir_all(&git_dir)
            .context("Failed to remove .git directory")?;
    }

    // Remove empty directories
    remove_empty_directories(repo_path)?;

    Ok(())
}

fn get_tracked_files(repo: &Repository) -> Result<Vec<String>> {
    let mut tracked_files = Vec::new();
    
    // Get the index to find all tracked files
    let index = repo.index()?;
    
    for entry in index.iter() {
        if let Some(path) = std::str::from_utf8(&entry.path).ok() {
            tracked_files.push(path.to_string());
        }
    }
    
    Ok(tracked_files)
}

fn get_untracked_files(repo: &Repository) -> Result<Vec<String>> {
    let mut untracked_files = Vec::new();
    
    let mut opts = StatusOptions::new();
    opts.include_untracked(true);
    opts.include_ignored(false);
    
    let statuses = repo.statuses(Some(&mut opts))?;
    
    for entry in statuses.iter() {
        let status = entry.status();
        if status.contains(Status::WT_NEW) {
            if let Some(path) = entry.path() {
                untracked_files.push(path.to_string());
            }
        }
    }
    
    Ok(untracked_files)
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