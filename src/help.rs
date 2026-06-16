use dirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

// MARK: types
#[derive(Serialize, Deserialize)]
struct Task {
    name: String,
    prio: u16,
    state: u16,
    id: u32,
}

#[derive(Serialize, Deserialize)]
struct XP {
    xp: u32,
    last_checked: u64,
}

// MARK: directory helpers
pub fn home_dir() -> String {
    let dir = dirs::home_dir();
    return dir.map(|p| p.to_string_lossy().into_owned()).unwrap();
}

pub fn todo_path() -> String {
    home_dir() + "/.recall"
}

pub fn xp_path() -> String {
    home_dir() + "/.recall_xp"
}

// MARK: save/load helpers
pub fn ensure_file(path: &str, content: &[u8]) {
    if !Path::new(path).exists() {
        let _ = fs::write(path, content);
    }
}

pub fn init_storage() {
    ensure_file(&todo_path(), b"[]");
    ensure_file(&xp_path(), br#"{"XP":0,"last_checked":0}"#);
}
