use colored::Colorize;

use recall::{load_recall, save_recall, todo_path, Task};
pub fn busy(id: u32) {
    let mut tasks: Vec<Task> = load_recall(&todo_path());

    let ta = tasks.clone();
    for (i, t) in ta.iter().enumerate() {
        if t.id == id {
            tasks[i].state = 1;
            save_recall(&todo_path(), tasks.clone());
            println!("{}", format!("󰥔 Set task {id} to busy").yellow());
            return;
        }
    }

    println!("{}", " Task does not exist".red())
}
