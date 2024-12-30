#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // don't remove
use sysinfo::System;
use tauri::Manager;
use std::sync::Mutex;
use std::{thread, time::Duration};

#[tauri::command]
fn find_process(process_name: &str) -> bool {
    let sys = System::new_all();
    let mut found = false;
    for proc in sys.processes_by_name(process_name) {
        found = true;
        println!("{}", proc.name())
        // break;
    }

    return found
}

#[tauri::command]
fn insert_new_game(state: tauri::State<Mutex<Vec<String>>>, new_game: &str) {
    match state.lock() {
        Ok(mut all_games) => {
            all_games.push(new_game.to_string());
        },
        Err(e) => eprintln!("{e}"),
    }
}

fn main() {
    let all_games: Mutex<Vec<String>> = Mutex::new(Vec::new());

    tauri::Builder::default()
        .manage(all_games)
        .setup(|app| {
            let app_handle = app.handle().clone();

            tauri::async_runtime::spawn(async move {
                let state = app_handle.state::<Mutex<Vec<String>>>();
                loop {
                    match state.lock() {
                        Ok(all_games) => {
                            for game in &*all_games {
                                find_process(&game);
                            }
                        },
                        Err(e) => eprintln!("{e}"),
                    }
                    thread::sleep(Duration::from_secs(2));
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![find_process, insert_new_game])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    println!("hello world");
}
