use chrono::{DateTime, Local};
use cli_todo_list::DateTimeInfo::DateTimeInfo;

#[test]
fn test_datetime_info_creation() {
    let datetime_info = DateTimeInfo::new();

    let now = Local::now();

    assert!(datetime_info.created_date() <= &now);
    assert!(datetime_info.updated_date() <= &now);
    assert_eq!(datetime_info.ended_date(), &None);
}

#[test]
fn test_datetime_info_update() {
    let mut datetime_info = DateTimeInfo::new();
    let now = Local::now();
    datetime_info.set_updated_date();
    assert!(datetime_info.updated_date() > &now);
    assert!(datetime_info.updated_date() > datetime_info.created_date());

    datetime_info.toggle_ended_date();
    let now = Local::now();
    assert!(datetime_info.ended_date().unwrap() <= now);
    datetime_info.toggle_ended_date();
    assert!(datetime_info.ended_date() == &None);

    let now = Local::now();
    datetime_info.toggle_ended_date();
    assert!(datetime_info.ended_date().unwrap() >= now);
}
