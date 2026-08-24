use std::fs;
use std::path::Path;

use recall::{todo_path, xp_path};

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
