// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use notesmng::notes::{Notes, NotesAction};
use tauri::Manager;

mod commands;

fn main() {
    let app = tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::notes::create_note,
            commands::notes::find_all_notes
            ])
        .setup(|app| {
            Notes::init(app.app_handle());

            Ok(())
        });

    app.run(tauri::generate_context!())
        .expect("error while running tauri application");
}
