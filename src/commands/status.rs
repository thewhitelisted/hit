// status refers to the level of being or condition of something, in this case, the state of the repository

use super::objects::{Object};
use super::hash_object::hash_object;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

pub fn status() {
    // TODO: implement support for .hitignore
    let head_sha = resolve_head().expect("Failed to resolve HEAD");
    let head_commit = match Object::read(&head_sha).expect("Failed to read HEAD object") {
        Object::Commit(commit) => commit,
        _ => panic!("HEAD does not point to a commit"),
    };

    let mut head_tree = HashMap::new();
    build_tree_map(&head_commit.tree, PathBuf::from(""), &mut head_tree);

    let mut modified = Vec::new();
    let mut untracked = Vec::new();

    let mut visited = HashSet::new();

    for entry in walk_working_dir(".") {
        let rel_path = entry.strip_prefix(".").unwrap().to_path_buf();
        visited.insert(rel_path.clone());

        if let Some(expected_sha) = head_tree.get(&rel_path) {
            let actual_sha = hash_object(entry.to_str().unwrap(), false, false);
            if &actual_sha != expected_sha {
                modified.push(rel_path);
            }
        } else {
            untracked.push(rel_path);
        }
    }

    let deleted: Vec<_> = head_tree
        .keys()
        .filter(|path| !visited.contains(*path))
        .cloned()
        .collect();

    // Print results
    if !modified.is_empty() {
        println!("Changes not staged for commit:");
        for path in &modified {
            println!("  modified:   {}", path.display());
        }
    }

    if !deleted.is_empty() {
        println!("\nDeleted files:");
        for path in &deleted {
            println!("  deleted:    {}", path.display());
        }
    }

    if !untracked.is_empty() {
        println!("\nUntracked files:");
        for path in &untracked {
            println!("  {}", path.display());
        }
    }

    if modified.is_empty() && untracked.is_empty() && deleted.is_empty() {
        println!("Nothing to commit, working directory clean.");
    }
}

/// Recursively builds a map from paths to blob SHAs
fn build_tree_map(tree_sha: &str, base: PathBuf, map: &mut HashMap<PathBuf, String>) {
    let obj = Object::read(tree_sha).expect("Failed to read tree object");
    let tree = match obj {
        Object::Tree(tree) => tree,
        _ => panic!("Not a tree object"),
    };

    for entry in tree.entries {
        let full_path = base.join(entry.name);
        match entry.mode.as_str() {
            "100644" | "100755" => {
                map.insert(full_path, entry.sha);
            }
            "40000" => {
                build_tree_map(&entry.sha, full_path, map);
            }
            _ => {}
        }
    }
}

/// Recursively walks the working directory and returns file paths
fn walk_working_dir(root: &str) -> Vec<PathBuf> {
    let mut files = Vec::new();
    let entries = fs::read_dir(root).expect("Failed to read working directory");

    for entry in entries {
        let entry = entry.expect("Failed to read dir entry");
        let path = entry.path();

        // Skip .hit directory
        if path.strip_prefix(".").unwrap_or(&path).starts_with(".hit") {
            continue;
        }

        if path.is_dir() {
            files.extend(walk_working_dir(path.to_str().unwrap()));
        } else {
            files.push(path);
        }
    }

    files
}

/// Resolves HEAD to a commit SHA
fn resolve_head() -> Option<String> {
    let head = fs::read_to_string(".hit/HEAD").ok()?;
    if head.starts_with("ref: ") {
        let ref_path = head[5..].trim();
        let full_path = Path::new(".hit").join(ref_path);
        fs::read_to_string(full_path).ok().map(|s| s.trim().to_string())
    } else {
        Some(head.trim().to_string())
    }
}
