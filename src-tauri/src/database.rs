use anyhow::Result;
use directories::ProjectDirs;
use rusqlite::{Connection, Result as SqliteResult};
use std::{fs::create_dir_all, path::PathBuf, sync::RwLock};

use crate::types::{
    common::{Content, ContentType},
    database::UserActivity,
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

        Ok(Database {
            conn: RwLock::new(conn),
        })
    }

    pub fn write_item(&self, item: &Content) -> Result<()> {
        let conn = self
            .conn
            .write()
            .map_err(|e| anyhow::Error::msg(e.to_string()))?;

        conn.execute(
            "INSERT OR REPLACE INTO content (
                    external_id, content_type, title, title_japanese,
                    description, file_path, image_path, release_date,
                    episodes, length_minutes, length_votes, rating, votecount
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)",
            (
                &item.external_id,
                format!("{:?}", item.content_type),
                &item.title,
                &item.title_japanese,
                &item.description,
                &item.file_path,
                &item.image_path,
                &item.release_date,
                &item.episodes,
                &item.length_minutes,
                &item.length_votes,
                &item.rating,
                &item.votecount,
            ),
        )?;

        Ok(())
    }

    pub fn read_items(&self, content_type: ContentType) -> Result<Vec<Content>> {
        let conn = self
            .conn
            .read()
            .map_err(|e| anyhow::Error::msg(e.to_string()))?;

        let mut stmt = conn.prepare(
            "SELECT id, external_id, content_type, title, title_japanese,
                description, file_path, image_path, release_date,
                episodes, length_minutes, length_votes, rating, votecount
            FROM content 
            WHERE content_type = ?1",
        )?;

        let items = stmt.query_map([format!("{:?}", content_type)], |row| {
            Ok(Content {
                id: row.get(0)?,
                external_id: row.get(1)?,
                content_type: content_type.clone(),
                title: row.get(3)?,
                title_japanese: row.get(4)?,
                description: row.get(5)?,
                file_path: row.get(6)?,
                image_path: row.get(7)?,
                release_date: row.get(8)?,
                episodes: row.get(9)?,
                length_minutes: row.get(10)?,
                length_votes: row.get(11)?,
                rating: row.get(12)?,
                votecount: row.get(13)?,
            })
        })?;

        let items = items.collect::<SqliteResult<Vec<Content>>>()?;

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
