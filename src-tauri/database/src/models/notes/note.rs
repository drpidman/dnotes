use serde::*;
use tauri::{AppHandle, Manager};

use utils::*;

use crate::{get_conn, close_conn};

/// ### Note - Struct for note
/// - Create a new notes object, parse to JSON for ready data
#[derive(Debug, Serialize, Deserialize)]
pub struct Notes {
    pub note: String,
    pub description: String,
    pub content: String,
    pub tags: Vec<String>,
    pub accent_color: i32,
    pub created_at: std::time::Duration,
}

pub trait NotesActions {
    fn create(app: AppHandle, note: Notes) -> String;

    fn find() -> String;
    fn delete() -> String;

    fn init(app: AppHandle);
}

impl NotesActions for Notes {
    fn init(app: AppHandle) {
        
        println!("notes#init()");
        let db = get_conn(&app, path_resolver::Databases::Notes);

        match db.execute(
            "CREATE TABLE IF NOT EXISTS notes(note TEXT PRIMARY KEY,
             description TEXT,
             content TEXT,
             tags TEXT,
             accent_color INTEGER,
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

    fn create(app: AppHandle, note: Notes) -> String {
        let db = get_conn(&app, path_resolver::Databases::Notes);

        let tags = serde_json::to_string(&note.tags).unwrap().to_string();
    
        match db.execute("INSERT INTO notes(
            note,
            description,
            content,
            tags,
            accent_color,
            created_at
        ) VALUES(:note, :description, :content, :tags, :accent_color, :created_at)",
            &[
                (":note", &note.note),
                (":description", &note.description),
                (":content", &note.content),
                (":tags", &tags),
                (":accent_color", &note.accent_color.to_string()),
                (":created_at", &note.created_at.as_millis().to_string())
            ]
        ) {
            Ok(_) => {
                println!("Statement notes#create() Success!");
            },
            Err(err) => {
                println!("A Error as ocurred: {:#?}", err);
                panic!()
            }
        };

        close_conn(db);

        String::from("")
    }

    fn find() -> String {
        String::from("")
    }

    fn delete() -> String {
        String::from("")
    }
}