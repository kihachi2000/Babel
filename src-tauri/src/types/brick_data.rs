use chrono::NaiveDate;
use serde::{Serialize, Deserialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BrickData {
    date: NaiveDate,
    color_num: usize,
}

impl BrickData {
    pub fn new(date: NaiveDate, color_num: usize) -> Self {
        Self {
            date,
            color_num,
        }
    }
}
