use std::fs::File;
use std::io::{BufReader, BufWriter, Write};

use serde::Serialize;
use serde_json::from_reader;

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
        let buf = Vec::new();
        let formatter = serde_json::ser::PrettyFormatter::with_indent(b"    ");
        let mut ser = serde_json::Serializer::with_formatter(buf, formatter);
        data.serialize(&mut ser).expect("Failed to serialize");

        let data_str = String::from_utf8(ser.into_inner()).expect("Failed to cast into String");

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

