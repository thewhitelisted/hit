// object is a word refering to disagreeing with something

use flate2::read::ZlibDecoder;
use std::io::Read;

// datatype for git objects epic rust enums
pub enum Object {
    Blob(Vec<u8>),
    Tree(Tree),
    Commit(Commit),
}

impl Object {
    pub fn read(sha: &str) -> Result<Self, String> {
        // Validate SHA is not empty
        if sha.is_empty() {
            return Err("SHA cannot be empty".into());
        }
        
        // Make sure SHA is at least 2 characters long
        if sha.len() < 2 {
            return Err(format!("Invalid SHA: '{}' is too short", sha));
        }
        
        // Build object path from SHA
        let path = format!(".hit/objects/{}/{}", &sha[..2], &sha[2..]);
        let compressed = std::fs::read(&path).map_err(|_| "Object not found")?;

        // decompress
        let mut decoder = ZlibDecoder::new(&compressed[..]);
        let mut data = Vec::new();
        decoder
            .read_to_end(&mut data)
            .map_err(|_| "Decompression failed")?;

        // Parse header
        if let Some(null_pos) = data.iter().position(|&b| b == 0) {
            let header = std::str::from_utf8(&data[..null_pos]).map_err(|_| "UTF-8 error")?;
            let content = &data[null_pos + 1..];

            if header.starts_with("blob ") {
                Ok(Object::Blob(content.to_vec()))
            } else if header.starts_with("tree ") {
                Tree::parse(content).map(Object::Tree)
            } else if header.starts_with("commit ") {
                Commit::parse(content).map(Object::Commit)
            } else {
                Err("Unknown object type".into())
            }
        } else {
            Err("Invalid object format".into())
        }
    }
}

// wht is this
pub struct Tree {
    pub entries: Vec<TreeEntry>,
}

// oh this is
#[derive(Debug, Clone)]
pub struct TreeEntry {
    pub mode: String,
    pub name: String,
    pub sha: String, // 40-char hex
}

impl Tree {
    pub fn parse(data: &[u8]) -> Result<Self, String> {
        let mut entries = Vec::new();
        let mut i = 0;

        while i < data.len() {
            // mode and name
            let mut mode = Vec::new();
            while data[i] != b' ' {
                mode.push(data[i]);
                i += 1;
            }
            i += 1; // skip space

            let mut name = Vec::new();
            while data[i] != 0 {
                name.push(data[i]);
                i += 1;
            }
            i += 1; // skip null byte... evil byte hack

            // SHA (20 bytes)
            let sha_bin = &data[i..i + 20];
            let sha = sha_bin.iter().map(|b| format!("{:02x}", b)).collect();
            i += 20;

            entries.push(TreeEntry {
                mode: String::from_utf8(mode).unwrap(),
                name: String::from_utf8(name).unwrap(),
                sha,
            });
        }

        Ok(Tree { entries })
    }
}

// if this goes wrong, we have a commitment issue :(
pub struct Commit {
    pub tree: String,
    pub parent: Option<String>,
    pub message: String,
}

impl Commit {
    pub fn parse(data: &[u8]) -> Result<Self, String> {
        let text = std::str::from_utf8(data).map_err(|_| "Invalid UTF-8 in commit")?;
        let lines = text.lines();

        let mut tree = String::new();
        let mut parent = None;
        let mut message = String::new();
        let mut in_message = false;

        for line in lines {
            if line.is_empty() {
                in_message = true;
                continue;
            }

            if in_message {
                message.push_str(line);
                message.push('\n');
            } else if line.starts_with("tree ") {
                tree = line[5..].to_string();
            } else if line.starts_with("parent ") {
                parent = Some(line[7..].to_string());
            }
        }

        Ok(Commit {
            tree,
            parent,
            message: message.trim().to_string(),
        })
    }
}
