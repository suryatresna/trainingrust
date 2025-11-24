# Tutorial 4: Task Management System (Exercise)

## Overview
This is a hands-on exercise where you'll build a complete task management system from scratch. Unlike the previous tutorials, this one provides requirements and starter code, but you'll implement the functionality independently. This tutorial focuses on enums, pattern matching, and building a full CRUD (Create, Read, Update, Delete) system.

## Learning Objectives

By completing this exercise, you will:
- Define and use custom enums with `#[derive]` attributes
- Implement pattern matching with enum variants
- Build a complete CRUD system from scratch
- Work with auto-incrementing IDs
- Filter collections by multiple criteria
- Apply all concepts learned from Tutorials 1-3

## Requirements

### 1. Create a Priority Enum
Define an enum with three variants:
- `Low`
- `Medium`
- `High`

Add `#[derive(Debug, PartialEq)]` to enable debugging and comparison.

### 2. Create a Status Enum
Define an enum with three variants:
- `Todo`
- `InProgress`
- `Completed`

Add `#[derive(Debug, PartialEq)]` to enable debugging and comparison.

### 3. Create a Task Struct
Fields required:
```rust
id: u32                 // Unique identifier
title: String           // Task title
description: String     // Detailed description
priority: Priority      // Task priority level
status: Status          // Current task status
```

### 4. Create a TaskManager Struct
Fields required:
```rust
tasks: Vec<Task>        // Collection of all tasks
next_id: u32            // Auto-incrementing ID counter
```

### 5. Implement Methods for TaskManager

#### Required Methods

**`new() -> Self`**
- Creates an empty task manager
- Sets `next_id` to 1

**`create_task(&mut self, title: String, description: String, priority: Priority) -> u32`**
- Creates a new task with `Status::Todo`
- Assigns the current `next_id` and increments it
- Adds task to the collection
- Returns the assigned ID

**`get_task(&self, id: u32) -> Option<&Task>`**
- Finds and returns a reference to a task by ID
- Returns `None` if not found

**`update_status(&mut self, id: u32, status: Status) -> Result<(), String>`**
- Updates the status of a task
- Returns `Err` if task not found

**`delete_task(&mut self, id: u32) -> Result<String, String>`**
- Removes a task from the collection
- Returns `Ok` with the task title if found
- Returns `Err` if not found

**`get_tasks_by_status(&self, status: Status) -> Vec<&Task>`**
- Returns all tasks with the given status
- Uses iterator filtering and pattern matching

**`get_tasks_by_priority(&self, priority: Priority) -> Vec<&Task>`**
- Returns all tasks with the given priority
- Uses iterator filtering and pattern matching

**`count_by_status(&self, status: Status) -> usize`**
- Counts how many tasks have the given status

**`mark_completed(&mut self, id: u32) -> Result<(), String>`**
- Convenience method to mark a task as completed
- Internally calls `update_status` with `Status::Completed`

## Example Usage

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

Expected output:
```
Completed tasks: 1
High priority tasks: 1
```

## Starter Code

The file [main.rs](main.rs) currently contains a basic template. Replace it with your implementation:

```rust
#[derive(Debug, PartialEq)]
enum Priority {
    // TODO: Add your variants here
    // Low, Medium, High
}

#[derive(Debug, PartialEq)]
enum Status {
    // TODO: Add your variants here
    // Todo, InProgress, Completed
}

struct Task {
    // TODO: Add your fields here
}

struct TaskManager {
    // TODO: Add your fields here
}

impl TaskManager {
    // TODO: Implement all required methods
}

fn main() {
    // TODO: Test your implementation here
}
```

## Implementation Hints

### Understanding Enums
Enums in Rust allow you to define a type with a fixed set of possible values:
```rust
#[derive(Debug, PartialEq)]
enum Priority {
    Low,
    Medium,
    High,
}
```

The `#[derive(Debug, PartialEq)]` attributes:
- `Debug`: Enables printing with `{:?}` in format strings
- `PartialEq`: Allows using `==` to compare enum values

### Auto-Incrementing IDs
In the `create_task` method:
```rust
fn create_task(&mut self, title: String, description: String, priority: Priority) -> u32 {
    let id = self.next_id;
    self.next_id += 1;  // Increment for next task

    let task = Task {
        id,
        title,
        description,
        priority,
        status: Status::Todo,
    };

    self.tasks.push(task);
    id  // Return the ID
}
```

### Pattern Matching with Enums
When filtering by status or priority:
```rust
fn get_tasks_by_status(&self, status: Status) -> Vec<&Task> {
    self.tasks.iter()
        .filter(|task| task.status == status)
        .collect()
}
```

### Counting with Iterators
Instead of manually looping:
```rust
fn count_by_status(&self, status: Status) -> usize {
    self.get_tasks_by_status(status).len()
}
```

