use colored::Colorize;

use recall::{Task, decrease_xp, load_recall, save_recall, todo_path};

pub fn undo(id: u32) {
    let mut tasks: Vec<Task> = load_recall(&todo_path());

    let ta = tasks.clone();
    for (i, t) in ta.iter().enumerate() {
        if t.id == id {
            let prev_state = t.state;

            tasks[i].state = 0;
            save_recall(&todo_path(), tasks.clone());

            if prev_state == 2 {
                let penalty = t.prio * 10;
                decrease_xp(penalty as u32);
                println!("{}", format!("󰓑 Lost {penalty} XP").yellow())
            }
        }
    }
}
