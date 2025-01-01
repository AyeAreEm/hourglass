#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // don't remove
use sysinfo::System;
use tauri::Manager;
use serde::{Deserialize, Serialize};
use std::sync::Mutex;
use std::{thread, time::Duration};

#[derive(Deserialize, Serialize, Debug)]
struct Stats {
    total: f64,
    longest_sesh: f64,
    today: f64,
}

#[derive(Deserialize, Serialize, Debug)]
struct Program {
    name: String,
    stats: Stats,
}

impl Clone for Program {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            stats: Stats {
                total: self.stats.total,
                longest_sesh: self.stats.longest_sesh,
                today: self.stats.today,
            }
        }
    }
}

#[tauri::command]
fn find_program(process_name: Program) -> bool {
    let sys = System::new_all();
    let all_processes = sys.processes();
    let lower = process_name.name.to_lowercase();
    
    for (_, process) in all_processes {
        let process_lower = process.name().to_lowercase();

        if process_lower.strip_suffix(".exe").unwrap_or(&process_lower) == lower {
            return true
        }
    }

    return false
}

#[tauri::command]
fn insert_new_program(state: tauri::State<Mutex<Vec<Program>>>, new_program: Program) {
    match state.lock() {
        Ok(mut all_program) => {
            all_program.push(new_program);
        },
        Err(e) => eprintln!("{e}"),
    }
}

#[tauri::command]
fn compare_timestamps(today: u64) -> bool {
    let timestamp = match std::fs::read_to_string("./timestamp.txt") {
        Ok(content) => content,
        Err(_) => {
            std::fs::write("./timestamp.txt", format!("{today}")).unwrap();
            return true
        }
    };

    if today != timestamp.parse::<u64>().unwrap() {
        std::fs::write("./timestamp.txt", format!("{today}")).unwrap();
        return false;
    }

    true
}

fn main() {
    let all_games: Mutex<Vec<Program>> = Mutex::new(Vec::new());

    tauri::Builder::default()
        .manage(all_games)
        .setup(|app| {
            let app_handle = app.handle().clone();

            tauri::async_runtime::spawn(async move {
                let state = app_handle.state::<Mutex<Vec<Program>>>();
                loop {
                    match state.lock() {
                        Ok(mut all_programs) => {
                            println!("{:?}", all_programs);
                            for program in &mut *all_programs {
                                if find_program(program.clone()) {
                                    let minute_as_hour = 1.0 / 60.0;
                                    program.stats.today += minute_as_hour;
                                    program.stats.total += minute_as_hour;
                                    println!("{}", program.stats.total);
                                }
                            }
                        },
                        Err(e) => eprintln!("{e}"),
                    }
                    thread::sleep(Duration::from_secs(2));
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![find_program, insert_new_program, compare_timestamps])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    println!("hello world");
}
