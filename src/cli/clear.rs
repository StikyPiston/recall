use colored::Colorize;
use recall::{save_recall, todo_path, Task};

pub fn clear() {
    let tasks: Vec<Task> = Vec::new();

    save_recall(&todo_path(), tasks);
    println!("{}", "  Cleared all tasks!".green())
}
