use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Deserialize, Serialize)]
pub struct Task {
    pub name: String,
    pub prio: u8,
    pub state: u8,
    pub id: u32,
    pub project: String,
}

#[derive(Deserialize, Serialize)]
pub struct XP {
    pub xp: u32,
    pub last_checked: u64,
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
