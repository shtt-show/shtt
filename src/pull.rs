use anyhow::{Context, Result};
use git2::{Repository, FetchOptions, RemoteCallbacks, BranchType, ResetType};
use std::path::Path;

/// Pull changes from origin or clone a repository
pub fn pull_changes(remote: Option<String>) -> Result<()> {
    // Check if we're in a git repository
    match Repository::open(".") {
        Ok(repo) => {
            // We're in a repo - do a pull operation
            if remote.is_some() {
                return Err(anyhow::anyhow!(
                    "Already in a git repository. Remote URL argument is not needed for pull operations."
                ));
            }
            pull_from_origin(&repo)
        }
        Err(_) => {
            // We're not in a repo - do a clone operation
            let remote_url = remote.ok_or_else(|| {
                anyhow::anyhow!("Not in a git repository. Please provide a remote URL to clone.")
            })?;
            clone_repository(&remote_url)
        }
    }
}

fn pull_from_origin(repo: &Repository) -> Result<()> {
    // Get the current branch name
    let head = repo.head()
        .context("Failed to get HEAD reference. Make sure you're on a branch.")?;
    
    let current_branch = head.shorthand()
        .context("Failed to get current branch name")?;

    println!("Fetching from origin...");

    // Find the origin remote
    let mut remote = repo.find_remote("origin")
        .context("Failed to find 'origin' remote. Make sure you have an origin remote configured.")?;

    // Set up callbacks for authentication
    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(|_url, username_from_url, allowed_types| {
        // Try credential helper first
        if allowed_types.contains(git2::CredentialType::USER_PASS_PLAINTEXT) {
            if let Ok(cred) = git2::Cred::credential_helper(
                &git2::Config::open_default().unwrap_or_else(|_| git2::Config::new().unwrap()),
                _url,
                username_from_url,
            ) {
                return Ok(cred);
            }
        }
        
        // Try SSH key if available
        if allowed_types.contains(git2::CredentialType::SSH_KEY) {
            if let Some(username) = username_from_url {
                return git2::Cred::ssh_key_from_agent(username);
            }
        }
        
        // Default credentials
        git2::Cred::default()
    });

    let mut fetch_options = FetchOptions::new();
    fetch_options.remote_callbacks(callbacks);

    // Fetch all refs from origin
    remote.fetch(&[] as &[&str], Some(&mut fetch_options), None)
        .context("Failed to fetch from origin. Make sure you have access and the remote is configured correctly.")?;

    println!("Fetch completed successfully.");

    // Find the remote branch
    let remote_branch_name = format!("origin/{}", current_branch);
    let remote_branch = repo.find_branch(&remote_branch_name, BranchType::Remote)
        .with_context(|| format!("Failed to find remote branch '{}'. Make sure the branch exists on origin.", remote_branch_name))?;

    let remote_commit = remote_branch.get().peel_to_commit()
        .context("Failed to get commit from remote branch")?;

    // Check if we're already up to date
    let local_commit = head.peel_to_commit()
        .context("Failed to get local commit")?;

    if local_commit.id() == remote_commit.id() {
        println!("Already up to date.");
        return Ok(());
    }

    // Check if there are local changes (only working tree and index changes, not ahead/behind status)
    let mut opts = git2::StatusOptions::new();
    opts.include_untracked(true);
    opts.include_ignored(false);
    
    let statuses = repo.statuses(Some(&mut opts))
        .context("Failed to get repository status")?;

    // Check if there are any actual file changes (not just branch status)
    let has_changes = statuses.iter().any(|entry| {
        let status = entry.status();
        // Check for any working tree or index changes
        status.intersects(
            git2::Status::INDEX_NEW |
            git2::Status::INDEX_MODIFIED |
            git2::Status::INDEX_DELETED |
            git2::Status::INDEX_RENAMED |
            git2::Status::INDEX_TYPECHANGE |
            git2::Status::WT_NEW |
            git2::Status::WT_MODIFIED |
            git2::Status::WT_DELETED |
            git2::Status::WT_RENAMED |
            git2::Status::WT_TYPECHANGE
        )
    });

    if has_changes {
        return Err(anyhow::anyhow!(
            "Cannot pull with uncommitted changes. Please commit or stash your changes first."
        ));
    }

    // Create an annotated commit for merge analysis
    let annotated_commit = repo.find_annotated_commit(remote_commit.id())
        .context("Failed to create annotated commit for merge analysis")?;

    // Perform a fast-forward merge or reset
    let (analysis, _) = repo.merge_analysis(&[&annotated_commit])
        .context("Failed to perform merge analysis")?;

    if analysis.is_fast_forward() {
        println!("Fast-forwarding to origin/{}...", current_branch);
        
        // Update the branch reference
        let mut branch_ref = repo.find_reference(&format!("refs/heads/{}", current_branch))
            .context("Failed to find local branch reference")?;
        
        branch_ref.set_target(remote_commit.id(), "pull: Fast-forward")
            .context("Failed to update branch reference")?;

        // Reset working directory to match the new commit
        repo.reset(&remote_commit.as_object(), ResetType::Hard, None)
            .context("Failed to reset working directory")?;

        println!("Successfully updated to {}.", remote_commit.id());
    } else if analysis.is_up_to_date() {
        println!("Already up to date.");
    } else {
        return Err(anyhow::anyhow!(
            "Cannot perform fast-forward merge. Manual merge required. Consider using 'git pull' directly."
        ));
    }

    Ok(())
}

