use chrono::{DateTime, Utc};
use cli_todo_list::Status::Status;
use cli_todo_list::Task::Task;

#[test]
fn test_task_creation() {
    let task = Task::new("Task 1");
    assert_eq!(task.id(), 0);
    assert_eq!(task.name(), "Task 1");
    assert_eq!(task.description(), "");
    assert!(task.created_date().timestamp() <= Utc::now().timestamp());
    assert!(task.updated_date().timestamp() <= Utc::now().timestamp());
    assert_eq!(task.ended_date(), &None);
}

#[test]
fn test_task_modification() {
    let mut task = Task::new("Task 1");
    task.set_id(1);
    task.set_name("My Task");
    task.set_description("My Description");
    task.set_status(Status::Completed);

    assert_eq!(task.id(), 1);
    assert_eq!(task.name(), "My Task");
    assert_eq!(task.description(), "My Description");
    assert_eq!(task.status(), "completed");
    assert!(task.created_date().timestamp() <= Utc::now().timestamp());
    assert!(task.updated_date().timestamp() <= Utc::now().timestamp());
    assert_eq!(task.ended_date(), &None);
}
