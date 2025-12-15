// main.swift

import Foundation

// Todolist type
struct List: Codable {
    let name: String
    let prio: Int
    let state: Bool
}

// Paths
let fileManager = FileManager.default
let homeDir     = fileManager.homeDirectoryForCurrentUser
let todoList    = homeDir.appendingPathComponent(".recall")

// Load todolist
func loadList() -> List {
    if fileManager.fileExists(atPath: todoList.path) {
        do {
            let data = try Data(contentsOf: todoList)
            return try JSONDecoder().decode(List.self, from: data)
        } catch {
            print("Failed to load todolist: \(error)")
        }
    }
}
