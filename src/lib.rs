use serde::{Deserialize, Serialize};
use std::{
    env, fs,
    time::{SystemTime, UNIX_EPOCH},
};

#[derive(Clone, Deserialize, Serialize)]
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

impl Default for XP {
    fn default() -> Self {
        Self {
            xp: 0,
            last_checked: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        }
    }
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

// MARK: directory helpers
pub fn home_dir() -> String {
    let dir = dirs::home_dir();
    dir.map(|p| p.to_string_lossy().into_owned()).unwrap()
}

pub fn todo_path() -> String {
    match env::current_dir() {
        Ok(p) => {
            let p = p.to_string_lossy().into_owned();
            match fs::exists(p.clone() + "/TODO.recall") {
                Ok(true) => (p + "/TODO.recall").to_string(),
                Ok(false) => home_dir() + "/.recall",
                Err(_) => home_dir() + "/.recall",
            }
        }
        Err(_) => home_dir() + "/.recall",
    }
}

// MARK: xp
pub fn xp_path() -> String {
    home_dir() + "/.recall_xp"
}

pub fn load_xp() -> XP {
    fs::read_to_string(xp_path())
        .ok()
        .and_then(|s| serde_json::from_str(&s).ok())
        .unwrap_or_default()
}

pub fn save_xp(xp: XP) {
    let json = serde_json::to_string_pretty(&xp).unwrap();
    fs::write(xp_path(), json).unwrap();
}

pub fn increase_xp(amount: u32) {
    let mut xp = load_xp();
    xp.xp += amount;
    save_xp(xp);
}

pub fn decrease_xp(amount: u32) {
    let mut xp = load_xp();
    xp.xp -= amount;
    save_xp(xp);
}
