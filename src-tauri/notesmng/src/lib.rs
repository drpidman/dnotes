pub mod notes;

#[cfg(test)]
pub mod tests {
    use utils::path_resolver::get_notes_dir;

    use crate::notes::{NoteFile, Notes, NotesAction};
    use std::fs::{self, *};

    fn create(note: Notes) -> Result<Option<NoteFile>, String> {
        let notes_dir = "notes/".to_string();

        if let Some(find_note) = find_note(&note.title) {
            return Err(String::from(format!(
                "Note {} already exists",
                find_note.file_name
            )));
        } 

        let file_name = notes_dir + &note.title + ".md";

        match fs::File::create(&file_name) {
            Ok(_) => Ok(find_note(&note.title)),
            Err(err) => {
                println!("Failed to create file. Error:{:?}", err);
                panic!()
            }
        }
    }

    fn find_note(note: &str) -> Option<NoteFile> {
        let notes_dir = "notes".to_string();
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
                    found_note = Some(NoteFile {
                        file_name: (file_type.file_name().into_string().unwrap()),
                    });
                    break;
                }
            }
        }

        found_note
    }

    #[test]
    fn create_note_correct() {

        let expected_title = "Hello";

        match create(Notes {
            title: String::from("Hello World mamaco ashdahsdhahs"),
            description: String::from("Hello"),
            tags: vec![String::from("1")],
        }) {
            Ok(note) => {
                println!("Ok")
            },
            Err(err) => {
                println!("{}", err);
            }
        }

    }
}
