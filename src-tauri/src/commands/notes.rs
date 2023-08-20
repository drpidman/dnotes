use notesmng::notes::{Notes, NotesAction};
use serde::*;
use tauri::{Manager, Runtime};

use gray_matter::engine::YAML;
use gray_matter::Matter;

#[tauri::command]
pub async fn create_note(
    app: tauri::AppHandle,
    window: tauri::Window,
    note: &str,
) -> Result<(), String> {
    if note.is_empty() {
        return Err("Note not null".to_string());
    }

    let mut matter = Matter::<YAML>::new();
    matter.delimiter = "***".to_owned();

    let result = matter.parse(note);
    let data: Notes = result.data.clone().unwrap().deserialize().unwrap();

    println!("note {:#?}", result);
    println!("note {:#?}", data);

    let created_note = Notes::create(app.app_handle(), data, result.orig);

    println!("{:#?}", created_note);

    Ok(())
}
