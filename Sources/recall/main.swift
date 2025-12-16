// main.swift

import Foundation

// Todolist type
struct Task: Codable {
    let name: String
    let prio: Int
    var state: Bool
    let id: Int
}

// XP type
struct XP: Codable {
    var XP: Int
}

// Paths
let fileManager    = FileManager.default
let homeDir        = fileManager.homeDirectoryForCurrentUser
let todoListURL    = homeDir.appendingPathComponent(".recall")
let xpURL          = homeDir.appendingPathComponent(".recall_xp")

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

// Load XP File
func loadXP() -> XP {
    do {
        let data = try Data(contentsOf: xpURL)
        return try JSONDecoder().decode(XP.self, from: data)
    } catch {
        // First run or corrupted file → start fresh
        return XP(XP: 0)
    }
}

// Save todolist
func saveTasks(_ tasks: [Task]) throws {
    let encoder = JSONEncoder()
    encoder.outputFormatting = .prettyPrinted

    let data = try encoder.encode(tasks)
    try data.write(to: todoListURL, options: .atomic)
}

// Save XP
func saveXP(_ xp: XP) throws {
    let encoder = JSONEncoder()
    encoder.outputFormatting = .prettyPrinted

    let data = try encoder.encode(xp)
    try data.write(to: xpURL, options: .atomic)
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

// Increase XP
func increaseXP(_ amount: Int) {
    var xp = loadXP()
    xp.XP += amount

    do {
        try saveXP(xp)
    } catch {
        print(" Failed to save XP: \(error)")
    }
}

// Decrease XP
func decreaseXP(_ amount: Int) {
    var xp = loadXP()
    xp.XP -= amount

    do {
        try saveXP(xp)
    } catch {
        print(" Failed to save XP: \(error)")
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

            let reward = tasks[index].prio * 10
            increaseXP(reward)

            print("󱕣 Earned \(reward) XP")
        } else {
            print(" Task with ID \(id) does not exist.")
        }
    } catch {
        print(" Failed to complete task: \(error)")
    }
}

// Undo task
func undoTask(id: Int) {
    do {
        var tasks = loadTasks()

        if let index = tasks.firstIndex(where: { $0.id == id }) {
            tasks[index].state = false
            try saveTasks(tasks)
            
            let penalty = tasks[index].prio * 10
            decreaseXP(penalty)

            print("󰓑 Lost \(penalty) XP")
        }
    } catch {
        print(" Failed to undo task: \(error)")
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
        case "undo":
            undoTask(id: Int(args[2]) ?? 1)
        case "clear":
            clearTasks()
        case "xp":
            let xp = loadXP()
            print(" XP: \(xp.XP)")
        case "help":
            print("Usage: recall <action> <arguments>")
            print("> add <name> <priority> - Add a Task")
            print("> clear                 - Clears all tasks")
            print("> done <id>             - Finish a Task")
            print("> list                  - List tasks")
            print("> undo <id>             - Undo a Task")
            print("> xp                    - Prints XP amount")
        default:
            print("Unknown command. Run recall help for help")
    }

} else {
    print("Usage: recall <action> <arguments>")
    print("> add <name> <priority> - Add a Task")
    print("> clear                 - Clears all tasks")
    print("> done <id>             - Finish a Task")
    print("> list                  - List tasks")
    print("> undo <id>             - Undo a Task")
    print("> xp                    - Prints XP amount")
}
