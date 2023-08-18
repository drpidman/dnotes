// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod database;
mod utils;
mod models;


fn main() {
  
  let app = tauri::Builder::default()
  .invoke_handler(tauri::generate_handler![])
  .setup(|app| {
    database::init(app);
    Ok(())
  });

  app.run(tauri::generate_context!())
  .expect("error while running tauri application");
  
}
