use crate::utils::hash_object::resolve_head;
use crate::utils::index::{Index, IndexEntry};
use crate::utils::objects::{Object, TreeEntry};
use crate::utils::config::get_config_value;
use flate2::Compression;
use flate2::write::ZlibEncoder;
use sha1::{Digest, Sha1};
use std::collections::{BTreeMap, HashMap};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

pub fn commit(message: &str) {
    let index = Index::load();

    if index.entries.is_empty() {
        eprintln!("nothing to commit");
        return;
    }

    let head_sha = resolve_head();
    if let Some(head) = &head_sha {
        if index_matches_head(&index, head) {
            println!("Nothing to commit — index matches HEAD.");
            return;
        }
    }

    let tree_sha = build_tree_from_index(&index);
    let commit_sha = write_commit(&tree_sha, message);
    update_head(&commit_sha);

    println!("[{}] {}", &commit_sha[..7], message);
}

fn index_matches_head(index: &Index, head_sha: &str) -> bool {
    let head_tree_map = load_tree_map_from_commit(head_sha);

    index.entries.iter().all(|entry| {
        let path = PathBuf::from(&entry.path);
        head_tree_map.get(&path) == Some(&entry.sha)
    })
}

pub fn load_tree_map_from_commit(commit_sha: &str) -> HashMap<PathBuf, String> {
    let commit_obj = Object::read(commit_sha).expect("Failed to read commit");
    let tree_sha = match commit_obj {
        Object::Commit(c) => c.tree,
        _ => panic!("Not a commit object"),
    };

    let mut map = HashMap::new();
    build_tree_map_recursive(&tree_sha, PathBuf::from(""), &mut map);
    map
}

fn build_tree_map_recursive(tree_sha: &str, base: PathBuf, map: &mut HashMap<PathBuf, String>) {
    let obj = Object::read(tree_sha).expect("Failed to read tree object");
    let tree = match obj {
        Object::Tree(tree) => tree,
        _ => panic!("Not a tree object"),
    };

    for entry in tree.entries {
        let path = base.join(entry.name);
        match entry.mode.as_str() {
            "100644" | "100755" => {
                map.insert(path, entry.sha);
            }
            "40000" => {
                build_tree_map_recursive(&entry.sha, path, map);
            }
            _ => {}
        }
    }
}

fn build_tree_from_index(index: &Index) -> String {
    let mut path_map: BTreeMap<PathBuf, Vec<(PathBuf, &IndexEntry)>> = BTreeMap::new();

    for entry in &index.entries {
        let path = PathBuf::from(&entry.path);
        let parent = path.parent().unwrap_or_else(|| Path::new("")).to_path_buf();
        path_map.entry(parent).or_default().push((path, entry));
    }

    fn build_tree(
        dir: &Path,
        path_map: &BTreeMap<PathBuf, Vec<(PathBuf, &IndexEntry)>>,
    ) -> (Vec<TreeEntry>, String) {
        let mut entries = Vec::new();
        let mut raw = Vec::new();

        for (path, entry) in path_map.get(dir).cloned().unwrap_or_default() {
            let name = path.file_name().unwrap().to_string_lossy().to_string();

            let tree_entry = TreeEntry {
                mode: entry.mode.clone(),
                name: name.clone(),
                sha: entry.sha.clone(),
            };

            entries.push(tree_entry.clone());

            raw.extend_from_slice(format!("{} {}\0", entry.mode, name).as_bytes());
            let sha_bin = hex::decode(&entry.sha).unwrap();
            raw.extend_from_slice(&sha_bin);
        }

        let mut tree_object = Vec::new();
        tree_object.extend_from_slice(format!("tree {}\0", raw.len()).as_bytes());
        tree_object.extend_from_slice(&raw);

        let mut hasher = Sha1::new();
        hasher.update(&tree_object);
        let sha = hasher.finalize();
        let sha_hex = format!("{:x}", sha);

        let dir = format!(".hit/objects/{}", &sha_hex[..2]);
        let file = format!("{}", &sha_hex[2..]);
        let object_path = Path::new(&dir).join(file);

        if !object_path.exists() {
            fs::create_dir_all(&dir).unwrap();
            let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
            encoder.write_all(&tree_object).unwrap();
            let compressed = encoder.finish().unwrap();
            fs::write(&object_path, compressed).unwrap();
        }

        (entries, sha_hex)
    }

    let (_, root_tree_sha) = build_tree(Path::new(""), &path_map);
    root_tree_sha
}

fn write_commit(tree_sha: &str, message: &str) -> String {
    let (name, email) = get_author_info();
    let author = format!("{} <{}>", name.unwrap(), email.unwrap());


    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let parent = resolve_head();

    let mut content = String::new();
    content += &format!("tree {}\n", tree_sha);
    if let Some(p) = parent.clone() {
        content += &format!("parent {}\n", p);
    }
    content += &format!("author {} {} +0000\n", author, timestamp);
    content += &format!("committer {} {} +0000\n", author, timestamp);
    content += "\n";
    content += message;
    content += "\n";

    let full = format!("commit {}\0{}", content.len(), content);

    let mut hasher = Sha1::new();
    hasher.update(full.as_bytes());
    let sha = hasher.finalize();
    let sha_hex = format!("{:x}", sha);

    let dir = format!(".hit/objects/{}", &sha_hex[..2]);
    let file = format!("{}", &sha_hex[2..]);
    let object_path = Path::new(&dir).join(file);

    if !object_path.exists() {
        fs::create_dir_all(&dir).unwrap();
        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
        encoder.write_all(full.as_bytes()).unwrap();
        let compressed = encoder.finish().unwrap();
        fs::write(&object_path, compressed).unwrap();
    }

    sha_hex
}

fn update_head(new_sha: &str) {
    let head_path = Path::new(".hit/HEAD");
    let head_contents = fs::read_to_string(head_path).unwrap();

    if head_contents.starts_with("ref: ") {
        let ref_path = head_contents[5..].trim();
        let full_ref_path = Path::new(".hit").join(ref_path);
        fs::write(full_ref_path, format!("{}\n", new_sha)).unwrap();
    } else {
        fs::write(".hit/HEAD", format!("{}\n", new_sha)).unwrap(); // detached HEAD
    }
}

fn get_author_info() -> (Option<String>, Option<String>) {
    let name = get_config_value("user", "name").unwrap_or(Some("You".to_owned()));
    let email = get_config_value("user", "email").unwrap_or(Some("you@example.com".to_owned()));

    (name, email)
}
