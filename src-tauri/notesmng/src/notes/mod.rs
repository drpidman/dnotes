use core::panic;
use serde::*;
use std::fs::{self, create_dir, metadata, read_dir, remove_file};
use std::io::prelude::*;
use std::path::PathBuf;
use tauri::{AppHandle, Manager};
use utils::logger::*;
use utils::path_resolver::get_notes_dir;

// ? Estrutura para .md
#[derive(Debug, Serialize, Deserialize)]
pub struct Notes {
    pub title: String,
    pub description: String,
    pub tags: Vec<String>,
}

// ? Estrutura para arquivo
#[derive(Debug, Serialize, Deserialize)]
pub struct NoteFile {
    pub file_name: String,
    pub file_path: PathBuf,
    pub contents: String,
}

pub trait NotesAction {
    // ? Função para inicializar/criar o diretorio de notas
    // ? Criar  diretorios adicionais
    fn init(app: AppHandle);

    // ? Metodos para criar e deletar uma nota.
    // ? Toda a manipulação de arquivos como exclusão/inserção/edição
    fn create(app: AppHandle, note: Notes, content: String) -> Result<Option<NoteFile>, String>;
    fn delete(app: AppHandle, note: &str) -> Result<Option<NoteFile>, String>;

    // ? Metodos para lidar com metadados dos arquivos
    // ? Leitura de modelo de .md como YAML
    fn find_note(app: &AppHandle, note: &str) -> Option<NoteFile>;
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
        log(LogType::Info, "note#create() handled"); 
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
                log(LogType::Error, format!("note#create() error part: file_create. Error:{:}", err)); 
                panic!()
            }
        };

        match file_create.write(content.as_bytes()) {
            Ok(_) => {
                log(LogType::Info, "note#create() success"); 
            }
            Err(err) => {
                log(LogType::Error, format!("note#create() error part: file_create:write. Error:{:}", err)); 
                panic!()
            }
        }

        Ok(Self::find_note(&app, &note.title))
    }

    fn delete(app: AppHandle, note: &str) -> Result<Option<NoteFile>, String> {
        log(LogType::Info, "note#delete() handled"); 
        let note = Self::find_note(&app, &note).unwrap();

        match remove_file(&note.file_path) {
            Ok(_) => Ok(Some(note)),
            Err(err) => {
                log(LogType::Error, format!("note#delete() error part: file_delete. Error:{:}", err)); 
                panic!()
            }
        }
    }

    fn find_note(app: &AppHandle, note: &str) -> Option<NoteFile> {
        log(LogType::Info, "note#find_note() handled"); 

        let notes_dir = get_notes_dir(app.clone().app_handle());
        let mut found_note: Option<NoteFile> = None;

        let notes_files = match read_dir(&notes_dir) {
            Ok(files) => files,

            Err(err) => {
                log(LogType::Error, format!("note#find_note() error part: file_read. Error:{:}", err)); 
                panic!()
            }
        };

        for file_item in notes_files {
            if let Ok(file_type) = file_item {
                if file_type.file_name().into_string().unwrap() == note.to_string() {
                    found_note = Some(NoteFile {
                        file_name: (file_type.file_name().into_string().unwrap()),
                        file_path: (file_type.path()),
                        contents: (fs::read_to_string(file_type.path()).unwrap()),
                    });
                    break;
                }
            }
        }

        found_note
    }

    fn find_all(app: AppHandle) -> Vec<NoteFile> {
        log(LogType::Info, "note#find_all() handled"); 

        let notes_dir = get_notes_dir(app.clone());
        let mut notes: Vec<NoteFile> = vec![];

        let notes_files = match read_dir(&notes_dir) {
            Ok(files) => files,
            Err(err) => {
                log(LogType::Error, format!("note#find_all() error part: file_read. Error:{:}", err)); 
                panic!()
            }
        };

        for file_item in notes_files {
            if let Ok(file) = file_item {
                notes.push(Self::find_note(&app, &file.file_name().into_string().unwrap()).unwrap())
            }
        }
        notes
    }
}
