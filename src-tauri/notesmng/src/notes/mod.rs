use core::panic;
use serde::*;
use std::fs::{self, create_dir, metadata, read_dir, File};
use std::io::{prelude::*, self};
use std::path::{PathBuf, Path};
use tauri::api::file;
use tauri::{AppHandle, Manager};
use utils::path_resolver::get_notes_dir;

#[derive(Debug, Serialize, Deserialize)]
pub struct Notes {
    pub title: String,
    pub description: String,
    pub tags: Vec<String>,
}

// ! Only one return
#[derive(Debug, Serialize, Deserialize)]
pub struct NoteFile {
    pub file_name: String,
    pub file_path: PathBuf,
    pub contents: String
}

pub trait NotesAction {
    // ? Initialize/create notes directory
    fn init(app: AppHandle);

    // ? Methods to create and delete notes
    fn create(app: AppHandle, note: Notes, content: String) -> Result<Option<NoteFile>, String>;
    fn delete(app: AppHandle, note: Notes) -> Result<Option<NoteFile>, String>;

    // ? Methods to find and find all notes
    fn find_note(app: AppHandle, note: &str) -> Option<NoteFile>;
    fn find_all(app: AppHandle) -> Vec<NoteFile>;
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
            Ok(file) => file,
            Err(err) => {
                println!("Failed to create file. Error:{:?}", err);
                panic!()
            }
        };

        match file_create.write(content.as_bytes()) {
            Ok(_) => {
                println!("File creation success");
            }
            Err(err) => {
                println!("Failed to create file. Error:{:?}", err);
                panic!()
            }
        }

        Ok(Notes::find_note(app, &note.title))
    }

    fn delete(app: AppHandle, note: Notes) -> Result<Option<NoteFile>, String> {
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
                    found_note = Some(NoteFile {
                        file_name: (file_type.file_name().into_string().unwrap()),
                        file_path: (file_type.path()),
                        contents: (fs::read_to_string(file_type.path()).unwrap())
                    });
                    break;
                }
            }
        }

        found_note
    }

    fn find_all(app: AppHandle) -> Vec<NoteFile> {
        let notes_dir = get_notes_dir(app.clone());

        let mut notes: Vec<NoteFile> = vec![];

        let notes_files = match read_dir(&notes_dir) {
            Ok(files) => files,
            Err(err) => {
                println!("Error ocurred:{:?}", err);
                panic!()
            }
        };

        for file_item in notes_files {
            if let Ok(file) = file_item {
                notes.push(NoteFile {
                    file_name: (file.file_name().into_string().unwrap()),
                    file_path: (file.path()),
                    contents: fs::read_to_string(file.path()).unwrap()
                })
            }
        }
        notes
    }
}
