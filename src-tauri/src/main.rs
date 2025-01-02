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
    todays_sesh: f64,
    today: f64, // total play time today
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
                todays_sesh: self.stats.todays_sesh,
                today: self.stats.today,
            }
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
struct LongestSeshPayload {
    name: String,
    sesh: f64,
}

impl Clone for LongestSeshPayload {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            sesh: self.sesh,
        }
    }
}

fn find_program(process_name: &Program) -> bool {
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
            println!("inserting new: {}", new_program.name);
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
            let window = app.get_window("main").unwrap();
            let app_handle = app.handle().clone();

            tauri::async_runtime::spawn(async move {
                let mut updated_programs = Vec::new();
                let state = app_handle.state::<Mutex<Vec<Program>>>();

                loop {
                    match state.lock() {
                        Ok(mut all_programs) => {
                            for program in &mut *all_programs {
                                if find_program(&program) {
                                    let minute_as_hour = 1.0 / 60.0;
                                    program.stats.today += minute_as_hour;
                                    program.stats.total += minute_as_hour;
                                    program.stats.todays_sesh += minute_as_hour;

                                    // TODO: update program.stats.longest_sesh
                                    updated_programs.push(program.clone());
                                } else if program.stats.todays_sesh != 0.0 {
                                    match window.emit("update-longest-sesh", LongestSeshPayload{ name: program.name.clone(), sesh: program.stats.todays_sesh }) {
                                        Ok(_) => (),
                                        Err(e) => eprintln!("error: {e}"),
                                    }
                                    program.stats.todays_sesh = 0.0;
                                }
                            }
                        },
                        Err(e) => eprintln!("{e}"),
                    }

                    match window.emit("update-program-data", updated_programs.clone()) {
                        Ok(_) => (),
                        Err(e) => eprintln!("error: {e}"),
                    }
                    thread::sleep(Duration::from_secs(2));
                    updated_programs.clear();
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![insert_new_program, compare_timestamps])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    println!("hello world");
}