fn clone_repository(remote_url: &str) -> Result<()> {
    // Extract repository name from URL for the directory name
    let repo_name = extract_repo_name(remote_url)?;
    
    println!("Cloning {} into {}...", remote_url, repo_name);

    // Set up callbacks for authentication
    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(|_url, username_from_url, allowed_types| {
        // Try credential helper first
        if allowed_types.contains(git2::CredentialType::USER_PASS_PLAINTEXT) {
            if let Ok(cred) = git2::Cred::credential_helper(
                &git2::Config::open_default().unwrap_or_else(|_| git2::Config::new().unwrap()),
                _url,
                username_from_url,
            ) {
                return Ok(cred);
            }
        }
        
        // Try SSH key if available
        if allowed_types.contains(git2::CredentialType::SSH_KEY) {
            if let Some(username) = username_from_url {
                return git2::Cred::ssh_key_from_agent(username);
            }
        }
        
        // Default credentials
        git2::Cred::default()
    });

    // Set up fetch options with callbacks
    let mut fetch_options = FetchOptions::new();
    fetch_options.remote_callbacks(callbacks);

    // Clone the repository
    let mut builder = git2::build::RepoBuilder::new();
    builder.fetch_options(fetch_options);
    
    let repo = builder.clone(remote_url, Path::new(&repo_name))
        .with_context(|| format!("Failed to clone repository from '{}'. Check the URL and your access permissions.", remote_url))?;

    println!("Repository cloned successfully into '{}'", repo_name);
    
    // Print the current branch
    if let Ok(head) = repo.head() {
        if let Some(branch_name) = head.shorthand() {
            println!("Switched to branch '{}'", branch_name);
        }
    }

    Ok(())
}

fn extract_repo_name(url: &str) -> Result<String> {
    // Handle different URL formats:
    // https://github.com/user/repo.git -> repo
    // git@github.com:user/repo.git -> repo
    // https://github.com/user/repo -> repo
    
    let url = url.trim_end_matches('/');
    
    // Remove .git suffix if present
    let url = if url.ends_with(".git") {
        &url[..url.len() - 4]
    } else {
        url
    };
    
    // Extract the last part after / or :
    let repo_name = url.split(&['/', ':'][..])
        .last()
        .ok_or_else(|| anyhow::anyhow!("Could not extract repository name from URL: {}", url))?;
    
    if repo_name.is_empty() {
        return Err(anyhow::anyhow!("Invalid repository URL: {}", url));
    }
    
    Ok(repo_name.to_string())
}
