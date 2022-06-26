use chrono::{Local, NaiveDate};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DailyData {
    date: NaiveDate,
    blocks: usize,
}

impl DailyData {
    pub fn new() -> Self {
        Self {
            date: Local::today().naive_local(),
            blocks: 1,
        }
    }

    pub fn get_date(&self) -> &NaiveDate {
        &self.date
    }

    pub fn add_brick(&mut self) {
        self.blocks += 1;
    }
}
