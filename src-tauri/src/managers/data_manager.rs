use std::sync::Mutex;

use chrono::Local;

use crate::types::daily_data::DailyData;
use crate::managers::json_manager::JsonManager;

pub struct DataManager {
    data: Mutex<Vec<DailyData>>,
    json_manager: JsonManager,
}

impl DataManager {
    pub fn new(file_path: impl Into<String>) -> Self {
        let json_manager = JsonManager::new(file_path);
        let data = Mutex::new(json_manager.read_file());

        Self {data, json_manager}
    }

    pub fn add_brick(&self) {
        let mut data = self.data.lock().unwrap();

        match data.iter_mut().last() {
            Some(last_data) => {
                let today = Local::today().naive_local();
                let last_date = last_data.get_date();

                if *last_date == today {
                    last_data.add_brick();
                } else {
                    data.push(DailyData::new());
                }
            },

            None => {
                data.push(DailyData::new());
            },
        }

        self.json_manager.write_file(&data);
    }

    pub fn get_data(&self) -> Vec<DailyData> {
        let data = self.data.lock().unwrap();
        data.clone()
    }
}
