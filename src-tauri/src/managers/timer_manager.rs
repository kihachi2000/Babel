use std::sync::Mutex;

use chrono::{DateTime, Duration, Local};

use crate::types::TimerData;

pub struct TimerManager {
    interval: i64,
    remain: Mutex<Duration>,
    last_update: Mutex<Option<DateTime<Local>>>
}

impl TimerManager {
    pub fn new(interval: i64) -> Self {
        Self {
            interval,
            remain: Mutex::new(Duration::seconds(interval)),
            last_update: Default::default(),
        }
    }

    pub fn sync(&self) -> i64 {
        self.remain.lock().unwrap().num_seconds()
    }

    pub fn update(&self) -> TimerData {
        let mut remain = self.remain.lock().unwrap();
        let mut update = self.last_update.lock().unwrap();

        let prev_remain = *remain;
        let prev_update = *update;
        let now = Local::now();

        let duration = match prev_update {
            Some(prev) => now - prev,
            None => Duration::zero(),
        };

        let new_remain = prev_remain - duration;
        let zero = Duration::zero();

        *update = Some(now);
        if zero < new_remain {
            *remain = new_remain;
            TimerData::c(new_remain.num_seconds())
        } else {
            *remain = Duration::seconds(self.interval);
            TimerData::n(self.interval)
        }
    }

    pub fn stop(&self) {
        let mut last_update = self.last_update.lock().unwrap();
        *last_update = None;
    }
}
