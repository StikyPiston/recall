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
