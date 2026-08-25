use colored::Colorize;

use recall::{load_recall, todo_path, Task};

pub fn list() {
    let tasks: Vec<Task> = load_recall(&todo_path());

    if tasks.is_empty() {
        println!("{}", "󰄭 All tasks done!".green());
        return;
    }

    let mut pending: Vec<Task> = Vec::new();
    let mut busy: Vec<Task> = Vec::new();
    let mut done: Vec<Task> = Vec::new();
    let mut back_burner: Vec<Task> = Vec::new();

    for mut t in tasks {
        if t.project.is_empty() {
            t.project = "*".to_string();
        }

        match t.state {
            0 => pending.push(t),
            1 => busy.push(t),
            2 => done.push(t),
            3 => back_burner.push(t),
            _ => panic!("Priorities cannot be greater than three"),
        }
    }

    println!("{}", " Tasks:".white());
    // completed tasks
    if !done.is_empty() {
        println!("{}", " Completed".white());
    }
    for t in done {
        let stat = "󰄲 ";
        let line = format!("{} {}: [{}] {} ({})", t.id, stat, t.project, t.name, t.prio);
        match t.prio {
            1 => println!("{}", line.green()),
            2 => println!("{}", line.yellow()),
            3 => println!("{}", line.red()),
            _ => panic!("Priorities cannot be greater than three"),
        }
    }
    // busy tasks
    if !busy.is_empty() {
        println!("{}", " Busy".white());
    }
    for t in busy {
        let stat = "󰥔 ";
        let line = format!("{} {}: [{}] {} ({})", t.id, stat, t.project, t.name, t.prio);
        match t.prio {
            1 => println!("{}", line.green()),
            2 => println!("{}", line.yellow()),
            3 => println!("{}", line.red()),
            _ => panic!("Priorities cannot be greater than three"),
        }
    }
    // pending tasks
    if !pending.is_empty() {
        println!("{}", " Pending".white());
    }
    for t in pending {
        let stat = " ";
        let line = format!("{} {}: [{}] {} ({})", t.id, stat, t.project, t.name, t.prio);
        match t.prio {
            1 => println!("{}", line.green()),
            2 => println!("{}", line.yellow()),
            3 => println!("{}", line.red()),
            _ => panic!("Priorities cannot be greater than three"),
        }
    }
    // back_burner tasks
    if !back_burner.is_empty() {
        println!("{}", "󰀼 Back-Burner".white());
    }
    for t in back_burner {
        let stat = " ";
        let line = format!("{} {}: [{}] {} ({})", t.id, stat, t.project, t.name, t.prio);
        match t.prio {
            1 => println!("{}", line.green()),
            2 => println!("{}", line.yellow()),
            3 => println!("{}", line.red()),
            _ => panic!("Priorities cannot be greater than three"),
        }
    }
}
