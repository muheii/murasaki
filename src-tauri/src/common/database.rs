use anyhow::Result;
use directories::ProjectDirs;
use rusqlite::Connection;
use std::{fs::create_dir_all, path::PathBuf, sync::RwLock};

pub struct Database {
    conn: RwLock<Connection>,
}

unsafe impl Sync for Database {}
unsafe impl Send for Database {}

impl Database {
    pub fn new() -> Result<Self> {
        let db_path = get_database_path()?;
        let conn = Connection::open(db_path)?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS content (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                external_id TEXT NOT NULL,
                content_type TEXT NOT NULL,
                title TEXT NOT NULL,
                title_japanese TEXT,
                description TEXT,
                file_path TEXT,
                image_path TEXT NOT NULL,
                release_date TEXT,
                episodes INTEGER,
                length_minutes INTEGER,
                length_votes INTEGER,
                rating INTEGER,
                votecount INTEGER,
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

        conn.execute(
            "CREATE TABLE IF NOT EXISTS episodes (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                content_id TEXT NOT NULL,
                episode_number INTEGER NOT NULL,
                path TEXT NOT NULL,
                watched BOOL NOT NULL
            )",
            [],
        )?;

        Ok(Database {
            conn: RwLock::new(conn),
        })
    }

    pub fn read(&self) -> Result<std::sync::RwLockReadGuard<'_, Connection>> {
        self.conn
            .read()
            .map_err(|e| anyhow::Error::msg(e.to_string()))
    }

    pub fn write(&self) -> Result<std::sync::RwLockWriteGuard<'_, Connection>> {
        self.conn
            .write()
            .map_err(|e| anyhow::Error::msg(e.to_string()))
    }
}

fn get_database_path() -> Result<PathBuf> {
    let proj_dirs = ProjectDirs::from("com", "muhei", "murasaki")
        .ok_or_else(|| anyhow::Error::msg("Failed to find project directories."))?;

    create_dir_all(proj_dirs.data_dir())?;

    Ok(proj_dirs.data_dir().join("database.db"))
}
