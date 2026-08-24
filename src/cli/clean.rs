use colored::Colorize;
use recall::{Task, load_recall, save_recall, todo_path};

pub fn clean() {
    let tasks: Vec<Task> = load_recall(&todo_path());
    let mut remaining: Vec<Task> = Vec::new();

    for t in tasks {
        if t.state < 2 || t.state == 3 {
            remaining.push(t);
        }
    }

    save_recall(&todo_path(), remaining);
    println!("{}", " Cleared completed tasks!".green())
}
