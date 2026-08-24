use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Deserialize, Serialize)]
pub struct Task {
    name: String,
    prio: u8,
    state: u8,
}

#[derive(Deserialize, Serialize)]
pub struct XP {
    xp: u32,
    last_checked: u64,
}

pub fn load_recall(path: &str) -> Vec<Task> {
    fs::read_to_string(path)
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default()
}

pub fn save_recall(path: &str, tasks: Vec<Task>) {
    let json = serde_json::to_string_pretty(&tasks).unwrap();
    fs::write(path, json).unwrap();
}
