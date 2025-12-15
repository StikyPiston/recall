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
