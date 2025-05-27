use crate::utils::index::{Index, IndexEntry};
use crate::utils::hash_object::resolve_head;
use crate::commands::commit::load_tree_map_from_commit;
use std::path::PathBuf;

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
            mode: "100644".to_string(),
        };
        index.add(entry);
        println!("Unstaged changes in '{}'", path);
    } else {
        index.remove(path);
        println!("Removed '{}' from staging (not in HEAD)", path);
    }

    index.save();
}
