use crate::DateTimeInfo::DateTimeInfo;
use crate::Status::Status;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct Task {
    id: usize,
    name: String,
    description: String,
    status: Status,
    datetime_info: DateTimeInfo,
}

impl Task {
    pub fn new(name: &str) -> Self {
        let mut task = Task::default();
        task.set_name(name);
        task
    }

    pub fn default() -> Self {
        Self {
            id: 0,
            name: String::new(),
            description: String::new(),
            status: Status::Pending,
            datetime_info: DateTimeInfo::new(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn set_id(&mut self, id: usize) {
        self.id = id;
    }

    pub fn set_name(&mut self, name: &str) {
        self.date_update();
        self.name = name.to_string();
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn set_description(&mut self, description: &str) {
        self.date_update();
        self.description = description.to_string();
    }

    pub fn status(&self) -> &str {
        &self.status.info()
    }

    pub fn set_status(&mut self, status: Status) {
        self.date_update();
        self.status.update(status);
    }

    pub fn set_status_by_text(&mut self, status: &str) {
        if status == "completed" {
            self.set_status(Status::Completed);
        } else if status == "in progress" {
            self.set_status(Status::InProgress);
        } else if status == "cancelled" {
            self.set_status(Status::Cancelled);
        }
        /*
        match status {
            "completed" => {
                self.set_status(Status::Completed);
            }
            "in progress" => {}
            "cancelled" => {}
            "panding" => {}
        }
        */
    }

    pub fn update_task(&mut self, task: &Task) {
        self.set_id(task.id());
        self.set_name(task.name());
        self.set_description(task.description());
        self.set_created_date(task.created_date().clone());
        self.date_update();
        self.set_status_by_text(task.status());
        match self.status() {
            "completed" => {
                if self.ended_date().is_none() {
                    self.toggle_ended_date();
                }
            }
            _ => {
                if !self.ended_date().is_none() {
                    self.toggle_ended_date();
                }
            }
        }
    }

    pub fn created_date(&self) -> &DateTime<Local> {
        &self.datetime_info.created_date()
    }

    fn set_created_date(&mut self, date: DateTime<Local>) {
        self.datetime_info.set_created_date(date);
    }

    pub fn updated_date(&self) -> &DateTime<Local> {
        &self.datetime_info.updated_date()
    }

    pub fn ended_date(&self) -> &Option<DateTime<Local>> {
        &self.datetime_info.ended_date()
    }

    pub fn toggle_ended_date(&mut self) {
        self.datetime_info.toggle_ended_date();
    }

    pub fn set_ended_date(&mut self, ended_date: Option<DateTime<Local>>) {
        self.datetime_info.set_ended_date(ended_date);
    }

    fn date_update(&mut self) {
        self.datetime_info.set_updated_date();
    }
}
