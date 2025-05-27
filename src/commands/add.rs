use std::fs;
use std::path::{Path, PathBuf};

use crate::utils::hash_object;
use crate::utils::index::{Index, IndexEntry};

pub fn add(path: &str) {
    let path_buf = PathBuf::from(path);

    if !path_buf.exists() {
        eprintln!("fatal: path '{}' does not exist", path);
        std::process::exit(1);
    }

    // create ignore list
    if path_buf.starts_with(".hit") {
        eprintln!("fatal: cannot add files in .hit directory");
        std::process::exit(1);
    }
    // go through .hitignore and create a list of ignored files/directories
    let hitignore = fs::read_to_string(".hitignore").unwrap_or_default();
    let ignored_paths: Vec<String> = hitignore
        .clone()
        .lines()
        .filter_map(|line| {
            let trimmed = line.trim();
            if !trimmed.is_empty() && !trimmed.starts_with('#') {
                Some(trimmed.to_string())
            } else {
                None
            }
        })
        .collect();
    // check if the path is in the ignore list
    if ignored_paths
        .iter()
        .any(|ignore| path_buf.starts_with(ignore))
    {
        eprintln!("fatal: '{}' is ignored by .hitignore", path);
        std::process::exit(1);
    }

    let mut index = Index::load();

    if path_buf.is_file() {
        add_file(&path_buf, &mut index);
    } else if path_buf.is_dir() {
        add_directory(&path_buf, &mut index, hitignore);
    } else {
        eprintln!("fatal: '{}' is not a valid file or directory", path);
        std::process::exit(1);
    }

    index.save();
}

fn add_file(path: &Path, index: &mut Index) {
    let rel_path = path.strip_prefix(".").unwrap_or(path);
    let rel_str = rel_path.to_str().unwrap().replace("\\", "/"); // normalize for Windows

    let sha = hash_object::hash_object(path.to_str().unwrap(), true, false);
    let entry = IndexEntry {
        path: rel_str,
        sha,
        mode: "100644".to_string(),
    };

    index.add(entry);
}

fn add_directory(dir: &Path, index: &mut Index, ignorelist: String) {
    // Normalize the directory path
    let rel_dir = dir.strip_prefix(".").unwrap_or(dir);
    let rel_str = "/".to_owned() + &rel_dir.to_str().unwrap().replace("\\", "/"); // normalize for Windows
    let ignore_paths: Vec<&str> = ignorelist
        .lines()
        .filter_map(|line| {
            let trimmed = line.trim();
            if !trimmed.is_empty() && !trimmed.starts_with('#') {
                Some(trimmed)
            } else {
                None
            }
        })
        .collect();

    // check if the directory is in the ignore list
    if ignore_paths
        .iter()
        .any(|ignore| rel_str.starts_with(ignore))
    {
        return;
    }
    for entry in fs::read_dir(dir).expect("Failed to read directory") {
        let entry = entry.expect("Failed to read entry");
        let path = entry.path();

        if path.strip_prefix(".").unwrap_or(&path).starts_with(".hit") {
            continue; // skip internal metadata
        }

        if path.is_file() {
            add_file(&path, index);
        } else if path.is_dir() {
            add_directory(&path, index, ignorelist.clone());
        }
    }
}

/// Removes a file from the index (and optionally the working directory)
pub fn rm(path: &str, cached: bool) {
    let mut index = Index::load();

    if !index.entries.iter().any(|e| e.path == path) {
        eprintln!("fatal: path '{}' is not in the index", path);
        std::process::exit(1);
    }

    // Remove from index
    index.remove(path);
    index.save();

    // Remove from working directory if not --cached
    if !cached {
        if Path::new(path).exists() {
            fs::remove_file(path).expect("Failed to delete file");
        } else {
            eprintln!("warning: file '{}' already missing", path);
        }
    }

    println!("removed '{}'", path);
}
