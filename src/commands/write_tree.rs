// writing trees is grammatically correct, but not semantically correct

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use flate2::Compression;
use flate2::write::ZlibEncoder;
use sha1::{Digest, Sha1};

use crate::utils::hash_object;

/// Entry point: write the root tree from the current directory
pub fn write_tree() {
    let sha = write_directory(".", Vec::new());
    println!("{}", sha);
}

/// Recursively writes a tree object for a directory
pub fn write_directory(path: &str, ignore: Vec<String>) -> String {
    // println!("Writing directory: {}", path);
    let mut tree_entries = Vec::new();
    let path_buf = PathBuf::from(path);

    // set ignorelist to ignore or read .hitignore if it exists
    let mut ignore_list = ignore;
    let ignore_path = path_buf.join(".hitignore");
    if ignore_path.exists() {
        let contents = fs::read_to_string(&ignore_path).expect("Failed to read .hitignore");
        for line in contents.lines() {
            let trimmed = line.trim();
            if !trimmed.is_empty() {
                ignore_list.push(trimmed.to_string());
            }
        }
    }

    for entry in fs::read_dir(&path_buf).expect("Failed to read directory") {
        let entry = entry.expect("Failed to read entry");
        let file_path = entry.path();
        let name = entry.file_name().to_string_lossy().to_string();

        if file_path.is_file() {
            // Hash file and write blob
            let blob_hash = hash_object::hash_object(file_path.to_str().unwrap(), true, false);

            // Convert hex SHA to 20-byte binary
            let sha_bytes = hex::decode(blob_hash).expect("Invalid SHA");

            // Add tree entry: 100644 <name>\0<binary SHA>
            let mut entry = Vec::new();
            entry.extend_from_slice(b"100644 ");
            entry.extend_from_slice(name.as_bytes());
            entry.push(0);
            entry.extend_from_slice(&sha_bytes);

            tree_entries.extend(entry);
        } else if file_path.is_dir() && name != ".hit" {
            // skip all directories that are in the ignore list
            if ignore_list.iter().any(|ignore| ignore.contains(&name)) {
                // println!("Ignoring directory: {}", name);
                continue;
            }
            // Recurse into subdirectory
            let sub_tree_hash = write_directory(file_path.to_str().unwrap(), ignore_list.clone());
            let sha_bytes = hex::decode(sub_tree_hash).expect("Invalid SHA");

            // Add tree entry: 40000 <dirname>\0<binary SHA>
            let mut entry = Vec::new();
            entry.extend_from_slice(b"40000 ");
            entry.extend_from_slice(name.as_bytes());
            entry.push(0);
            entry.extend_from_slice(&sha_bytes);

            tree_entries.extend(entry);
        }
    }

    // Prepare tree object: "tree <len>\0<entries>"
    let mut tree_obj = Vec::new();
    let header = format!("tree {}\0", tree_entries.len());
    tree_obj.extend_from_slice(header.as_bytes());
    tree_obj.extend(tree_entries);

    // Hash the tree object
    let mut hasher = Sha1::new();
    hasher.update(&tree_obj);
    let hash = hasher.finalize();
    let hash_hex = format!("{:x}", hash);

    // Store in .git/objects/xx/yyyy...
    let object_dir = format!(".hit/objects/{}", &hash_hex[..2]);
    let object_file = (&hash_hex[2..]).to_string();
    let object_path = Path::new(&object_dir).join(&object_file);

    if !object_path.exists() {
        fs::create_dir_all(&object_dir).expect("Failed to create object directory");

        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
        encoder
            .write_all(&tree_obj)
            .expect("Failed to compress object");
        let compressed = encoder.finish().expect("Failed to finalize compression");

        fs::write(&object_path, compressed).expect("Failed to write tree object");
    }

    hash_hex
}
