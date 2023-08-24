use notesmng::notes::{NoteFile, Notes, NotesAction};

use crate::tools::matter::parse;

use serde::*;
use tauri::Manager;

use gray_matter::engine::YAML;
use gray_matter::Matter;

#[derive(Debug, Serialize, Deserialize)]
struct ResponseNote {
    file_data: NoteFile,
    data: Notes,
}

// ! REFEATORAÇÃO NECESSARIA
#[tauri::command]
pub async fn create_note(app: tauri::AppHandle, note: &str) -> Result<String, String> {
    if note.is_empty() {
        return Err("Note not null".to_string());
    }

    let data = parse(note);
    let created_note = Notes::create(&app.app_handle(), data.notes, data.result.orig).unwrap();

    Ok(serde_json::to_string(&created_note).unwrap())
}

// ! REFEATORAÇÃO NECESSARIA
#[tauri::command]
pub async fn find_all_notes(app: tauri::AppHandle) -> Result<String, String> {
    let notes = Notes::find_all(&app);
    let mut note_response: Vec<ResponseNote> = vec![];

    let mut matter = Matter::<YAML>::new();
    matter.delimiter = "***".to_owned();

    for note in notes {
        let data: Notes = matter
            .parse(&note.contents)
            .data
            .unwrap()
            .deserialize()
            .unwrap();

        note_response.push(ResponseNote {
            file_data: note,
            data,
        })
    }

    Ok(serde_json::to_string(&note_response).unwrap())
}

#[tauri::command]
pub async fn find_note(app: tauri::AppHandle, note: &str) -> Result<String, String> {
    let found_note = Notes::find_note(&app, note).unwrap();

    let mut note_response: Vec<ResponseNote> = vec![];
    let mut matter = Matter::<YAML>::new();
    matter.delimiter = "***".to_owned();

    let data: Notes = matter
        .parse(&found_note.contents)
        .data
        .unwrap()
        .deserialize()
        .unwrap();

    note_response.push(ResponseNote {
        file_data: found_note,
        data,
    });

    Ok(serde_json::to_string(&note_response).unwrap())
}

#[tauri::command]
pub async fn delete_note(app: tauri::AppHandle, note: &str) -> Result<String, String> {
    if note.is_empty() {
        return Err("Note not null".to_string());
    }

    let deleted_note = Notes::delete(&app, note).unwrap();

    Ok(serde_json::to_string(&deleted_note).unwrap())
}
