use std::collections::HashSet;
use recall::{load_recall, save_recall, Task, todo_path};

pub fn add(name: String, priority: u8, project: Option<String>) {
    let mut tasks: Vec<Task> = load_recall(&todo_path());
    let id = {
      let used: HashSet<u32> = tasks
          .iter()
          .map(|t| t.id)
          .collect();
      (0..).find(|&id| !used.contains(&id)).unwrap_or(u32::MAX)
    };

    tasks.push(Task {
        name: name,
        prio: priority,
        state: 0,
        id: id,
        project: match project {
            Some(p) => p,
            None => "*".to_string(),
        }
    });

    save_recall(&todo_path(), tasks);
}