### Finding and Updating
```rust
fn update_status(&mut self, id: u32, status: Status) -> Result<(), String> {
    match self.tasks.iter_mut().find(|task| task.id == id) {
        Some(task) => {
            task.status = status;
            Ok(())
        }
        None => Err(String::from("Task not found"))
    }
}
```

## Testing Your Implementation

Add these tests to verify your code works:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_task() {
        let mut manager = TaskManager::new();
        let id = manager.create_task(
            String::from("Test"),
            String::from("Description"),
            Priority::High
        );
        assert_eq!(id, 1);
        assert_eq!(manager.tasks.len(), 1);
    }

    #[test]
    fn test_auto_increment() {
        let mut manager = TaskManager::new();
        let id1 = manager.create_task(
            String::from("Task 1"),
            String::from("First"),
            Priority::Low
        );
        let id2 = manager.create_task(
            String::from("Task 2"),
            String::from("Second"),
            Priority::High
        );
        assert_eq!(id1, 1);
        assert_eq!(id2, 2);
    }

    #[test]
    fn test_update_status() {
        let mut manager = TaskManager::new();
        let id = manager.create_task(
            String::from("Task"),
            String::from("Desc"),
            Priority::Medium
        );
        assert!(manager.update_status(id, Status::InProgress).is_ok());
        let task = manager.get_task(id).unwrap();
        assert_eq!(task.status, Status::InProgress);
    }

    #[test]
    fn test_delete_task() {
        let mut manager = TaskManager::new();
        let id = manager.create_task(
            String::from("To Delete"),
            String::from("Desc"),
            Priority::Low
        );
        assert!(manager.delete_task(id).is_ok());
        assert!(manager.get_task(id).is_none());
    }

    #[test]
    fn test_filter_by_status() {
        let mut manager = TaskManager::new();
        manager.create_task(String::from("Task 1"), String::from(""), Priority::Low);
        let id2 = manager.create_task(String::from("Task 2"), String::from(""), Priority::High);
        manager.update_status(id2, Status::Completed).unwrap();

        assert_eq!(manager.count_by_status(Status::Todo), 1);
        assert_eq!(manager.count_by_status(Status::Completed), 1);
    }

    #[test]
    fn test_filter_by_priority() {
        let mut manager = TaskManager::new();
        manager.create_task(String::from("Low Task"), String::from(""), Priority::Low);
        manager.create_task(String::from("High Task"), String::from(""), Priority::High);
        manager.create_task(String::from("Another High"), String::from(""), Priority::High);

        let high_tasks = manager.get_tasks_by_priority(Priority::High);
        assert_eq!(high_tasks.len(), 2);
    }

    #[test]
    fn test_mark_completed() {
        let mut manager = TaskManager::new();
        let id = manager.create_task(
            String::from("Task"),
            String::from("Description"),
            Priority::Medium
        );
        assert!(manager.mark_completed(id).is_ok());
        let task = manager.get_task(id).unwrap();
        assert_eq!(task.status, Status::Completed);
    }
}
```

Run tests with:
```bash
cargo test --bin tutor4
```

## Bonus Challenges (Optional)

Once you've completed the required functionality, try these extensions:

### 1. Get Incomplete Tasks
```rust
fn get_incomplete_tasks(&self) -> Vec<&Task>
```
Returns all tasks that are either `Todo` or `InProgress`.

**Hint**: Use a filter that checks if status is NOT `Completed`:
```rust
.filter(|task| task.status != Status::Completed)
```

### 2. Update Priority
```rust
fn update_priority(&mut self, id: u32, priority: Priority) -> Result<(), String>
```
Allows changing a task's priority after creation.

### 3. Sort by Priority
```rust
fn get_tasks_sorted_by_priority(&self) -> Vec<&Task>
```
Returns tasks sorted: High → Medium → Low

**Hint**: Create a helper function to convert Priority to a number:
```rust
fn priority_value(priority: &Priority) -> u8 {
    match priority {
        Priority::High => 3,
        Priority::Medium => 2,
        Priority::Low => 1,
    }
}
```

### 4. Task Statistics
```rust
struct TaskStatistics {
    total: usize,
    todo: usize,
    in_progress: usize,
    completed: usize,
}

fn get_statistics(&self) -> TaskStatistics
```
Create a struct that holds counts for each status.

### 5. Bulk Operations
```rust
fn delete_completed_tasks(&mut self) -> usize
```
Removes all completed tasks and returns how many were deleted.

## Key Concepts to Remember

### Enums vs Structs
- **Enums**: Fixed set of variants (Status has 3 possible values)
- **Structs**: Collection of named fields (Task has 5 fields)

### PartialEq for Enums
Allows comparing enum values:
```rust
if task.status == Status::Completed {
    // ...
}
```

### Auto-Incrementing Pattern
Useful for generating unique IDs:
```rust
let id = self.next_id;
self.next_id += 1;
```

### Iterator Methods Review
- `filter()`: Keep items matching a condition
- `find()`: Get first item matching a condition
- `position()`: Get index of first match
- `collect()`: Convert iterator into a collection
- `len()`: Count items
- `iter_mut()`: Iterate with mutable references

## Common Pitfalls

1. **Forgetting to increment `next_id`**: Always increment after assigning
2. **Comparing wrong types**: Ensure you're comparing `Status` with `Status`, not `&Status`
3. **Not handling errors**: Use `Result` for operations that can fail
4. **Forgetting `#[derive]` attributes**: Needed for `==` and debugging
5. **Ownership issues**: Remember that `filter()` returns references (`&Task`)

