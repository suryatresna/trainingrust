# Question: Task Management System
Create a task management system that tracks tasks with different priorities and statuses.
# Requirements:

Create a Priority enum with:
* Low
* Medium
* High


Create a Status enum with:
* Todo
* InProgress
* Completed


Create a Task struct with:
```rust
id: u32
title: String
description: String
priority: Priority
status: Status
```


Create a TaskManager struct with:
```rust
tasks: Vec<Task>
next_id: u32 (for auto-incrementing IDs)
```

Implement the following methods for TaskManager:

```rust
new() - Constructor that creates an empty task manager with next_id starting at 1
create_task(&mut self, title: String, description: String, priority: Priority) -> u32 - Creates a new task with status Todo and returns its ID
get_task(&self, id: u32) -> Option<&Task> - Finds a task by ID
update_status(&mut self, id: u32, status: Status) -> Result<(), String> - Updates task status

Return Err if task not found


delete_task(&mut self, id: u32) -> Result<String, String> - Deletes a task

Return Ok with task title if found
Return Err if not found


get_tasks_by_status(&self, status: Status) -> Vec<&Task> - Returns all tasks with given status
get_tasks_by_priority(&self, priority: Priority) -> Vec<&Task> - Returns all tasks with given priority
count_by_status(&self, status: Status) -> usize - Counts tasks with given status
mark_completed(&mut self, id: u32) -> Result<(), String> - Shortcut to mark task as completed
```

Example Usage:

```rust
fn main() {
    let mut manager = TaskManager::new();
    
    let id1 = manager.create_task(
        String::from("Fix bug"),
        String::from("Fix login issue"),
        Priority::High
    );
    
    let id2 = manager.create_task(
        String::from("Write tests"),
        String::from("Add unit tests"),
        Priority::Medium
    );
    
    manager.update_status(id1, Status::InProgress).unwrap();
    manager.mark_completed(id1).unwrap();
    
    println!("Completed tasks: {}", manager.count_by_status(Status::Completed));
    
    let high_priority = manager.get_tasks_by_priority(Priority::High);
    println!("High priority tasks: {}", high_priority.len());
}
```

Starter Code:

```rust
#[derive(Debug, PartialEq)]
enum Priority {
    // Your variants here
}

#[derive(Debug, PartialEq)]
enum Status {
    // Your variants here
}

struct Task {
    // Your fields here
}

struct TaskManager {
    // Your fields here
}

impl TaskManager {
    // Your methods here
}

fn main() {
    // Test your implementation
}
```

# Notes:

The #[derive(Debug, PartialEq)] attributes are needed for comparing enums and debugging
PartialEq allows you to use == to compare enum values
When filtering by status or priority, you'll need to compare enum variants

Bonus Challenge (Optional):

Add a get_incomplete_tasks(&self) -> Vec<&Task> method (Todo + InProgress)
Add an update_priority(&mut self, id: u32, priority: Priority) -> Result<(), String> method
Implement a way to list tasks sorted by priority (High → Medium → Low)