use anyhow::{Context, Result};
use git2::{Repository, Signature, IndexAddOption, PushOptions, RemoteCallbacks};

/// Save changes by committing and pushing to origin
pub fn save_changes(message: Option<String>) -> Result<()> {
    let repo = Repository::open(".")
        .context("Failed to open git repository. Are you in a git repository?")?;

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

    // Create commit
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

    // Push to origin
    push_to_origin(&repo)?;

    Ok(())
}

fn format_ordinal(n: usize) -> String {
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

fn count_commits(repo: &Repository) -> Result<usize> {
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

// This function is no longer used since we auto-generate commit messages
// when none are provided, but keeping it commented for reference
/*
fn prompt_for_commit_message() -> Result<String> {
    print!("Enter commit message: ");
    io::stdout().flush().context("Failed to flush stdout")?;
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .context("Failed to read commit message")?;
    
    let message = input.trim();
    if message.is_empty() {
        return Err(anyhow::anyhow!("Commit message cannot be empty"));
    }
    
    Ok(message.to_string())
}
*/

fn get_signature(repo: &Repository) -> Result<Signature> {
    // Try to get signature from git config
    let config = repo.config()
        .context("Failed to get repository config")?;
    
    let name = config.get_string("user.name")
        .context("Git user.name not configured. Run: git config user.name \"Your Name\"")?;
    let email = config.get_string("user.email")
        .context("Git user.email not configured. Run: git config user.email \"your.email@example.com\"")?;
    
    Signature::now(&name, &email)
        .context("Failed to create git signature")
}

fn push_to_origin(repo: &Repository) -> Result<()> {
    // Get the current branch name
    let head = repo.head()
        .context("Failed to get HEAD reference")?;
    let branch_name = head.shorthand()
        .context("Failed to get branch name")?;

    // Find the origin remote
    let mut remote = repo.find_remote("origin")
        .context("Failed to find 'origin' remote. Make sure you have an origin remote configured.")?;

    // Set up callbacks for authentication (this will use default git credentials)
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

    let mut push_options = PushOptions::new();
    push_options.remote_callbacks(callbacks);

    // Push the current branch to origin
    let refspec = format!("refs/heads/{}:refs/heads/{}", branch_name, branch_name);
    remote.push(&[&refspec], Some(&mut push_options))
        .context("Failed to push to origin. Make sure you have push access and the remote is configured correctly.")?;

    println!("Pushed changes to origin/{}", branch_name);
    Ok(())
}
