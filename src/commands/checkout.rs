use std::fs;
use std::path::PathBuf;

pub fn resolve_head() -> Option<String> {
    let head_path = PathBuf::from(".hit/HEAD");
    let head_contents = fs::read_to_string(&head_path).ok()?.trim().to_string();

    if head_contents.starts_with("ref:") {
        // symbolic ref
        let ref_path = head_contents[5..].trim(); // after "ref: "
        let full_ref_path = PathBuf::from(".hit").join(ref_path);
        fs::read_to_string(full_ref_path).ok().map(|s| s.trim().to_string())
    } else {
        // detached HEAD
        Some(head_contents)
    }
}

pub fn update_head_to_branch(branch: &str, sha: &str) {
    let head_contents = format!("ref: refs/heads/{}\n", branch);
    fs::write(".hit/HEAD", head_contents).expect("Failed to write HEAD");

    let ref_path = format!(".hit/refs/heads/{}", branch);
    fs::create_dir_all(".hit/refs/heads").expect("Failed to create branch ref dir");
    fs::write(ref_path, format!("{}\n", sha)).expect("Failed to update branch ref");
}

pub fn update_head_to_commit(sha: &str) {
    fs::write(".hit/HEAD", format!("{}\n", sha)).expect("Failed to write detached HEAD");
}


pub fn current_branch_name() -> Option<String> {
    let head_path = PathBuf::from(".hit/HEAD");
    let head_contents = fs::read_to_string(&head_path).ok()?;
    if head_contents.starts_with("ref: ") {
        let full = head_contents[5..].trim(); // "refs/heads/master"
        full.strip_prefix("refs/heads/").map(|s| s.to_string())
    } else {
        None // detached HEAD
    }
}

fn checkout(target: &str) {
    // is this a branch name?
    let branch_ref = format!(".hit/refs/heads/{}", target);
    if PathBuf::from(&branch_ref).exists() {
        // resolve the SHA
        let sha = fs::read_to_string(&branch_ref)
            .expect("Failed to read branch ref")
            .trim()
            .to_string();

        restore_commit(&sha); // do your restore logic
        update_head_to_branch(target, &sha);
    } else {
        // assume it's a commit SHA (detached)
        restore_commit(target);
        update_head_to_commit(target);
    }
}
