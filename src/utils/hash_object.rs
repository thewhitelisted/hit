// a hash is the prefix to the word: hashbrown.
// a hashbrown is a type of potato.
// i like potato.

use flate2::Compression;
use flate2::write::ZlibEncoder;
use sha1::{Digest, Sha1};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

pub fn hash_object(file_path: &str, write: bool, print: bool) -> String {
    // Resolve the absolute path of the file
    let resolved_path = Path::new(file_path)
        .canonicalize()
        .expect("Failed to resolve file path");

    // Read the file content as binary
    let content = fs::read(&resolved_path).expect("Failed to read file");

    // Prepare the Git object: "blob <len>\0<content>"
    let header = format!("blob {}\0", content.len());
    let mut object_data = Vec::new();
    object_data.extend_from_slice(header.as_bytes());
    object_data.extend_from_slice(&content);

    // Compute the SHA-1 hash of the object data
    let mut hasher = Sha1::new();
    hasher.update(&object_data);
    let hash = hasher.finalize();
    let hash_hex = format!("{:x}", hash);

    // Print the hash
    if print {
        println!("{}", hash_hex);
    }

    if write {
        // Prepare the path: .git/objects/ab/cdef... based on hash
        let object_dir = format!(".hit/objects/{}", &hash_hex[..2]);
        let object_file = hash_hex[2..].to_string();
        let object_path = PathBuf::from(&object_dir).join(object_file);

        // Skip if the object already exists
        if object_path.exists() {
            return hash_hex;
        }

        // Ensure the directory exists
        fs::create_dir_all(&object_dir).expect("Failed to create object directory");

        // Compress the object data using zlib
        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
        encoder
            .write_all(&object_data)
            .expect("Failed to compress object");
        let compressed = encoder.finish().expect("Failed to finalize compression");

        // Write the compressed data to disk
        fs::write(&object_path, compressed).expect("Failed to write object file");
    }

    hash_hex
}

pub fn resolve_head() -> Option<String> {
    let head = fs::read_to_string(".hit/HEAD").ok()?;
    if head.starts_with("ref: ") {
        let ref_path = head[5..].trim();
        let full_path = Path::new(".hit").join(ref_path);
        fs::read_to_string(full_path).ok().map(|s| s.trim().to_string())
    } else {
        Some(head.trim().to_string())
    }
}
