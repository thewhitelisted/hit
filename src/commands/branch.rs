// a branch refers to the part of a tree that is smaller than the trunk, in this case, a branch of the commit history

use std::fs;
use std::path::{Path, PathBuf};
use std::io;

/// Create a new branch or list all branches
pub fn branch(branch_name: Option<&str>) -> Result<(), io::Error> {
    if let Some(name) = branch_name {
        // Create a new branch pointing to the current commit
        create_branch(name)?;
    } else {
        // List all branches
        list_branches()?;
    }

    Ok(())
}

/// Create a new branch pointing to the current commit
fn create_branch(branch_name: &str) -> Result<(), io::Error> {
    // Validate branch name (simple validation)
    if branch_name.contains('/') || branch_name.contains('\\') || branch_name.trim().is_empty() {
        eprintln!("Error: Invalid branch name '{}'", branch_name);
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid branch name"));
    }

    // Read the current commit SHA from HEAD
    let head_contents = fs::read_to_string(".hit/HEAD")?;
    let commit_sha = if head_contents.starts_with("ref: ") {
        // HEAD points to a ref
        let ref_path = head_contents[5..].trim();
        let full_ref_path = Path::new(".hit").join(ref_path);
        fs::read_to_string(full_ref_path)?.trim().to_string()
    } else {
        // Detached HEAD
        head_contents.trim().to_string()
    };

    // Create branch ref
    let branch_path = PathBuf::from(format!(".hit/refs/heads/{}", branch_name));
    
    // Check if branch already exists
    if branch_path.exists() {
        eprintln!("Error: Branch '{}' already exists", branch_name);
        return Err(io::Error::new(io::ErrorKind::AlreadyExists, "Branch already exists"));
    }

    // Create the branch (write commit SHA to branch ref file)
    fs::write(&branch_path, format!("{}\n", commit_sha))?;
    println!("Created branch '{}'", branch_name);

    Ok(())
}

/// List all branches in the repository
fn list_branches() -> Result<(), io::Error> {
    let heads_dir = Path::new(".hit/refs/heads");
    if !heads_dir.exists() {
        return Ok(());  // No branches yet
    }

    // Read HEAD to determine current branch
    let head_contents = fs::read_to_string(".hit/HEAD")?;
    let current_branch = if head_contents.starts_with("ref: refs/heads/") {
        Some(head_contents["ref: refs/heads/".len()..].trim())
    } else {
        None  // Detached HEAD
    };

    // List all files in refs/heads directory
    let entries = fs::read_dir(heads_dir)?;
    let mut found_branches = false;

    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_file() {
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                found_branches = true;
                if Some(name) == current_branch {
                    println!("* {}", name);
                } else {
                    println!("  {}", name);
                }
            }
        }
    }

    if !found_branches {
        println!("No branches found");
    }

    if current_branch.is_none() {
        println!("(HEAD detached at {})", head_contents.trim());
    }

    Ok(())
}
