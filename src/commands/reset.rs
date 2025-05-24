use crate::utils::index::{Index, IndexEntry};
use crate::utils::objects::Object;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

/// Reset index entry to match HEAD commit
pub fn reset(path: &str) {
    let head_sha = resolve_head().expect("No HEAD found");
    let head_tree = load_tree_map_from_commit(&head_sha);

    let mut index = Index::load();
    let file_path = PathBuf::from(path);

    if let Some(sha) = head_tree.get(&file_path) {
        // Re-stage the version from HEAD (unstage new changes)
        let entry = IndexEntry {
            path: path.to_string(),
            sha: sha.clone(),
            mode: "100644".to_string(), // Assume normal file for now
        };
        index.add(entry);
        println!("Unstaged changes in '{}'", path);
    } else {
        // If not in HEAD, remove from index
        index.remove(path);
        println!("Removed '{}' from staging (not in HEAD)", path);
    }

    index.save();
}

fn resolve_head() -> Option<String> {
    let head = fs::read_to_string(".hit/HEAD").ok()?;
    if head.starts_with("ref: ") {
        let ref_path = head[5..].trim();
        let full_path = Path::new(".hit").join(ref_path);
        fs::read_to_string(full_path)
            .ok()
            .map(|s| s.trim().to_string())
    } else {
        Some(head.trim().to_string())
    }
}

fn load_tree_map_from_commit(commit_sha: &str) -> HashMap<PathBuf, String> {
    let commit_obj = Object::read(commit_sha).expect("Failed to read commit");
    let tree_sha = match commit_obj {
        Object::Commit(c) => c.tree,
        _ => panic!("Not a commit object"),
    };

    let mut map = HashMap::new();
    build_tree_map_recursive(&tree_sha, PathBuf::from(""), &mut map);
    map
}

fn build_tree_map_recursive(tree_sha: &str, base: PathBuf, map: &mut HashMap<PathBuf, String>) {
    let obj = Object::read(tree_sha).expect("Failed to read tree object");
    let tree = match obj {
        Object::Tree(tree) => tree,
        _ => panic!("Not a tree object"),
    };

    for entry in tree.entries {
        let path = base.join(entry.name);
        match entry.mode.as_str() {
            "100644" | "100755" => {
                map.insert(path, entry.sha);
            }
            "40000" => {
                build_tree_map_recursive(&entry.sha, path, map);
            }
            _ => {}
        }
    }
}
