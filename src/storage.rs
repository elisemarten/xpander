use crate::models::Snapshot;
use std::fs;

const FILE: &str = "data/snapshots.json";

pub fn save(snapshot: Snapshot) {
    let mut list = load();

    list.push(snapshot);

    let json = serde_json::to_string_pretty(&list).unwrap();

    fs::write(FILE, json).unwrap();
}

pub fn load() -> Vec<Snapshot> {
    match fs::read_to_string(FILE) {
        Ok(content) => serde_json::from_str(&content).unwrap_or_default(),
        Err(_) => Vec::new(),
    }
}