## Completion Checklist

- [ ] Define `Priority` enum with three variants
- [ ] Define `Status` enum with three variants
- [ ] Add `#[derive(Debug, PartialEq)]` to both enums
- [ ] Create `Task` struct with all required fields
- [ ] Create `TaskManager` struct with `tasks` and `next_id`
- [ ] Implement `new()` method
- [ ] Implement `create_task()` with auto-incrementing IDs
- [ ] Implement `get_task()` returning `Option<&Task>`
- [ ] Implement `update_status()` returning `Result`
- [ ] Implement `delete_task()` returning `Result`
- [ ] Implement `get_tasks_by_status()` using filters
- [ ] Implement `get_tasks_by_priority()` using filters
- [ ] Implement `count_by_status()` method
- [ ] Implement `mark_completed()` convenience method
- [ ] Write tests for your implementation
- [ ] All tests pass

## Running Your Solution

```bash
# Run the program
cargo run --bin tutor4

# Run tests
cargo test --bin tutor4

# Run with output
cargo test --bin tutor4 -- --nocapture

# Run a specific test
cargo test --bin tutor4 test_create_task
```

## Getting Help

If you get stuck:

1. **Review previous tutorials**:
   - [Tutorial 1](../tutor1/README.md) for structs and methods
   - [Tutorial 2](../tutor2/README.md) for `Result` and filtering
   - [Tutorial 3](../tutor3/README.md) for complex operations

2. **Check the Rust documentation**:
   - [Enums](https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html)
   - [Pattern Matching](https://doc.rust-lang.org/book/ch06-02-match.html)
   - [Iterator Methods](https://doc.rust-lang.org/std/iter/trait.Iterator.html)

3. **Look at the test cases** to understand expected behavior

4. **Implement one method at a time** and test each before moving on

## Debugging Tips

If your code doesn't compile or tests fail:

### Missing `PartialEq`
```
error: binary operation `==` cannot be applied to type `Status`
```
Solution: Add `#[derive(PartialEq)]` to your enum

### Borrow Checker Issues
```
error: cannot borrow `*self` as mutable because it is also borrowed as immutable
```
Solution: Ensure you're not holding immutable references while trying to mutate

### Type Mismatch
```
expected `Priority`, found `&Priority`
```
Solution: Dereference with `*` or compare references consistently

## Example Solution Structure

Here's how your final code structure should look:

```rust
#[derive(Debug, PartialEq)]
enum Priority { /* ... */ }

#[derive(Debug, PartialEq)]
enum Status { /* ... */ }

struct Task { /* ... */ }

struct TaskManager {
    tasks: Vec<Task>,
    next_id: u32,
}

impl TaskManager {
    fn new() -> Self { /* ... */ }
    fn create_task(&mut self, ...) -> u32 { /* ... */ }
    fn get_task(&self, id: u32) -> Option<&Task> { /* ... */ }
    fn update_status(&mut self, ...) -> Result<(), String> { /* ... */ }
    fn delete_task(&mut self, id: u32) -> Result<String, String> { /* ... */ }
    fn get_tasks_by_status(&self, ...) -> Vec<&Task> { /* ... */ }
    fn get_tasks_by_priority(&self, ...) -> Vec<&Task> { /* ... */ }
    fn count_by_status(&self, ...) -> usize { /* ... */ }
    fn mark_completed(&mut self, id: u32) -> Result<(), String> { /* ... */ }
}

fn main() { /* ... */ }

#[cfg(test)]
mod tests { /* ... */ }
```

## Previous Tutorials

- [Tutorial 1: Student Grade Management](../tutor1/README.md) - Structs, methods, and Option types
- [Tutorial 2: Library Management](../tutor2/README.md) - Result types and iterators
- [Tutorial 3: Shopping Cart](../tutor3/README.md) - Complex struct relationships

## Next Steps

After completing this exercise:
- Review all four tutorials to reinforce concepts
- Try building your own project using these patterns
- Explore the [Rust Book](https://doc.rust-lang.org/book/) for advanced topics
- Practice with more complex data structures like `HashMap` and `HashSet`
- Learn about traits and generics for more flexible code

Congratulations on completing the Rust Training tutorials!
