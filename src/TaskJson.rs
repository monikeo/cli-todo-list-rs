use crate::{Task::Task, TodoList::TodoList};
use std::fs::{self, File};
use std::io::{self, Read, Write};

pub fn file_exists(file_path: &str) -> bool {
    fs::metadata(file_path).is_ok()
}

pub fn create_file(file_path: &str) -> io::Result<()> {
    if !file_exists(file_path) {
        File::create(file_path)?;
    }
    Ok(())
}

pub fn add_task_to_json(task: &str, file_path: &str) -> TodoList {
    if !file_exists(file_path) {
        return TodoList::new();
    }
    let mut tasks = read_tasks_from_json(file_path).unwrap();
    tasks.add(task);
    let _ = write_tasks_to_json(&tasks.get_all_task(), file_path);
    tasks
}

pub fn read_tasks_from_json(file_path: &str) -> io::Result<TodoList> {
    if !file_exists(file_path) {
        create_file(file_path)?;
        return Ok(TodoList::new());
    }
    let mut file = File::open(file_path)?;
    let mut data = String::new();

    file.read_to_string(&mut data)?;
    if data.len() > 4 {
        let json_data: Vec<Task> = serde_json::from_str(&data)?;
        let mut todo_list = TodoList::new();
        todo_list.import(&json_data);
        Ok(todo_list)
    } else {
        Ok(TodoList::new())
    }
}

pub fn write_tasks_to_json(tasks: &[Task], file_path: &str) -> io::Result<()> {
    let json_data = serde_json::to_string_pretty(tasks)?;
    println!("haha: {}", tasks[0].status());
    let mut file = File::create(file_path)?;
    file.write_all(json_data.as_bytes())?;
    Ok(())
}

pub fn update_task_in_json(updated_task: &Task, file_path: &str) -> io::Result<()> {
    if !file_exists(file_path) {
        return Ok(());
    }
    let mut tasks = read_tasks_from_json(file_path)?;
    if updated_task.id() <= tasks.count_task() {
        println!("test3: {}", updated_task.status());
        tasks.update_task_by_id(updated_task.id(), &updated_task);
        println!("test4: {}", tasks.get_task_by_id(1).unwrap().status());
        write_tasks_to_json(&tasks.get_all_task(), file_path)?;
    }
    Ok(())
}

pub fn delete_task_from_json(id: usize, file_path: &str) -> io::Result<()> {
    if !file_exists(file_path) {
        return Ok(());
    }
    let mut tasks = read_tasks_from_json(file_path)?;
    if id <= tasks.count_task() {
        tasks.delete_by_id(id);
        write_tasks_to_json(&tasks.get_all_task(), file_path)?;
    }
    Ok(())
}
