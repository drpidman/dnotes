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
    fn create(app: AppHandle, note: Notes) -> Result<Option<NoteFile>, String>;
    fn find_note(app: AppHandle, note: &str) -> Option<NoteFile>;
}

impl NotesAction for Notes {
    fn create(app: AppHandle, note: Notes) -> Result<Option<NoteFile>, String> {
        let notes_dir = get_notes_dir(app.app_handle());

        if let Some(find_note) = Notes::find_note(app.clone(), &note.note) {
            return Err(String::from(format!("Note {} already exists", find_note.file_name)));
        }

        let file_name = notes_dir + &note.note + ".md";

        match fs::File::create(&file_name) {
            Ok(_) => {
                Ok(Notes::find_note(app, &note.note))
            },
            Err(err) => {
                println!("Failed to create file. Error:{:?}", err);
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
