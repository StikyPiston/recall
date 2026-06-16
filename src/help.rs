use dirs;
use serde::{Deserialize, Serialize};

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
