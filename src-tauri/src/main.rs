// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
extern crate rusqlite;

use rusqlite::{Connection, Result, params};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() -> Result<()> {
    tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![greet])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");


    let conn = Connection::open("cars.db")?;

    conn.execute(
        "create table if not exists cats (
             id integer primary key,
             car_name text not null,
             car_number text not null
             odometer number

         )",
        params![],
    )?;

    Ok(())
}

// tauri::Builder::default()
//     .invoke_handler(tauri::generate_handler![greet])
//     .run(tauri::generate_context!())
//     .expect("error while running tauri application");