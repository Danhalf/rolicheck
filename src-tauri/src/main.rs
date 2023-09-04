// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
extern crate rusqlite;

use rusqlite::{Connection, Result, params};

#[tauri::command]
fn my_custom_command() -> String {
  "Hello from Rust!".into()
}

fn main() -> Result<()> {
    let conn = Connection::open("cars.db")?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS cars (
            id INTEGER PRIMARY KEY,
            car_name TEXT NOT NULL,
            car_number TEXT NOT NULL UNIQUE, 
            fuel_consumption_main NUMBER NOT NULL,
            fuel_consumption_other NUMBER
         )",
        params![],
    )?;

      // Розпочніть транзакцію
      conn.execute("BEGIN", params![])?;

      if let Err(err) = conn.execute(
          "INSERT INTO cars (car_name, car_number, fuel_consumption_main, fuel_consumption_other)
           VALUES (?, ?, ?, ?)",
          params![
              "Toyota Camry",
              "ABC123",
              8.5,
              6.0, // Інші дані, які ви хочете вставити
          ],
      ) {
          eprintln!("Помилка під час вставки даних: {:?}", err);
          // Відкатити транзакцію у разі помилки
          conn.execute("ROLLBACK", params![])?;
      } else {
          // Закінчити транзакцію, якщо все пройшло успішно
          conn.execute("COMMIT", params![])?;
      }

    tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![my_custom_command])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");

    Ok(())
}


// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
// #[tauri::command]
// fn greet(name: &str) -> String {
//     format!("Hello, {}! You've been greeted from Rust!", name)
// }