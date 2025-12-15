// main.swift

import Foundation

// Todolist type
struct Task: Codable {
    let name: String
    let prio: Int
    let state: Bool
}

// Paths
let fileManager    = FileManager.default
let homeDir        = fileManager.homeDirectoryForCurrentUser
let todoListURL    = homeDir.appendingPathComponent(".recall")

// Load todolist
func loadTasks() throws -> [Task] {
    let data = try Data(contentsOf: todoListURL)
    let decoder = JSONDecoder()
    return try decoder.decode([Task].self, from: data)
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

}

// Add task
func addTask() {

}

// Complete task
func completeTask() {

}

// Clear tasks
func clearTasks() {

}

// CLI entry point
let args = CommandLine.arguments

if args.count > 1 {
    let action = args[1]

    switch action {
        case "list":
            listTasks()
        case "add":
            addTask(args[2], args[3])
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
