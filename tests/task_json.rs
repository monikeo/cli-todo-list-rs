use cli_todo_list::Task::Task;
use cli_todo_list::TaskJson::*;
use std::fs;
use std::path::Path;

fn create_test_file(file_path: &str) {
    let tasks: Vec<Task> = Vec::new();
    write_tasks_to_json(&tasks, file_path).unwrap();
}

fn delete_test_file(file_path: &str) {
    if Path::new(file_path).exists() {
        fs::remove_file(file_path).unwrap();
    }
}

#[test]
fn test_file_exists() {
    let file_path = "test_tasks.json";
    if file_exists(file_path) {
        delete_test_file(file_path);
    }
    create_test_file(file_path);
    assert_eq!(file_exists(file_path), true);

    delete_test_file(file_path);
    assert_eq!(file_exists(file_path), false);
}

#[test]
fn test_add_task_to_json() {
    let file_path = "test_tasks.json";
    if file_exists(file_path) {
        delete_test_file(file_path);
    }
    create_test_file(file_path);

    let tasks = add_task_to_json("Task 1", file_path);
    assert_eq!(tasks.count_task(), 1);
    assert_eq!(tasks.get_task_by_id(1).unwrap().name(), "Task 1");

    let tasks = add_task_to_json("Task 2", file_path);
    assert_eq!(tasks.count_task(), 2);
    assert_eq!(tasks.get_task_by_id(2).unwrap().name(), "Task 2");

    delete_test_file(file_path);
    assert!(file_exists(file_path) == false);
}

#[test]
fn test_read_tasks_from_json() {
    todo!();
}

#[test]
fn test_write_tasks_to_json() {
    todo!();
}

#[test]
fn test_update_tasks_in_json() {
    todo!();
}

#[test]
fn test_delete_task_from_json() {
    todo!();
}
