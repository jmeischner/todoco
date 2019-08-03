use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Eq)]
pub struct Meta {
    created_at: DateTime<Local>,
    last_edited: DateTime<Local>,
    is_active: bool
}

impl Meta {
    pub fn new() -> Self {
        Meta {
            created_at: Local::now(),
            last_edited: Local::now(),
            is_active: true
        }
    }

    pub fn get_created_at(&self) -> DateTime<Local> {
        self.created_at
    }

    pub fn set_created_at(&mut self, created_at: DateTime<Local>) {
        self.created_at = created_at;
    }

    pub fn get_last_edited(&self) -> DateTime<Local> {
        self.last_edited
    }

    pub fn set_last_edited(&mut self, date: DateTime<Local>) {
        self.last_edited = date;
    }

    pub fn get_is_active(&self) -> bool {
        self.is_active
    }

    pub fn set_is_active(&mut self, active: bool) {
        self.is_active = active;
    }

}

impl PartialEq for Meta {
    fn eq(&self, other: &Meta) -> bool {
        self.created_at == other.created_at &&
        self.last_edited == other.last_edited &&
        self.is_active == other.is_active
    }
}