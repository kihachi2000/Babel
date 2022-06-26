#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod commands;
mod managers;
mod types;

use dirs::home_dir;

use managers::{DataManager, TimerManager};

fn main() {
    let mut path = home_dir().unwrap();
    path.push(".babel");
    path.push("data.json");
    let path_string = String::from(path.to_str().unwrap());

    let context = tauri::generate_context!();
    tauri::Builder::default()
        .manage(DataManager::new(path_string))
        .manage(TimerManager::new(1800))
        .invoke_handler(tauri::generate_handler![
            commands::get_data,
            commands::add_brick,
            commands::timer_update,
            commands::timer_stop,
        ])
        .run(context)
        .expect("error while running tauri application");
}
