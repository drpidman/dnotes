pub mod models;

use models::notes::note::{Notes, NotesActions};
use rusqlite::Connection;
use tauri::{AppHandle, App, Manager};
use std::fs::*;

use utils::path_resolver::{get_database, get_data_dir};

pub fn init(app: &App) {
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
            eprintln!("Error ocurred:{:#?}", err);
            panic!("Failed to open connection to database");
        }
    }
}

pub fn close_conn(db: Connection) {
    match db.close() {
        Ok(_) => {
            println!("Connection ...#close");
        },
        Err(_) => {
            panic!("Failed to close connection to db");
        }
    }
}