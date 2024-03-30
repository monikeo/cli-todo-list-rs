use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct DateTimeInfo {
    created_date: DateTime<Local>,
    updated_date: DateTime<Local>,
    ended_date: Option<DateTime<Local>>,
}

impl DateTimeInfo {
    pub fn new() -> Self {
        Self {
            created_date: Local::now(),
            updated_date: Local::now(),
            ended_date: None,
        }
    }

    pub fn created_date(&self) -> &DateTime<Local> {
        &self.created_date
    }

    pub fn updated_date(&self) -> &DateTime<Local> {
        &self.updated_date
    }

    pub fn ended_date(&self) -> &Option<DateTime<Local>> {
        &self.ended_date
    }

    pub fn set_created_date(&mut self, date: DateTime<Local>) {
        self.created_date = date;
    }

    pub fn set_updated_date(&mut self) {
        self.updated_date = Local::now();
    }

    pub fn set_ended_date(&mut self, ended_date: Option<DateTime<Local>>) {
        self.ended_date = ended_date;
    }

    pub fn toggle_ended_date(&mut self) {
        if self.ended_date.is_none() {
            self.ended_date = Some(Local::now());
        } else {
            self.ended_date = None;
        }
    }
}
