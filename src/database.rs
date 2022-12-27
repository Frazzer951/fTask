use std::path::PathBuf;

use rusqlite::Connection;

pub struct Database {
    pub conn: Connection,
    pub path: PathBuf,
}

impl Database {
    pub fn new() -> Database {
        // Create SQLite directory
        let mut config_dir = dirs::config_dir().unwrap();
        config_dir.push("ftask");
        let mut sqlite_path = config_dir;
        std::fs::create_dir_all(&sqlite_path).expect("Failed to create task database directory");
        sqlite_path.push("tasks.sqlite");
        let conn = Connection::open(&sqlite_path).expect("Failed to open database");

        // Make sure the Task Table Exists
        conn.execute(
            "CREATE TABLE IF NOT EXISTS task (
                  id                INTEGER PRIMARY KEY,
                  priority          INTEGER NOT NULL,
                  name              TEXT NOT NULL,
                  description       TEXT NOT NULL,
                  completed         INTEGER NOT NULL
                  )",
            [],
        )
        .expect("Failed to create task database");

        Database { conn, path: sqlite_path }
    }
}
