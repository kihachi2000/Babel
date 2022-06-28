use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TimerData {
    next: bool,
    remain: i64,
}

impl TimerData {
    // continue
    pub fn c(remain: i64) -> Self {
        Self {
            next: false,
            remain,
        }
    }

    // next
    pub fn n(remain: i64) -> Self {
        Self {
            next: true,
            remain,
        }
    }

    pub fn get_next(&self) -> bool {
        self.next
    }
}
