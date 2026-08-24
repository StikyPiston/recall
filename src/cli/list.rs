use colored::Colorize;

use crate::help::home_dir;
use recall::{load_recall, Task};

pub fn list() {
    let tasks: Vec<Task> = load_recall(&(home_dir() + "/.recall"));

    if tasks.is_empty() {
        println!("{}", "󰄭 All tasks done!".green());
        return
    }

    let _pending: Vec<Task> = Vec::new();
    let _busy: Vec<Task> = Vec::new();
    let _done: Vec<Task> = Vec::new();
    let _back_burner: Vec<Task> = Vec::new();
}
