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
