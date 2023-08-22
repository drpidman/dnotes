use tauri::Manager;

pub enum Databases {
    Notes,
    Analytics,
    History
}

fn get_app_data(app: &tauri::AppHandle) -> String {
    app.path_resolver()
    .app_data_dir()
    .unwrap().display().to_string()
}

/**
Return com.app.info/data
 */
pub fn get_data_dir(app: &tauri::AppHandle) -> String {
    if cfg!(windows) {
        get_app_data(app) + "data"
    } else {
        get_app_data(app) + "/data"
    }
}

pub fn get_notes_dir(app: &tauri::AppHandle) -> String {
    if cfg!(windows) {
        get_app_data(&app.app_handle()) + "notes"
    } else {
        get_app_data(&app.app_handle()) + "/notes"
    }
}

pub fn get_database(app: &tauri::AppHandle, database: Databases) -> String {
    match database {
        Databases::Notes => {
            if cfg!(windows) {
                get_data_dir(app) + "\\notes.sqlite"
            } else {
                get_data_dir(app) + "/notes.sqlite"
            }
        },
        _ => "".to_string()
    }
}