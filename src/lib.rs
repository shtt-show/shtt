use anyhow::{Context, Result};
use git2::{Repository, Signature, PushOptions, RemoteCallbacks, FetchOptions};

/// Common Git utilities shared across SHTT commands
pub mod git_utils {
    use super::*;

    /// Create a Git signature from repository configuration
    pub fn get_signature(repo: &Repository) -> Result<Signature> {
        let config = repo.config()
            .context("Failed to get repository config")?;
        
        let name = config.get_string("user.name")
            .context("Git user.name not configured. Run: git config user.name \"Your Name\"")?;
        let email = config.get_string("user.email")
            .context("Git user.email not configured. Run: git config user.email \"your.email@example.com\"")?;
        
        Signature::now(&name, &email)
            .context("Failed to create git signature")
    }

    /// Create authentication callbacks for Git operations
    pub fn create_auth_callbacks() -> RemoteCallbacks<'static> {
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
        callbacks
    }

    /// Push a branch to origin remote
    pub fn push_to_origin(repo: &Repository, branch_name: &str) -> Result<()> {
        let mut remote = repo.find_remote("origin")
            .context("Failed to find 'origin' remote. Make sure you have an origin remote configured.")?;

        let mut push_options = PushOptions::new();
        push_options.remote_callbacks(create_auth_callbacks());

        let refspec = format!("refs/heads/{}:refs/heads/{}", branch_name, branch_name);
        remote.push(&[&refspec], Some(&mut push_options))
            .context("Failed to push to origin. Make sure you have push access and the remote is configured correctly.")?;

        println!("Pushed changes to origin/{}", branch_name);
        Ok(())
    }

    /// Push a tag to origin remote
    pub fn push_tag_to_origin(repo: &Repository, tag_name: &str) -> Result<()> {
        let mut remote = repo.find_remote("origin")
            .context("Failed to find 'origin' remote. Make sure you have an origin remote configured.")?;

        let mut push_options = PushOptions::new();
        push_options.remote_callbacks(create_auth_callbacks());

        let refspec = format!("refs/tags/{}:refs/tags/{}", tag_name, tag_name);
        remote.push(&[&refspec], Some(&mut push_options))
            .with_context(|| format!("Failed to push tag '{}' to origin. Make sure you have push access and the remote is configured correctly.", tag_name))?;

        println!("Pushed tag {} to origin", tag_name);
        Ok(())
    }

    /// Get the current branch name from repository
    pub fn get_current_branch(repo: &Repository) -> Result<String> {
        let head = repo.head()
            .context("Failed to get HEAD reference. Make sure you're on a branch.")?;
        
        let branch_name = head.shorthand()
            .context("Failed to get current branch name")?;
        
        Ok(branch_name.to_string())
    }

    /// Open and validate a git repository
    pub fn open_repository() -> Result<Repository> {
        Repository::open(".")
            .context("Failed to open git repository. Are you in a git repository?")
    }

    /// Create fetch options with authentication
    pub fn create_fetch_options() -> FetchOptions<'static> {
        let mut fetch_options = FetchOptions::new();
        fetch_options.remote_callbacks(create_auth_callbacks());
        fetch_options
    }

    /// Extract repository name from a Git URL
    pub fn extract_repo_name(url: &str) -> Result<String> {
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
}

/// Common ordinal formatting used in commit messages
pub mod format_utils {
    /// Format a number as an ordinal (1st, 2nd, 3rd, 4th, etc.)
    pub fn format_ordinal(n: usize) -> String {
        let suffix = match n % 100 {
            11..=13 => "th", // Special case: 11th, 12th, 13th (not 11st, 12nd, 13rd)
            _ => match n % 10 {
                1 => "st",
                2 => "nd", 
                3 => "rd",
                _ => "th",
            },
        };
        format!("{}{}", n, suffix)
    }
}

/// Common repository operations
pub mod repo_utils {
    use super::*;
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
}