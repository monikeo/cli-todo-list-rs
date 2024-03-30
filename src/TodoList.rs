use crate::Status::Status;
use crate::Table::*;
use crate::Task::Task;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct TodoList {
    tasks: Vec<Task>,
}

impl TodoList {
    pub fn new() -> Self {
        Self {
            tasks: Vec::<Task>::new(),
        }
    }

    pub fn import(&mut self, tasks: &[Task]) {
        /*
        for task in tasks.iter() {
            self.tasks.push(task.clone());
        }
        */
        self.tasks.extend_from_slice(tasks);
        self.update_id();
    }

    pub fn add(&mut self, name: &str) {
        let mut task = Task::new(name);
        let len = self.tasks.len();
        task.set_id(len + 1);
        self.tasks.push(task);
        self.update_id();
    }

    pub fn delete_by_id(&mut self, id: usize) {
        self.tasks.retain(|task| task.id() != id);
        self.update_id();
    }

    pub fn delete_by_name(&mut self, name: &str) {
        self.tasks.retain(|task| task.name() != name);
        self.update_id();
    }

    pub fn get_task_by_id(&self, id: usize) -> Option<&Task> {
        self.tasks.iter().find(|task| task.id() == id)
    }

    pub fn get_task_by_name(&self, name: &str) -> Option<&Task> {
        self.tasks.iter().find(|task| task.name() == name)
    }

    pub fn get_all_task(&self) -> &[Task] {
        &self.tasks
    }

    pub fn get_completed_tasks(&self) -> Option<Vec<&Task>> {
        let filtered_tasks: Vec<&Task> = self
            .tasks
            .iter()
            .filter(|task| task.status() == "completed")
            .collect();
        if filtered_tasks.is_empty() {
            None
        } else {
            Some(filtered_tasks)
        }
    }

    pub fn get_cancelled_tasks(&self) -> Option<Vec<&Task>> {
        let filtered_tasks: Vec<&Task> = self
            .tasks
            .iter()
            .filter(|task| task.status() == "cancelled")
            .collect();
        if filtered_tasks.is_empty() {
            None
        } else {
            Some(filtered_tasks)
        }
    }

    pub fn get_todo_tasks(&self) -> Option<Vec<&Task>> {
        let filtered_tasks: Vec<&Task> = self
            .tasks
            .iter()
            .filter(|task| task.status() == "pending")
            .collect();
        if filtered_tasks.is_empty() {
            None
        } else {
            Some(filtered_tasks)
        }
    }

    pub fn get_in_progress_tasks(&self) -> Option<Vec<&Task>> {
        let filtered_tasks: Vec<&Task> = self
            .tasks
            .iter()
            .filter(|task| task.status() == "in progress")
            .collect();
        if filtered_tasks.is_empty() {
            None
        } else {
            Some(filtered_tasks)
        }
    }

    pub fn get_filtered_tasks(&self, status: &str) -> Option<Vec<&Task>> {
        let filtered_tasks: Vec<&Task> = self
            .tasks
            .iter()
            .filter(|task| task.status() == status)
            .collect();
        if filtered_tasks.is_empty() {
            None
        } else {
            Some(filtered_tasks)
        }
    }

    pub fn update_id(&mut self) {
        for (index, task) in self.tasks.iter_mut().enumerate() {
            task.set_id(index + 1);
        }
    }

    pub fn update_task_by_id(&mut self, id: usize, updated_task: &Task) -> Option<&Task> {
        if let Some(task) = self.tasks.iter_mut().find(|task| task.id() == id) {
            task.update_task(updated_task);
            self.print_task_by_id(id);
        }
        self.update_id();
        println!("test11: {}", self.get_task_by_id(id).unwrap().status());
        self.get_task_by_id(id)
    }

    pub fn update_task_name_by_id(&mut self, id: usize, name: &str) -> Option<&Task> {
        if let Some(task) = self.tasks.iter_mut().find(|task| task.id() == id) {
            task.set_name(name);
            println!("Task id({}) name updated!", id);
            self.print_task_by_id(id);
        }
        self.update_id();
        self.get_task_by_id(id)
    }

    pub fn update_task_description_by_id(&mut self, id: usize, description: &str) -> Option<&Task> {
        if let Some(task) = self.tasks.iter_mut().find(|task| task.id() == id) {
            task.set_description(description);
            println!("Task id({}) description updated!", id);
            self.print_task_by_id(id);
        }
        self.update_id();
        self.get_task_by_id(id)
    }

    pub fn update_task_status_by_id(&mut self, id: usize, status: &str) -> Option<&Task> {
        // status = "done", "cancel",
        if let Some(task) = self.tasks.iter_mut().find(|task| task.id() == id) {
            if status == "done" {
                task.set_status(Status::Completed);
                if task.ended_date().is_none() {
                    task.toggle_ended_date();
                }
            } else if status == "cancel" {
                if !task.ended_date().is_none() {
                    task.toggle_ended_date();
                }
                task.set_status(Status::Cancelled);
            } else if status == "in progress" {
                task.set_status(Status::InProgress);
                if !task.ended_date().is_none() {
                    task.toggle_ended_date();
                }
            } else if status == "todo" {
                task.set_status(Status::Pending);
                if !task.ended_date().is_none() {
                    task.toggle_ended_date();
                }
            }
            println!("test1: {}", task.status());
            println!("Task id({}) status updated!", id);
        }
        self.update_id();
        self.get_task_by_id(id)
    }

    pub fn count_task(&self) -> usize {
        self.tasks.len()
    }

    pub fn clear_task(&mut self) {
        self.tasks.clear();
    }

    pub fn print_tasks(&self) {
        let mut table = create_task_table_header();
        for task in &self.tasks {
            table.add_row(create_task_row(task));
        }
        println!("{}", table);
    }

    pub fn print_task_by_id(&self, id: usize) {
        if let Some(task) = self.get_task_by_id(id) {
            let mut table = create_task_table_header();
            table.add_row(create_task_row(&task));
            println!("{}", table);
        } else {
            println!("Task not found");
        }
    }

    pub fn print_task_by_name(&self, name: &str) {
        if let Some(task) = self.get_task_by_name(name) {
            let mut table = create_task_table_header();
            table.add_row(create_task_row(&task));
            println!("{}", table);
        } else {
            println!("Task not found");
        }
    }

    pub fn print_filtered_tasks(&self, status: &str) {
        if let Some(filtered_tasks) = self.get_filtered_tasks(status) {
            let mut table = create_task_table_header();
            for task in filtered_tasks {
                table.add_row(create_task_row(task));
            }
            println!("{}", table);
        } else {
            println!("No found tasks with status: '{}'", status);
        }
    }

    pub fn print_todo_tasks(&self) {
        if let Some(todo_tasks) = self.get_todo_tasks() {
            let mut table = create_task_table_header();
            for task in todo_tasks {
                table.add_row(create_task_row(task));
            }
            println!("{}", table);
        } else {
            println!("No todo task");
        }
    }

    pub fn print_completed_tasks(&self) {
        if let Some(completed_tasks) = self.get_completed_tasks() {
            let mut table = create_task_table_header();
            for task in completed_tasks {
                table.add_row(create_task_row(task));
            }
            println!("{}", table);
        } else {
            println!("No completed task");
        }
    }

    pub fn print_cancelled_tasks(&self) {
        if let Some(cancelled_tasks) = self.get_cancelled_tasks() {
            let mut table = create_task_table_header();
            for task in cancelled_tasks {
                table.add_row(create_task_row(task));
            }
            println!("{}", table);
        } else {
            println!("No cancelled task");
        }
    }
}
