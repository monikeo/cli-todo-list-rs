use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub enum Status {
    Completed,
    InProgress,
    Cancelled,
    Pending,
}

impl Status {
    pub fn new() -> Self {
        Self::Pending
    }

    pub fn info(&self) -> &str {
        match self {
            Status::Completed => "completed",
            Status::InProgress => "in progress",
            Status::Cancelled => "cancelled",
            Status::Pending => "pending",
        }
    }

    pub fn update(&mut self, status: Status) {
        *self = status;
    }
}
