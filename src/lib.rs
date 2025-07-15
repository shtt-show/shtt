use anyhow::{Context, Result};
use git2::{Repository, Signature, PushOptions, RemoteCallbacks, FetchOptions};
use std::io::{self, Write};

/// Common Git utilities shared across SHTT commands
pub mod git_utils {
    use super::*;

    /// Setup git configuration interactively if missing
    pub fn setup_git_config_if_missing(repo: &Repository) -> Result<()> {
        // Try to get existing config
        let config = repo.config()
            .context("Failed to get repository config")?;
        
        let mut needs_name = false;
        let mut needs_email = false;
        
        // Check if user.name exists
        match config.get_string("user.name") {
            Ok(_) => {}, // Name is set
            Err(_) => needs_name = true,
        }
        
        // Check if user.email exists
        match config.get_string("user.email") {
            Ok(_) => {}, // Email is set
            Err(_) => needs_email = true,
        }
        
        // If both are set, we're good
        if !needs_name && !needs_email {
            return Ok(());
        }
        
        // We need to setup git config
        println!("Git configuration is not complete. Let's set it up!");
        println!("This information will be used to identify your commits.\n");
        
        // Get the global config file path and open it for writing
        let mut global_config = open_or_create_global_config()?;
        
        if needs_name {
            let name = prompt_for_name()?;
            global_config.set_str("user.name", &name)
                .context("Failed to set user.name in global config")?;
            println!("✓ Set user.name to: {}", name);
        }
        
        if needs_email {
            let email = prompt_for_email()?;
            global_config.set_str("user.email", &email)
                .context("Failed to set user.email in global config")?;
            println!("✓ Set user.email to: {}", email);
        }
        
        println!("\nGit configuration complete! 🎉");
        Ok(())
    }

    /// Open or create the global git config file
    fn open_or_create_global_config() -> Result<git2::Config> {
        // Try to open existing global config
        match git2::Config::open_default() {
            Ok(config) => {
                // Try to get the global level specifically
                match config.open_level(git2::ConfigLevel::Global) {
                    Ok(global_config) => Ok(global_config),
                    Err(_) => {
                        // Global config doesn't exist, create it
                        create_global_config()
                    }
                }
            }
            Err(_) => {
                // No config exists at all, create global config
                create_global_config()
            }
        }
    }

    /// Create a new global git config file
    fn create_global_config() -> Result<git2::Config> {
        use std::fs;
        
        // Get the home directory
        let home_dir = dirs::home_dir()
            .context("Could not determine home directory")?;
        
        let gitconfig_path = home_dir.join(".gitconfig");
        
        // Create the .gitconfig file if it doesn't exist
        if !gitconfig_path.exists() {
            fs::File::create(&gitconfig_path)
                .with_context(|| format!("Failed to create {}", gitconfig_path.display()))?;
        }
        
        // Open the config file
        git2::Config::open(&gitconfig_path)
            .with_context(|| format!("Failed to open {}", gitconfig_path.display()))
    }

    /// Prompt user for their name
    fn prompt_for_name() -> Result<String> {
        loop {
            print!("Enter your full name (e.g., 'John Doe'): ");
            io::stdout().flush()
                .context("Failed to flush stdout")?;
            
            let mut input = String::new();
            io::stdin().read_line(&mut input)
                .context("Failed to read from stdin")?;
            
            let name = input.trim().to_string();
            
            if name.is_empty() {
                println!("Name cannot be empty. Please try again.");
                continue;
            }
            
            if name.len() < 2 {
                println!("Please enter your full name.");
                continue;
            }
            
            return Ok(name);
        }
    }

    /// Prompt user for their email
    fn prompt_for_email() -> Result<String> {
        loop {
            print!("Enter your email address (e.g., 'john@example.com'): ");
            io::stdout().flush()
                .context("Failed to flush stdout")?;
            
            let mut input = String::new();
            io::stdin().read_line(&mut input)
                .context("Failed to read from stdin")?;
            
            let email = input.trim().to_string();
            
            if email.is_empty() {
                println!("Email cannot be empty. Please try again.");
                continue;
            }
            
            if !is_valid_email(&email) {
                println!("Please enter a valid email address.");
                continue;
            }
            
            return Ok(email);
        }
    }

    /// Basic email validation
    fn is_valid_email(email: &str) -> bool {
        // Check basic requirements
        if email.len() <= 3 || !email.contains('@') || email.starts_with('@') || email.ends_with('@') {
            return false;
        }
        
        // Check for invalid patterns
        if email.contains("..") || email.contains("@@") {
            return false;
        }
        
        // Split on @ and ensure exactly one @ symbol
        let parts: Vec<&str> = email.split('@').collect();
        if parts.len() != 2 {
            return false;
        }
        
        let (local, domain) = (parts[0], parts[1]);
        
        // Local part (before @) validation
        if local.is_empty() || local.starts_with('.') || local.ends_with('.') {
            return false;
        }
        
        // Domain part (after @) validation  
        if domain.is_empty() || domain.starts_with('.') || domain.ends_with('.') || !domain.contains('.') {
            return false;
        }
        
        // Check allowed characters
        email.chars().all(|c| c.is_ascii_alphanumeric() || "@.-_+".contains(c))
    }

    /// Create a Git signature from repository configuration with interactive setup
    pub fn get_signature(repo: &Repository) -> Result<Signature> {
        // First try to setup config if missing
        setup_git_config_if_missing(repo)?;
        
        // Now get the signature normally
        let config = repo.config()
            .context("Failed to get repository config")?;
        
        let name = config.get_string("user.name")
            .context("Git user.name not configured even after setup - this shouldn't happen")?;
        let email = config.get_string("user.email")
            .context("Git user.email not configured even after setup - this shouldn't happen")?;
        
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

    #[cfg(test)]
    mod tests {
        use super::*;
        
        #[test]
        fn test_email_validation() {
            // Valid emails
            assert!(is_valid_email("user@example.com"));
            assert!(is_valid_email("test.email+tag@example.co.uk"));
            assert!(is_valid_email("user123@domain-name.org"));
            assert!(is_valid_email("a@b.co"));
            assert!(is_valid_email("user_name@example.com"));
            assert!(is_valid_email("user+tag@example.com"));
            
            // Invalid emails
            assert!(!is_valid_email(""));                    // Empty
            assert!(!is_valid_email("@"));                   // Just @
            assert!(!is_valid_email("user@"));              // Missing domain
            assert!(!is_valid_email("@example.com"));       // Missing local part
            assert!(!is_valid_email("user@@example.com"));  // Double @
            assert!(!is_valid_email("user@example@com"));   // Multiple @
            assert!(!is_valid_email("user..name@example.com")); // Consecutive dots in local
            assert!(!is_valid_email("user@example..com"));  // Consecutive dots in domain
            assert!(!is_valid_email(".user@example.com"));  // Leading dot in local
            assert!(!is_valid_email("user.@example.com"));  // Trailing dot in local
            assert!(!is_valid_email("user@.example.com"));  // Leading dot in domain
            assert!(!is_valid_email("user@example.com."));  // Trailing dot in domain
            assert!(!is_valid_email("user@example"));       // No dot in domain
            assert!(!is_valid_email("a@b"));                // Domain too short
        }
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