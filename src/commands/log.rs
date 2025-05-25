use crate::utils::objects;
use chrono::{DateTime, Utc};
use std::fs;
use std::path::Path;

pub fn log() {
    let mut current = resolve_head().expect("HEAD not found");

    while let Some(commit) = read_commit(&current) {
        print_commit(&current, &commit);
        if let Some(parent) = &commit.parent {
            current = parent.clone();
        } else {
            break;
        }
    }
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

fn read_commit(sha: &str) -> Option<objects::Commit> {
    match objects::Object::read(sha).ok()? {
        objects::Object::Commit(c) => Some(c),
        _ => None,
    }
}

fn print_commit(sha: &str, commit: &objects::Commit) {
    println!("commit {}", sha);
    println!("{}", commit.timestamp);

    // Parse and format timestamp
    let datetime: DateTime<Utc> = DateTime::from_timestamp(commit.timestamp as i64, 0).unwrap();

    println!("Author: {}", commit.author);
    println!(
        "Date:   {} {}\n",
        datetime.format("%a %b %e %T %Y"),
        commit.timezone
    );

    println!("    {}\n", commit.message);
}
