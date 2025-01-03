use anyhow::Result;
use directories::ProjectDirs;
use rusqlite::{Connection, Result as SqliteResult};
use std::{fs::create_dir_all, path::PathBuf, sync::RwLock};

use crate::types::{
    common::ContentType,
    database::{StorageItem, UserActivity},
};

#[derive(Debug)]
pub struct Database {
    conn: RwLock<Connection>,
}

unsafe impl Send for Database {}
unsafe impl Sync for Database {}

impl Database {
    pub fn new() -> Result<Self> {
        let db_path = get_database_path()?;
        let conn = Connection::open(db_path)?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS content (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                content_type TEXT NOT NULL,
                external_id TEXT NOT NULL,
                name TEXT NOT NULL,
                description TEXT,
                thumbnail_path TEXT NOT NULL,
                content_path TEXT NOT NULL,
                UNIQUE(content_type, external_id)
            )",
            [],
        )?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS user_activity (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                content_id INTEGER NOT NULL,
                date TEXT NOT NULL,
                minutes_watched INTEGER,
                minutes_read INTEGER,
                characters_read INTEGER,
                FOREIGN KEY(content_id) REFERENCES content(id)
            )",
            [],
        )?;

        Ok(Database {
            conn: RwLock::new(conn),
        })
    }

    pub fn write_item(&self, item: &StorageItem) -> Result<()> {
        let conn = self
            .conn
            .write()
            .map_err(|e| anyhow::Error::msg(e.to_string()))?;

        conn.execute(
            "INSERT OR REPLACE INTO content (
                    content_type, external_id, name, description, 
                    thumbnail_path, content_path
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            (
                format!("{:?}", item.content_type),
                &item.external_id,
                &item.name,
                &item.description,
                &item.thumbnail_path,
                &item.content_path,
            ),
        )?;

        Ok(())
    }

    pub fn read_items(&self, content_type: ContentType) -> Result<Vec<StorageItem>> {
        let conn = self
            .conn
            .read()
            .map_err(|e| anyhow::Error::msg(e.to_string()))?;

        let mut stmt = conn.prepare(
            "SELECT id, content_type, external_id, name, description, thumbnail_path, content_path FROM content WHERE content_type = ?1"
        )?;

        let items = stmt.query_map([format!("{:?}", content_type)], |row| {
            Ok(StorageItem {
                id: row.get(0)?,
                content_type: content_type.clone(),
                external_id: row.get(2)?,
                name: row.get(3)?,
                description: row.get(4)?,
                thumbnail_path: row.get(5)?,
                content_path: row.get(6)?,
            })
        })?;

        let items = items.collect::<SqliteResult<Vec<StorageItem>>>()?;

        Ok(items)
    }

    pub fn write_user_activity(&self, activity: &UserActivity) -> Result<()> {
        let conn = self
            .conn
            .write()
            .map_err(|e| anyhow::Error::msg(e.to_string()))?;

        conn.execute(
            "INSERT INTO user_activity (content_id, date, minutes_watched, minutes_read, characters_read) VALUES (?1, ?2, ?3, ?4, ?5)",
            (activity.content_id, &activity.date, activity.minutes_watched, activity.minutes_read, activity.characters_read),
        )?;

        Ok(())
    }
}

fn get_database_path() -> Result<PathBuf> {
    let proj_dirs = ProjectDirs::from("com", "muhei", "murasaki")
        .ok_or_else(|| anyhow::Error::msg("Failed to find project directories."))?;

    // Ensure the data directory exists to prevent rusqlite Connection from hanging
    create_dir_all(proj_dirs.data_dir()).map_err(|e| anyhow::Error::msg(e.to_string()))?;

    Ok(proj_dirs.data_dir().join("database.db"))
}
