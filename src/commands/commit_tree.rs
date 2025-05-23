// i couldn't come up with a witty line for this file

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use chrono::Offset;
use flate2::Compression;
use flate2::write::ZlibEncoder;
use sha1::{Digest, Sha1};

pub fn commit_tree(tree_sha: &str, message: &str) -> String {
    let author = "Your Name <you@example.com>";
    let now = chrono::Local::now();
    let offset = now.offset().fix().local_minus_utc(); // in seconds
    let offset_hours = offset / 3600; // Convert seconds to hours
    let offset_minutes = (offset.abs() % 3600) / 60; // Get remaining minutes
    let offset_str = format!("{:+03}:{:02}", offset_hours, offset_minutes);

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time error")
        .as_secs();

    // Find HEAD ref (e.g., refs/heads/master)
    let head_path = Path::new(".hit/HEAD");
    let head_contents = fs::read_to_string(head_path).expect("Failed to read .hit/HEAD");

    let ref_path = if head_contents.starts_with("ref: ") {
        let rel_path = head_contents[5..].trim();
        PathBuf::from(".hit").join(rel_path)
    } else {
        panic!("Invalid HEAD format");
    };

    // Try reading the parent commit SHA from the ref
    let parent_sha = fs::read_to_string(&ref_path).ok();

    // Begin commit body
    let mut commit_body = String::new();
    commit_body += &format!("tree {}\n", tree_sha);

    if let Some(parent_sha) = parent_sha {
        let parent_sha = parent_sha.trim();
        if !parent_sha.is_empty() {
            commit_body += &format!("parent {}\n", parent_sha);
        }
    }

    commit_body += &format!("author {} {} {}\n", author, timestamp, offset_str);
    commit_body += &format!("committer {} {} {}\n", author, timestamp, offset_str);
    commit_body += "\n";
    commit_body += message;
    commit_body += "\n";

    // Prepend header: "commit <len>\0"
    let mut full_commit = Vec::new();
    let header = format!("commit {}\0", commit_body.len());
    full_commit.extend_from_slice(header.as_bytes());
    full_commit.extend_from_slice(commit_body.as_bytes());

    // Hash the commit
    let mut hasher = Sha1::new();
    hasher.update(&full_commit);
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
            .write_all(&full_commit)
            .expect("Failed to compress commit");
        let compressed = encoder.finish().expect("Failed to finish compression");

        fs::write(&object_path, compressed).expect("Failed to write commit object");
    }

    // Update the ref (e.g., refs/heads/master)
    fs::write(&ref_path, format!("{}\n", hash_hex)).expect("Failed to update ref");

    // Output the commit SHA
    println!("{}", hash_hex);
    hash_hex
}
