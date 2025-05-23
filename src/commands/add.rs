use std::fs;
use std::path::{Path, PathBuf};

use crate::utils::index::{Index, IndexEntry};
use crate::utils::hash_object;

pub fn add(path: &str) {
    let path_buf = PathBuf::from(path);

    if !path_buf.exists() {
        eprintln!("fatal: path '{}' does not exist", path);
        std::process::exit(1);
    }

    let mut index = Index::load();

    if path_buf.is_file() {
        add_file(&path_buf, &mut index);
    } else if path_buf.is_dir() {
        add_directory(&path_buf, &mut index);
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

fn add_directory(dir: &Path, index: &mut Index) {
    for entry in fs::read_dir(dir).expect("Failed to read directory") {
        let entry = entry.expect("Failed to read entry");
        let path = entry.path();

        if path.strip_prefix(".").unwrap_or(&path).starts_with(".hit") {
            continue; // skip internal metadata
        }

        if path.is_file() {
            add_file(&path, index);
        } else if path.is_dir() {
            add_directory(&path, index);
        }
    }
}
