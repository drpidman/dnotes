use serde::*;
use chrono::*;

#[derive(Debug, Serialize, Deserialize)]
/// ### Note - Struct for note
/// - Create a new notes object, parse to JSON for ready data
pub struct Notes {
    pub note: String,
    pub description: String,
    pub accent_color: i32,
    pub content: String,
    pub created_at: std::time::Duration
}


impl NotesActions for Notes {
    fn create() {
        
    }
    
    fn find() {
        
    }

    fn delete() {
        
    }
}

trait NotesActions {
    fn create() {}
    fn find() {}
    fn delete() {}
}