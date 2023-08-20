use notesmng::notes::{NoteFile, Notes, NotesAction};
use serde::*;
use tauri::{Manager, Runtime};

use gray_matter::engine::YAML;
use gray_matter::Matter;

#[derive(Debug, Serialize, Deserialize)]
struct ResponseNote {
    file_data: NoteFile,
    data: Notes,
}

// ! REFEATORAÇÃO NECESSARIA
#[tauri::command]
pub async fn create_note(app: tauri::AppHandle, note: &str) -> Result<(), String> {
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

// ! REFEATORAÇÃO NECESSARIA
#[tauri::command]
pub async fn find_all_notes(app: tauri::AppHandle) -> Result<String, String> {
    let notes = Notes::find_all(app);

    let mut note_response: Vec<ResponseNote> = vec![];

    let mut matter = Matter::<YAML>::new();
    matter.delimiter = "***".to_owned();

    for note in &notes {
        let data: Notes = matter
            .parse(&note.contents)
            .data
            .clone()
            .unwrap()
            .deserialize()
            .unwrap();

        note_response.push(ResponseNote {
            file_data: NoteFile {
                file_name: (note.file_name.clone()),
                file_path: (note.file_path.clone()),
                contents: (note.contents.clone()),
            },
            data: data,
        })
    }

    Ok(serde_json::to_string(&note_response).unwrap())
}
