use std::vec;

use tauri::Runtime;

use crate::models::notes::notes::{Notes, NotesActions};


#[tauri::command]
async fn create_note<R: Runtime>(app: tauri::AppHandle<R>, window: tauri::Window<R>, note: &str) -> Result<(), String> {
    if note.is_empty() {
        return Err("Note not null".to_string())
    }

    let note_data: Notes = serde_json::from_str(note).unwrap();
    

    Ok(())
}