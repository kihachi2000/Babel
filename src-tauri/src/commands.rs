use tauri::State;

use crate::managers::{DataManager, TimerManager};
use crate::types::{BrickData, TimerData};

#[tauri::command]
pub fn get_data(state: State<DataManager>) -> Vec<Vec<BrickData>> {
    println!("get_data");

    let source = state.get_data();
    let mut expanded = Vec::<BrickData>::new();
    for (index, data) in source.into_iter().enumerate() {
        for _ in 0..data.get_bricks() {
            expanded.push(BrickData::new(data.get_date().clone(), index));
        }
    }

    let steps_count = (expanded.len() + (5 - 1)) / 5;

    let mut i = 0;
    let mut table = Vec::new();

    for _ in 0..steps_count {
        let mut step = Vec::new();

        for _ in 0..5 {
            if let Some(data) = expanded.get(i) {
                step.push(data.clone());
                i += 1;
            }
        }

        table.push(step);
    }

    table.into_iter()
         .rev()
         .collect()
}

#[tauri::command]
pub fn add_brick(state: State<DataManager>) {
    println!("add_brick");
    state.add_brick();
}

#[tauri::command]
pub fn timer_sync(timer: State<TimerManager>) -> i64 {
    println!("timer_sync");
    timer.sync()
}

#[tauri::command]
pub fn timer_update(data: State<DataManager>, timer: State<TimerManager>) -> TimerData {
    println!("timer_update");
    let remain = timer.update();

    if remain.get_next() {
        data.add_brick();
    }

    remain
}

#[tauri::command]
pub fn timer_stop(timer: State<TimerManager>) {
    println!("timer_stop");
    timer.stop();
}
