use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct IndexEntry {
    pub path: String,
    pub sha: String,
    pub mode: String,
}

#[derive(Serialize, Deserialize)]
pub struct Index {
    pub entries: Vec<IndexEntry>,
}

impl Index {
    pub fn load() -> Self {
        let path = ".hit/index";
        if std::path::Path::new(path).exists() {
            let data = std::fs::read_to_string(path).expect("Failed to read index");
            serde_json::from_str(&data).expect("Invalid index format")
        } else {
            Index {
                entries: Vec::new(),
            }
        }
    }

    pub fn save(&self) {
        let json = serde_json::to_string_pretty(&self).expect("Failed to serialize index");
        std::fs::write(".hit/index", json).expect("Failed to write index");
    }

    /// Insert or update an entry by path
    pub fn add(&mut self, entry: IndexEntry) {
        if let Some(e) = self.entries.iter_mut().find(|e| e.path == entry.path) {
            *e = entry;
        } else {
            // load .hitignore
            let hitignore = std::fs::read_to_string(".hitignore").unwrap_or_default();
            // Check if the entry is ignored
            // check if the path of the file is in any .hitignore directories
            // eg. ".hitignore" contains "/logs/" and the entry path is "logs/error.log"
            if !hitignore.lines().any(|line| {
                let line = line.trim();
                !line.is_empty() && entry.path.starts_with(line)
            }) {
                self.entries.push(entry);
            }
        }
    }

    /// Remove an entry by path (for deletes)
    pub fn remove(&mut self, path: &str) {
        self.entries.retain(|e| e.path != path);
    }
}
