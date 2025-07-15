use anyhow::{Context, Result};
use git2::Repository;
use std::cmp::Ordering;
use shtt::git_utils::{get_signature, push_tag_to_origin, open_repository};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SemVer {
    pub major: u32,
    pub minor: u32,
    pub patch: u32,
}

impl SemVer {
    pub fn new(major: u32, minor: u32, patch: u32) -> Self {
        Self { major, minor, patch }
    }

    pub fn from_tag(tag: &str) -> Result<Self> {
        let tag = tag.strip_prefix('v').unwrap_or(tag);
        let parts: Vec<&str> = tag.split('.').collect();
        
        if parts.len() != 3 {
            return Err(anyhow::anyhow!("Invalid semver format: {}", tag));
        }

        let major = parts[0].parse::<u32>()
            .with_context(|| format!("Invalid major version: {}", parts[0]))?;
        let minor = parts[1].parse::<u32>()
            .with_context(|| format!("Invalid minor version: {}", parts[1]))?;
        let patch = parts[2].parse::<u32>()
            .with_context(|| format!("Invalid patch version: {}", parts[2]))?;

        Ok(Self::new(major, minor, patch))
    }

    pub fn increment_major(&mut self) {
        self.major += 1;
        self.minor = 0;
        self.patch = 0;
    }

    pub fn increment_minor(&mut self) {
        self.minor += 1;
        self.patch = 0;
    }

    pub fn increment_patch(&mut self) {
        self.patch += 1;
    }

    pub fn to_tag(&self) -> String {
        format!("v{}.{}.{}", self.major, self.minor, self.patch)
    }
}

impl PartialOrd for SemVer {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for SemVer {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.major.cmp(&other.major) {
            Ordering::Equal => match self.minor.cmp(&other.minor) {
                Ordering::Equal => self.patch.cmp(&other.patch),
                other => other,
            },
            other => other,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum VersionBump {
    Major,
    Minor,
    Patch,
}

impl std::str::FromStr for VersionBump {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "major" => Ok(VersionBump::Major),
            "minor" => Ok(VersionBump::Minor),
            "patch" => Ok(VersionBump::Patch),
            _ => Err(anyhow::anyhow!("Invalid version bump type: {}. Must be 'major', 'minor', or 'patch'", s)),
        }
    }
}

/// Create a new semver tag by incrementing the current highest tag
pub fn drop_tag(bump: VersionBump) -> Result<()> {
    let repo = open_repository()?;

    // Get all tags and find the highest semver tag
    let highest_version = get_highest_semver_tag(&repo)?;
    
    // Create the new version
    let mut new_version = highest_version.unwrap_or_else(|| SemVer::new(0, 0, 0));
    
    match bump {
        VersionBump::Major => new_version.increment_major(),
        VersionBump::Minor => new_version.increment_minor(),
        VersionBump::Patch => new_version.increment_patch(),
    }

    let new_tag = new_version.to_tag();
    
    // Check if tag already exists
    if tag_exists(&repo, &new_tag)? {
        return Err(anyhow::anyhow!("Tag '{}' already exists", new_tag));
    }

    // Get the current HEAD commit
    let head = repo.head()
        .context("Failed to get HEAD reference")?;
    let commit = head.peel_to_commit()
        .context("Failed to get HEAD commit")?;

    // Get signature for the tag using refactored function
    let signature = get_signature(&repo)?;

    // Create the tag
    let tag_message = format!("Release {}", new_tag);
    repo.tag(
        &new_tag,
        &commit.as_object(),
        &signature,
        &tag_message,
        false, // not forced
    ).with_context(|| format!("Failed to create tag '{}'", new_tag))?;

    println!("Created tag: {}", new_tag);
    
    // Push the tag to origin using refactored function
    push_tag_to_origin(&repo, &new_tag)?;

    Ok(())
}

fn get_highest_semver_tag(repo: &Repository) -> Result<Option<SemVer>> {
    let mut highest: Option<SemVer> = None;

    // Get all tag names
    let tag_names = repo.tag_names(None)
        .context("Failed to get tag names")?;

    for tag_name in tag_names.iter() {
        if let Some(name) = tag_name {
            // Try to parse as semver
            if let Ok(version) = SemVer::from_tag(name) {
                match &highest {
                    None => highest = Some(version),
                    Some(current_highest) => {
                        if version > *current_highest {
                            highest = Some(version);
                        }
                    }
                }
            }
        }
    }

    Ok(highest)
}

fn tag_exists(repo: &Repository, tag_name: &str) -> Result<bool> {
    match repo.find_reference(&format!("refs/tags/{}", tag_name)) {
        Ok(_) => Ok(true),
        Err(ref e) if e.code() == git2::ErrorCode::NotFound => Ok(false),
        Err(e) => Err(anyhow::anyhow!("Error checking if tag exists: {}", e)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_semver_parsing() {
        assert_eq!(SemVer::from_tag("v1.2.3").unwrap(), SemVer::new(1, 2, 3));
        assert_eq!(SemVer::from_tag("1.2.3").unwrap(), SemVer::new(1, 2, 3));
        assert_eq!(SemVer::from_tag("v0.1.0").unwrap(), SemVer::new(0, 1, 0));
        
        assert!(SemVer::from_tag("1.2").is_err());
        assert!(SemVer::from_tag("1.2.3.4").is_err());
        assert!(SemVer::from_tag("v1.2.a").is_err());
    }

    #[test]
    fn test_semver_ordering() {
        let v1 = SemVer::new(1, 0, 0);
        let v2 = SemVer::new(1, 1, 0);
        let v3 = SemVer::new(1, 1, 1);
        let v4 = SemVer::new(2, 0, 0);

        assert!(v1 < v2);
        assert!(v2 < v3);
        assert!(v3 < v4);
        assert!(v1 < v4);
    }

    #[test]
    fn test_version_bumps() {
        let mut v = SemVer::new(1, 2, 3);
        
        v.increment_patch();
        assert_eq!(v, SemVer::new(1, 2, 4));

        v.increment_minor();
        assert_eq!(v, SemVer::new(1, 3, 0));

        v.increment_major();
        assert_eq!(v, SemVer::new(2, 0, 0));
    }

    #[test]
    fn test_version_bump_parsing() {
        assert!(matches!("major".parse::<VersionBump>().unwrap(), VersionBump::Major));
        assert!(matches!("minor".parse::<VersionBump>().unwrap(), VersionBump::Minor));
        assert!(matches!("patch".parse::<VersionBump>().unwrap(), VersionBump::Patch));
        assert!(matches!("MAJOR".parse::<VersionBump>().unwrap(), VersionBump::Major));
        
        assert!("invalid".parse::<VersionBump>().is_err());
    }

    #[test]
    fn test_tag_formatting() {
        let v = SemVer::new(1, 2, 3);
        assert_eq!(v.to_tag(), "v1.2.3");
    }
}
