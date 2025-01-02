use directories::ProjectDirs;
use rusqlite::{Connection, Result as SqliteResult};
use std::{
    fs::create_dir_all,
    path::PathBuf,
    sync::{Arc, RwLock},
};

use crate::types::{ApiError, ContentType, Result, StorageItem, TimeEntry};

#[derive(Debug)]
pub struct Database {
    conn: Arc<RwLock<Connection>>,
}

unsafe impl Send for Database {}
unsafe impl Sync for Database {}

impl Database {
    pub fn new() -> Result<Self> {
        let db_path = get_database_path().map_err(|e| ApiError::DatabaseError(e.to_string()))?;
        let conn = Connection::open(db_path).map_err(|e| ApiError::DatabaseError(e.to_string()))?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS content (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                content_type TEXT NOT NULL,
                external_id TEXT NOT NULL,
                name TEXT NOT NULL,
                description TEXT,
                thumbnail_path TEXT NOT NULL,
                executable_path TEXT NOT NULL,
                UNIQUE(content_type, external_id)
            )",
            [],
        )
        .map_err(|e| ApiError::DatabaseError(e.to_string()))?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS time_entries (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                content_id INTEGER NOT NULL,
                start_time TEXT NOT NULL,
                duration_secs INTEGER NOT NULL,
                FOREIGN KEY(content_id) REFERENCES content(id)
            )",
            [],
        )
        .map_err(|e| ApiError::DatabaseError(e.to_string()))?;

        Ok(Database {
            conn: Arc::new(RwLock::new(conn)),
        })
    }

    pub fn write_item(&self, item: &StorageItem) -> Result<()> {
        let conn = self
            .conn
            .write()
            .map_err(|e| ApiError::DatabaseError(e.to_string()))?;

        conn.execute(
            "INSERT OR REPLACE INTO content (
                    content_type, external_id, name, description, 
                    thumbnail_path, executable_path
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            (
                format!("{:?}", item.content_type),
                &item.external_id,
                &item.name,
                &item.description,
                &item.thumbnail_path,
                &item.executable_path,
            ),
        )
        .map_err(|e| ApiError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    pub fn read_items(&self, content_type: ContentType) -> Result<Vec<StorageItem>> {
        let conn = self
            .conn
            .read()
            .map_err(|e| ApiError::DatabaseError(e.to_string()))?;

        let mut stmt = conn.prepare(
        "SELECT id, content_type, external_id, name, description, thumbnail_path, executable_path FROM content WHERE content_type = ?1"
      ).map_err(|e| ApiError::DatabaseError(e.to_string()))?;

        let items = stmt
            .query_map([format!("{:?}", content_type)], |row| {
                Ok(StorageItem {
                    id: row.get(0)?,
                    content_type: content_type.clone(),
                    external_id: row.get(2)?,
                    name: row.get(3)?,
                    description: row.get(4)?,
                    thumbnail_path: row.get(5)?,
                    executable_path: row.get(6)?,
                })
            })
            .map_err(|e| ApiError::DatabaseError(e.to_string()))?;

        let items = items
            .collect::<SqliteResult<Vec<StorageItem>>>()
            .map_err(|e| ApiError::DatabaseError(e.to_string()))?;

        Ok(items)
    }

    pub fn log_time_entry(&self, entry: &TimeEntry) -> Result<()> {
        let conn = self
            .conn
            .write()
            .map_err(|e| ApiError::DatabaseError(e.to_string()))?;

        conn.execute(
            "INSERT INTO time_entries (content_id, start_time, duration_secs) VALUES (?1, ?2, ?3)",
            (entry.content_id, &entry.start_time, entry.duration),
        )
        .map_err(|e| ApiError::DatabaseError(e.to_string()))?;

        Ok(())
    }
}

fn get_database_path() -> Result<PathBuf> {
    let proj_dirs = ProjectDirs::from("com", "muhei", "murasaki")
        .ok_or_else(|| ApiError::DatabaseError("Failed to find project directories.".into()))?;

    // Ensure the data directory exists to prevent rusqlite Connection from hanging
    create_dir_all(proj_dirs.data_dir()).map_err(|e| ApiError::DatabaseError(e.to_string()))?;

    Ok(proj_dirs.data_dir().join("database.db"))
}
