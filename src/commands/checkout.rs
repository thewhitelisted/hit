// checkout refers to the area of a grocery store where you pay for your items

use super::objects::{Object};
use std::fs;
use std::path::{Path, PathBuf};

/// Main checkout command — accepts a branch or commit SHA
pub fn checkout(target: &str) {
    let branch_path = format!(".hit/refs/heads/{}", target);

    if Path::new(&branch_path).exists() {
        // It's a branch name
        let sha = fs::read_to_string(&branch_path)
            .expect("Failed to read branch ref")
            .trim()
            .to_string();

        restore_commit(&sha);
        update_head_to_branch(target, &sha);
    } else {
        // Assume it's a commit SHA (detached)
        restore_commit(target);
        update_head_to_commit(target);
    }
}

/// Restore the working directory to the state of a commit
fn restore_commit(commit_sha: &str) {
    let commit_obj = Object::read(commit_sha).expect("Failed to read commit object");

    let tree_sha = match commit_obj {
        Object::Commit(commit) => commit.tree,
        _ => panic!("{} is not a commit object", commit_sha),
    };

    clear_working_directory();
    restore_tree(&tree_sha, PathBuf::from("."));
}

/// Recursively walk a tree and restore its files and subtrees
fn restore_tree(tree_sha: &str, base_path: PathBuf) {
    let tree_obj = Object::read(tree_sha).expect("Failed to read tree object");

    if let Object::Tree(tree) = tree_obj {
        for entry in tree.entries {
            let path = base_path.join(&entry.name);

            match entry.mode.as_str() {
                "100644" | "100755" => {
                    let blob = Object::read(&entry.sha).expect("Failed to read blob");
                    if let Object::Blob(data) = blob {
                        if let Some(parent) = path.parent() {
                            fs::create_dir_all(parent).expect("Failed to create directory");
                        }
                        fs::write(path, data).expect("Failed to write file");
                    }
                }
                "40000" => {
                    fs::create_dir_all(&path).expect("Failed to create directory");
                    restore_tree(&entry.sha, path);
                }
                _ => eprintln!("Unknown mode: {}", entry.mode),
            }
        }
    } else {
        panic!("{} is not a tree object", tree_sha);
    }
}

fn clear_working_directory() {
    // TODO: handle .hitignore
    let entries: Vec<_> = fs::read_dir(".")
        .expect("Failed to read directory")
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .collect();

    for path in entries {
        if path.file_name().map_or(false, |name| name == ".hit") {
            continue;
        }

        if path.is_dir() {
            fs::remove_dir_all(&path).expect("Failed to remove directory");
        } else {
            fs::remove_file(&path).expect("Failed to remove file");
        }
    }
}


/// Writes a detached HEAD (raw SHA)
fn update_head_to_commit(sha: &str) {
    fs::write(".hit/HEAD", format!("{}\n", sha)).expect("Failed to write detached HEAD");
}

/// Writes a symbolic HEAD and updates branch ref
fn update_head_to_branch(branch: &str, sha: &str) {
    let head_contents = format!("ref: refs/heads/{}\n", branch);
    fs::write(".hit/HEAD", head_contents).expect("Failed to write HEAD");

    let ref_path = format!(".hit/refs/heads/{}", branch);
    fs::create_dir_all(".hit/refs/heads").expect("Failed to create branch ref dir");
    fs::write(ref_path, format!("{}\n", sha)).expect("Failed to update branch ref");
}
