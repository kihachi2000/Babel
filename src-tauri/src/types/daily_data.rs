use chrono::{Local, NaiveDate};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DailyData {
    date: NaiveDate,
    bricks: usize,
}

impl DailyData {
    pub fn new() -> Self {
        Self {
            date: Local::today().naive_local(),
            bricks: 1,
        }
    }

    pub fn get_date(&self) -> &NaiveDate {
        &self.date
    }

    pub fn get_bricks(&self) -> usize {
        self.bricks
    }

    pub fn add_brick(&mut self) {
        self.bricks += 1;
    }
}
