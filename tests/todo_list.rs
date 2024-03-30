use chrono::{DateTime, Utc};
use cli_todo_list::Status::Status;
use cli_todo_list::Task::Task;
use cli_todo_list::TodoList::TodoList;

#[test]
fn test_todolist_creation() {
    let todo_list = TodoList::new();
    assert_eq!(todo_list.count_task(), 0);
}

fn test_todolist_add_task() {
    let mut todo_list = TodoList::new();
    todo_list.add("Task 1");
    todo_list.add("Task 2");
    todo_list.add("Task 3");

    let now = Utc::now().timestamp();

    for i in 1..=3 {
        assert_eq!(todo_list.get_task_by_id(i - 1).unwrap().id(), i);
        assert_eq!(
            todo_list.get_task_by_id(i - 1).unwrap().name(),
            format!("Task {}", i).as_str()
        );
        assert_eq!(todo_list.get_task_by_id(i - 1).unwrap().description(), "");
        assert!(
            todo_list
                .get_task_by_id(i - 1)
                .unwrap()
                .created_date()
                .timestamp()
                <= now
        );
        assert!(
            todo_list
                .get_task_by_id(i - 1)
                .unwrap()
                .updated_date()
                .timestamp()
                <= now
        );
    }
}

#[test]
fn test_todolist_delete_and_clear() {
    let mut todolist = TodoList::new();
    todolist.add("Task 1");
    todolist.add("Task 2");
    todolist.add("Task 3");
    let mut todolist1 = todolist.clone();

    todolist.delete_by_id(2);
    assert_eq!(
        todolist.get_task_by_id(1).unwrap().name(),
        todolist1.get_task_by_id(1).unwrap().name()
    );
    assert!(match todolist.get_task_by_id(2) {
        Some(task) => false,
        None => true,
    });
    assert_eq!(todolist.count_task(), 2);

    todolist1.clear_task();
    assert_eq!(todolist1.count_task(), 0);
}
#[test]
fn test_todolist_modification() {
    let mut todolist = TodoList::new();
    todolist.add("Task 1");
    todolist.add("Task 2");
    todolist.add("Task 3");

    todolist.update_task_name_by_id(1, "Task One");
    assert_eq!(todolist.get_task_by_id(1).expect("Not Found").id(), 1);
    assert_eq!(
        todolist.get_task_by_id(1).expect("Not Found").name(),
        "Task One"
    );

    todolist.update_task_description_by_id(1, "This is my first task");
    assert_eq!(
        todolist
            .get_task_by_name("Task One")
            .expect("Not Found")
            .description(),
        "This is my first task"
    );

    todolist.update_task_status_by_id(2, "done");
    assert_eq!(
        todolist
            .get_task_by_name("Task 2")
            .expect("Not Found")
            .status(),
        "completed"
    );
}
