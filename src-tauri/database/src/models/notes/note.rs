use casual_logger::Log;
use chrono::Duration;
use serde::*;
use tauri::AppHandle;
use utils::*;

use crate::{close_conn, get_conn};

/// ### Note - Struct for note
/// - Create a new notes object, parse to JSON for ready data
#[derive(Debug, Serialize, Deserialize)]
pub struct Notes {
    pub note: String,
    pub description: String,
    pub content: String,
    pub tags: Vec<String>,
    pub accent_color: i32,
    pub created_at: i32,
}

pub trait NotesActions {
    fn init(app: AppHandle);

    fn create(app: AppHandle, note: Notes) -> String;
    fn find(app: AppHandle, note: &str) -> String;
    fn delete(note: &str) -> String;
}

impl NotesActions for Notes {
    fn init(app: AppHandle) {
        Log::info("notes:init()->init");

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
                Log::info("notes:init()->init->ok");
            }
            Err(err) => {
                Log::error(&format!("notes:init()->init->error:{}", err));
                panic!("Error to execute sql string")
            }
        };

        close_conn(db);
    }

    fn create(app: AppHandle, note: Notes) -> String {
        let db = get_conn(&app, path_resolver::Databases::Notes);

        let tags = serde_json::to_string(&note.tags).unwrap().to_string();

        match db.execute(
            "INSERT INTO notes(
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
                (":created_at", &note.created_at.to_string()),
            ],
        ) {
            Ok(_) => {
                Log::info("notes:init()->create->ok");
            }
            Err(err) => {
                Log::error(&format!("notes:init()->create->error:{}", err));
                panic!()
            }
        };

        close_conn(db);

        String::from("")
    }

    fn find(app: AppHandle, note: &str) -> String {
        let db = get_conn(&app, path_resolver::Databases::Notes);

        let mut notes: Vec<Notes> = vec![];

        {
            let mut statement = match db.prepare("SELECT * FROM notes WHERE note = :note ") {
                Ok(stmt) => stmt,
                Err(err) => {
                    Log::error(&format!("notes:init()->find->error:{}", err));
                    panic!()
                }
            };

            let note_rows = match statement.query_map(&[(":note", &note)], |row| {
                Ok(Notes {
                    note: row.get(0)?,
                    description: row.get(1)?,
                    content: row.get(2)?,
                    tags: serde_json::from_str(&row.get_unwrap::<usize, String>(3).to_string())
                        .unwrap(),
                    accent_color: row.get_unwrap::<usize, i32>(4),
                    created_at: row.get(5)?,
                })
            }) {
                Ok(res) => res,
                Err(err) => {
                    Log::error(&format!("notes:init()->find->error:{}", err));
                    panic!()
                }
            };

            for note_item in note_rows {
                notes.push(note_item.unwrap())
            }
        }

        close_conn(db);
        String::from(serde_json::to_string(&notes.get(0)).unwrap())
    }

    fn delete(note: &str) -> String {
        String::from("")
    }
}
