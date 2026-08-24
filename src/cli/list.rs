use colored::Colorize;

use crate::help::home_dir;
use recall::{load_recall, Task};

pub fn list() {
    let tasks: Vec<Task> = load_recall(&(home_dir() + "/.recall"));

    if tasks.is_empty() {
        println!("{}", "󰄭 All tasks done!".green());
        return
    }

    let mut pending: Vec<Task> = Vec::new();
    let mut busy: Vec<Task> = Vec::new();
    let mut done: Vec<Task> = Vec::new();
    let mut back_burner: Vec<Task> = Vec::new();

    for mut t in tasks {
        if t.project == "".to_string() {
            t.project = "*".to_string();
        }

        match t.state {
            0 => pending.push(t),
            1 => busy.push(t),
            2 => done.push(t),
            3 => back_burner.push(t),
            _ => panic!("Priorities cannot be more than three"),
        }
    }

    println!("{}", " Tasks:".white());
}
