use colored::Colorize;

use recall::{increase_xp, load_recall, save_recall, todo_path, Task};

pub fn done(id: u32) {
    let mut tasks: Vec<Task> = load_recall(&todo_path());

    let ta = tasks.clone();
    for (i, t) in ta.iter().enumerate() {
        if t.id == id {
            tasks[i].state = 2;
            save_recall(&todo_path(), tasks.clone());

            let reward = t.prio * 10;
            increase_xp(reward as u32);
            println!("{}", format!("󱕣 Earned {reward} XP").green());
            return;
        }
    }

    println!("{}", " Task does not exist".red())
}
