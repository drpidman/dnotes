pub mod models;

use casual_logger::Log;
use chrono::format;
use models::notes::note::{Notes, NotesActions};
use rusqlite::Connection;
use tauri::{AppHandle, App, Manager};
use std::fs::*;

use utils::path_resolver::{get_database, get_data_dir};

pub fn init(app: &App) {
    Log::set_level(casual_logger::Level::Notice);

    let data_dir = get_data_dir(app.app_handle());

    if !metadata(data_dir.clone()).is_ok() {
        create_dir(data_dir.clone()).expect("Failed to create");
    }

    Notes::init(app.app_handle());
}

pub fn get_conn(app: &AppHandle, database: utils::path_resolver::Databases) -> Connection {
    match Connection::open(get_database(app.app_handle(), database)) {
        Ok(db) => db,
        Err(err) => {
            Log::error(&format!("connection:open()->error:{}", err));
            panic!("Failed to open connection to database");
        }
    }
}

pub fn close_conn(db: Connection) {
    match db.close() {
        Ok(_) => {
            Log::info("connection:close()->ok")
        },
        Err(_) => {
            Log::error("connection:close()->error");
            panic!();
        }
    }
}