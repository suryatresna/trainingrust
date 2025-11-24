#[derive(PartialEq)]
enum Priority {
    Low,
    Medium,
    High,
}

#[derive(PartialEq)]
enum Status {
    Todo,
    InProgress,
    Completed,
}

struct Task {
    id: u32,
    title: String,
    description: String,
    priority: Priority,
    status: Status,
}

struct TaskManager {
    tasks: Vec<Task>,
    next_id: u32,
}

impl TaskManager {
    fn new() -> Self {
        TaskManager {
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    fn create_task(&mut self, title: String, description: String, priority: Priority) -> u32 {
        let task = Task {
            id: self.next_id,
            title,
            description,
            priority,
            status: Status::Todo,
        };
        self.tasks.push(task);
        self.next_id += 1;

        self.next_id - 1
    }

    fn update_status(&mut self, id: u32, status: Status) -> Result<(), String> {
        match self.tasks.iter_mut().find(|task| task.id == id) {
            Some(task) => {
                task.status = status;
                Ok(())
            }
            None => Err(format!("Task with id {} not found", id)),
        } 
    }

    fn delete_task(&mut self, id: u32) -> Result<String, String> {
        if let Some(index) = self.tasks.iter().position(|task| task.id == id) {
            self.tasks.swap_remove(index);
            Ok(format!("Task with id {} successfully removed", id))
        } else {
            Err(format!("Task with id {} not found", id))
        }
    }

    fn get_tasks_by_status(&self, status: Status) -> Vec<&Task> {
        self.tasks.iter().filter(|task| task.status == status).collect()
    }

    fn get_tasks_by_priority(&self, priority: Priority) -> Vec<&Task> {
        self.tasks.iter().filter(|task| task.priority == priority).collect()
    }

    fn count_by_status(&self, status: Status) -> usize {
        self.tasks.iter().filter(|task|task.status == status).count()
    }

    fn mark_completed(&mut self, id: u32) -> Result<(), String> {
        self.update_status(id, Status::Completed)
    }
}

fn main() {
    let mut manager = TaskManager::new();
    
    let id1 = manager.create_task(
        String::from("Fix bug"),
        String::from("Fix login issue"),
        Priority::High
    );
    
    let _id2 = manager.create_task(
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