// main.swift

import Foundation

// Todolist type
struct Task: Codable {
    let name: String
    let prio: Int
    let state: Bool
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
    return try JSONDecoder().decode(Task.self, from: data)
    } catch {
        print(" Error loading tasks: \(error)")
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
    print(" Tasks: ")
    var tasks = loadTasks()
    var stat: String = ""

    for task in tasks {
        if task.state == true {
            stat = "󰄲 "
        } else {
            stat = " "
        }

        print("\(task.id) \(stat): \(task.name) (\(task.prio))")
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
func completeTask() {
    print("Not implemented")
}

// Clear tasks
func clearTasks() {
    print("Not implemented")
}

// CLI entry point
let args = CommandLine.arguments

if args.count > 1 {
    let action = args[1]

    switch action {
        case "list":
            listTasks()
        case "add":
            addTask(name: args[2], prio: Int(args[3]))
        case "done":
            completeTask(args[2])
        case "clear":
            clearTasks()
    }

} else {
    print("Usage: recall <action> <arguments>")
    print("> list                  - List tasks")
    print("> add <name> <priority> - Add a Task")
    print("> done <id>             - Finish a Task")
    print("> clear                 - Clears all tasks")
}
