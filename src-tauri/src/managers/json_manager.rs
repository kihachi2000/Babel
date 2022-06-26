use std::fs::File;
use std::io::{BufReader, BufWriter, Write};

use serde_json::{to_string, from_reader};

use crate::types::daily_data::DailyData;

pub struct JsonManager {
    file_path: String,
}

impl JsonManager {
    pub fn new(file_path: impl Into<String>) -> Self {
        Self {
            file_path: file_path.into(),
        }
    }

    pub fn write_file(&self, data: &Vec<DailyData>) {
        let data_str = to_string(data).expect("Failed to serialize");

        let file = File::create(&self.file_path).expect("Failed to open file");

        let mut writer = BufWriter::new(file);
        writer.write(data_str.as_bytes()).expect("Failed to write");
    }

    pub fn read_file(&self) -> Vec<DailyData> {
        if let Ok(file) = File::open(&self.file_path) {
            let reader = BufReader::new(file);
            from_reader(reader).expect("Failed to deserialize")
        } else {
            Vec::new()
        }
    }
}

