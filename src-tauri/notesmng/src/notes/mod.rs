use serde::*;
use tauri::api::file;
use std::fs::{self, read_dir, File, metadata, create_dir};
use std::io::prelude::*;
use tauri::{AppHandle, Manager};
use utils::path_resolver::get_notes_dir;

#[derive(Debug, Serialize, Deserialize)]
pub struct Notes {
    pub title: String,
    pub description: String,
    pub tags: Vec<String>
}

#[derive(Debug)]
pub struct NoteFile {
    pub file_name: String,
}

pub trait NotesAction {
    fn init(app: AppHandle);
    fn create(app: AppHandle, note: Notes, content: String) -> Result<Option<NoteFile>, String>;
    fn find_note(app: AppHandle, note: &str) -> Option<NoteFile>;
}

impl NotesAction for Notes {
    fn init(app: AppHandle) {
        let notes_dir = get_notes_dir(app.app_handle());

        if !metadata(notes_dir.clone()).is_ok() {
            create_dir(notes_dir).expect("Failed to create");
        }
    }

    fn create(app: AppHandle, note: Notes, content: String) -> Result<Option<NoteFile>, String> {
        let notes_dir = get_notes_dir(app.app_handle());

        // if let Some(find_note) = Notes::find_note(app.clone(), &note.title) {
        //     return Err(String::from(format!("Note {} already exists", find_note.file_name)));
        // }

        let file_name = if cfg!(windows) {
            notes_dir + "\\" + &note.title + ".md"
        } else {
            notes_dir + "/" + &note.title + ".md"
        };

        let mut file_create = match fs::File::create(&file_name) {
            Ok(file) => {
                file
            },
            Err(err) => {
                println!("Failed to create file. Error:{:?}", err);
                panic!()
            }
        };

        match file_create.write(content.as_bytes()) {
            Ok(_) => {
                println!("File creation success");
            },
            Err(err) => {
                println!("Failed to create file. Error:{:?}", err);
                panic!()
            }
        }

        Ok(Notes::find_note(app, &note.title))
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
                if file_type.file_name().into_string().unwrap() == note.to_string() + ".md" {
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
