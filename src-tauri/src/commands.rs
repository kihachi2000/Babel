use tauri::State;

use crate::managers::{DataManager, TimerManager};
use crate::types::{DailyData, TimerData};

#[tauri::command]
pub fn get_data(state: State<DataManager>) -> Vec<DailyData> {
    println!("get_data");
    state.get_data()
}

#[tauri::command]
pub fn add_brick(state: State<DataManager>) {
    println!("add_brick");
    state.add_brick();
}

#[tauri::command]
pub fn timer_update(data: State<DataManager>, timer: State<TimerManager>) -> TimerData {
    println!("timer_update");
    let remain = timer.update();

    if let TimerData::Next(_) = remain {
        data.add_brick();
    }

    remain
}

#[tauri::command]
pub fn timer_stop(timer: State<TimerManager>) {
    println!("timer_stop");
    timer.stop();
}
