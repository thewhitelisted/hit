use std::env;
use std::fs;

pub fn initialize_repo() {
    let path = env::current_dir().unwrap_or_else(|_| {
        eprintln!("Error: Failed to get current directory");
        std::process::exit(1);
    });
    if !path.exists() {
        eprintln!("Error: Directory does not exist");
        std::process::exit(1);
    }

    // add a .hit directory
    let hit_dir = path.join(".hit");
    if hit_dir.exists() {
        eprintln!("Error: .hit directory already exists");
        std::process::exit(1);
    }
    if let Err(e) = fs::create_dir(&hit_dir) {
        eprintln!("Error: Failed to create .hit directory: {}", e);
        std::process::exit(1);
    }

    // add a objects directory
    let objects_dir = hit_dir.join("objects");
    if objects_dir.exists() {
        eprintln!("Error: .hit/objects directory already exists");
        std::process::exit(1);
    }
    if let Err(e) = fs::create_dir(&objects_dir) {
        eprintln!("Error: Failed to create .hit/objects directory: {}", e);
        std::process::exit(1);
    }

    // add a refs directory
    let refs_dir = hit_dir.join("refs");
    if refs_dir.exists() {
        eprintln!("Error: .hit/refs directory already exists");
        std::process::exit(1);
    }
    if let Err(e) = fs::create_dir(&refs_dir) {
        eprintln!("Error: Failed to create .hit/refs directory: {}", e);
        std::process::exit(1);
    }

    // add a refs/heads directory
    let heads_dir = refs_dir.join("heads");
    if heads_dir.exists() {
        eprintln!("Error: .hit/refs/heads directory already exists");
        std::process::exit(1);
    }
    if let Err(e) = fs::create_dir(&heads_dir) {
        eprintln!("Error: Failed to create .hit/refs/heads directory: {}", e);
        std::process::exit(1);
    }

    // add a refs/tags directory
    let tags_dir = refs_dir.join("tags");
    if tags_dir.exists() {
        eprintln!("Error: .hit/refs/tags directory already exists");
        std::process::exit(1);
    }
    if let Err(e) = fs::create_dir(&tags_dir) {
        eprintln!("Error: Failed to create .hit/refs/tags directory: {}", e);
        std::process::exit(1);
    }

    // create empty refs/heads/master file
    let master_file = heads_dir.join("master");
    if master_file.exists() {
        eprintln!("Error: .hit/refs/heads/master file already exists");
        std::process::exit(1);
    }
    if let Err(e) = fs::write(&master_file, "") {
        eprintln!("Error: Failed to create .hit/refs/heads/master file: {}", e);
        std::process::exit(1);
    }

    // add a HEAD file
    let head_file = hit_dir.join("HEAD");
    if head_file.exists() {
        eprintln!("Error: .hit/HEAD file already exists");
        std::process::exit(1);
    }
    if let Err(e) = fs::write(&head_file, "ref: refs/heads/master") {
        eprintln!("Error: Failed to create .hit/HEAD file: {}", e);
        std::process::exit(1);
    }

    println!("Initialized empty Hit repository in {}", hit_dir.display());
}