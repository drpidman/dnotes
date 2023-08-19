use serde::*;
use std::fs::{self, read_dir};
use tauri::{api::file, AppHandle, Manager};
use utils::{path_resolver::get_notes_dir, *};

#[derive(Debug, Serialize, Deserialize)]
pub struct Notes {
    pub note: String,
    pub description: String,
    pub tags: Vec<String>,
    pub accent_color: i32,
}

pub struct NoteFile {
    pub file_name: String,
}

pub trait NotesAction {
    fn create(app: AppHandle, note: Notes) -> Result<String, String>;
    fn find_note(app: AppHandle, note: &str) -> Option<NoteFile>;
}

impl NotesAction for Notes {
    fn create(app: AppHandle, note: Notes) -> Result<String, String> {
        let notes_dir = get_notes_dir(app.app_handle());

        if !Notes::find_note(app.app_handle(), &note.note).is_none() {
            return Err("Note already exists".to_owned());
        };

        let file_name = notes_dir + &note.note + ".md";

        match fs::File::create(&file_name) {
            Ok(_) => {
                Ok(String::from("file created success"))
            },
            Err(err) => {
                println!("Failed to create file because error:{:?}", err);
                panic!()
            }
        }

    }

    fn find_note(app: AppHandle, note: &str) -> Option<NoteFile> {
        let notes_dir = get_notes_dir(app.app_handle());
        let mut found_note: Option<NoteFile> = None;

        let notes_files = match read_dir(&notes_dir) {
            Ok(files) => files,

            Err(err) => {
                println!("Error ocurred:{:?}", err);
                panic!()
            }
        };

        for file_item in notes_files {
            if let Ok(file_type) = file_item {
                if file_type.file_name() == note {
                    found_note = Some(NoteFile
                        { file_name: (file_type.file_name().into_string().unwrap()) }
                    );
                    break;
                }
            }
        };

        found_note
    }
}
