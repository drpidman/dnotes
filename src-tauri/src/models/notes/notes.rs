use chrono::*;
use serde::*;
use tauri::AppHandle;

use crate::database::{get_conn, close_conn};

#[derive(Debug, Serialize, Deserialize)]
/// ### Note - Struct for note
/// - Create a new notes object, parse to JSON for ready data
pub struct Notes {
    pub note: String,
    pub description: String,
    pub accent_color: i32,
    pub content: String,
    pub created_at: std::time::Duration,
}

pub trait NotesActions {
    fn create() -> String;
    fn find() -> String;
    fn delete() -> String;

    fn init(app: AppHandle);
}

impl NotesActions for Notes {
    fn init(app: AppHandle) {
        println!("notes#init()");
        let db = get_conn(&app, crate::utils::path_resolver::Databases::Notes);

        match db.execute(
            "CREATE TABLE IF NOT EXISTS notes(id INTEGER PRIMARY KEY,
             note TEXT,
             description TEXT,
             accent_color INTEGER,
             content TEXT,
             created_at Date)",
            (),
        ) {
            Ok(_) => {
                println!("Table notes#init created");
            }
            Err(err) => {
                println!("Error to execute notes#init: {:#?}", err);
                panic!("Error to execute db statement")
            }
        };

        close_conn(db);
    }

    fn create() -> String {
        String::from("")
    }

    fn find() -> String {
        String::from("")
    }

    fn delete() -> String {
        String::from("")
    }
}
