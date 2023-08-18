#[cfg(test)]
mod tests {
    use rusqlite::Connection;

    use serde::*;
    use serde_json::from_str;
    
    #[derive(Debug, Serialize, Deserialize)]
    /// ### Note - Struct for note
    /// - Create a new notes object, parse to JSON for ready data
    pub struct Notes {
        pub id: i32,
        pub note: String,
        pub description: String,
        pub accent_color: i32,
        pub content: String,
        pub created_at: i32,
    }
    
    const NOTES_TABLE_SQL_STRING: &'static str = 
    "CREATE TABLE IF NOT EXISTS notes(id INTEGER PRIMARY KEY AUTOINCREMENT, 
        note TEXT,
        description TEXT,
        accent_color INTEGER,
        content TEXT,
        created_at Date
    )";

    const NOTES_TABLE_SQL_INSERT: &'static str =
    "INSERT INTO notes(id, note, description, accent_color, content, created_at) VALUES(
        :id,
        :note,
        :description,
        :accent_color,
        :content,
        :created_at
    )";

    fn init_memory_db() -> Connection {
        match Connection::open("test.sqlite") {
            Ok(db) => db,
            Err(err) => {
                println!("Error to connect:{:}", err);
                panic!();
            }
        }
    }

    #[test]
    fn create_database() {
         let db = init_memory_db();

        // creating tables
        match db.execute(NOTES_TABLE_SQL_STRING, ()) {
                Ok(res) => {
                    println!("Table created");
                    assert_eq!(res, 0);
                },
                Err(err) => {
                    println!("Error, table not created:{:}", err);
                }
        };
    }

    #[test]
    fn insert_into_notes_query() {
        let db = init_memory_db();
        
        let mut notes: Vec<Notes> = vec![];

        match db.execute(NOTES_TABLE_SQL_INSERT, &[
            (":id", "5"),
            (":note", "Note name"),
            (":description", "Note description"),
            (":accent_color",  &0x12.to_string()),
            (":content", "Note content"),
            (":created_at", &std::time::Duration::new(5, 6).as_millis().to_string())
        ])
        {
            Ok(res) => {
                println!("Insertion:{:}", res);
                assert_eq!(res, 1)
            },
            Err(err) => {
                println!("Error, table not created:{:}", err);
            }
        }

        let mut stmt = match db.prepare("SELECT * FROM notes") {
            Ok(res) => res,
            Err(err) => {
                println!("Error, cannot prepare db: {:}", err);
                panic!()
            }
        };

        let rows = match stmt.query_map((), |f| {
            Ok(Notes {
                id: f.get(0)?,
                note: f.get(1)?,
                description: f.get(2)?,
                accent_color: f.get_unwrap::<usize, i32>(3),
                content: f.get(4)?,
                created_at: f.get(5)?
            })
        }) {
            Ok(res) => {
                println!("Statement executed");
                res
            },
            Err(err) => {
                println!("Error ocurred to execute stmt: {:}", err);
                panic!()
            }
        };

        for item in rows {
            notes.push(item.unwrap());
        }
    }
}