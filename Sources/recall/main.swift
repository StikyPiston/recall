// main.swift

import Foundation

// Todolist type
struct Task: Codable {
    let name: String
    let prio: Int
    var state: Bool
    let id: Int
}

// Paths
let fileManager    = FileManager.default
let homeDir        = fileManager.homeDirectoryForCurrentUser
let todoListURL    = homeDir.appendingPathComponent(".recall")

// Load todolist
func loadTasks() -> [Task] {
    do {
        let data = try Data(contentsOf: todoListURL)
        return try JSONDecoder().decode([Task].self, from: data)
    } catch {
        print(" Error loading tasks: \(error)")
        return []
    }
}

// Save todolist
func saveTasks(_ tasks: [Task]) throws {
    let encoder = JSONEncoder()
    encoder.outputFormatting = .prettyPrinted

    let data = try encoder.encode(tasks)
    try data.write(to: todoListURL, options: .atomic)
}

// List tasks
func listTasks() {
    let tasks = loadTasks()
    
    if tasks.isEmpty == false  {
        print(" Tasks: ")
        var stat: String = ""

        for task in tasks {
            if task.state == true {
                stat = "󰄲 "
            } else {
                stat = " "
            }

            print("\(task.id) \(stat): \(task.name) (\(task.prio))")
        }
    } else {
        print("󰄭 All tasks done!")
    }
}

// Add task
func addTask(name: String, prio: Int) {
    guard (1...3).contains(prio) else {
        print(" Priority must be between 1 and 3")
        return 
    }

    var tasks = loadTasks()

    let usedIDs = Set(tasks.map { $0.id })
    var newID = 0
    while usedIDs.contains(newID) {
        newID += 1
    }

    let newTask = Task(
        name: name,
        prio: prio,
        state: false,
        id: newID,
    )

    tasks.append(newTask)

    do {
        try saveTasks(tasks)
    } catch {
        print(" Failed to save tasks: \(error)")
    }
}

// Complete task
func completeTask(id: Int) {
    do {
        var tasks = loadTasks()
        
        if let index = tasks.firstIndex(where: { $0.id == id }) {
            tasks[index].state = true
            try saveTasks(tasks)
        } else {
            print(" Task with ID \(id) does not exist.")
        }
    } catch {
        print(" Failed to complete task: \(error)")
    }
}

// Clear tasks
func clearTasks() {
    do {
        try saveTasks([])
        print("  Cleared all tasks!")
    } catch {
        print(" Error clearing tasks: \(error)")
    }
}

// CLI entry point
let args = CommandLine.arguments

if args.count > 1 {
    let action = args[1]

    switch action {
        case "list":
            listTasks()
        case "add":
            addTask(name: args[2], prio: Int(args[3]) ?? 1)
        case "done":
            completeTask(id: Int(args[2]) ?? 1)
        case "clear":
            clearTasks()
        case "help":
            print("Usage: recall <action> <arguments>")
            print("> list                  - List tasks")
            print("> add <name> <priority> - Add a Task")
            print("> done <id>             - Finish a Task")
            print("> clear                 - Clears all tasks")
        default:
            print("Unknown command. Run recall help for help")
    }

} else {
    print("Usage: recall <action> <arguments>")
    print("> list                  - List tasks")
    print("> add <name> <priority> - Add a Task")
    print("> done <id>             - Finish a Task")
    print("> clear                 - Clears all tasks")
}
