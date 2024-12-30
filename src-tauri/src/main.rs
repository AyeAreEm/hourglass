#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // don't remove
use sysinfo::System;

#[tauri::command]
fn find_process(process_name: &str) -> bool {
    let sys = System::new_all();
    let mut found = false;
    for proc in sys.processes_by_name(process_name) {
        println!("proc name: {}", proc.name());
        found = true;
        // break;
    }

    return found
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![find_process])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
