use anyhow::Result;
use directories::ProjectDirs;
use rusqlite::{Connection, Result as SqliteResult};
use std::{fs::create_dir_all, path::PathBuf, sync::RwLock};

use crate::types::{
    common::{Content, ContentType},
    database::{ContentWithStats, Episode, UserActivity},
    stats::{ActivityStats, DailyActivity},
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

    // pub fn read_items(&self, content_type: ContentType) -> Result<Vec<Content>> {
    //     let conn = self
    //         .conn
    //         .read()
    //         .map_err(|e| anyhow::Error::msg(e.to_string()))?;

    //     let mut stmt = conn.prepare(
    //         "SELECT id, external_id, content_type, title, title_japanese,
    //             description, file_path, image_path, release_date,
    //             episodes, length_minutes, length_votes, rating, votecount
    //         FROM content
    //         WHERE content_type = ?1",
    //     )?;

    //     let items = stmt.query_map([format!("{:?}", content_type)], |row| {
    //         Ok(Content {
    //             id: row.get(0)?,
    //             external_id: row.get(1)?,
    //             content_type: content_type.clone(),
    //             title: row.get(3)?,
    //             title_japanese: row.get(4)?,
    //             description: row.get(5)?,
    //             file_path: row.get(6)?,
    //             image_path: row.get(7)?,
    //             release_date: row.get(8)?,
    //             episodes: row.get(9)?,
    //             length_minutes: row.get(10)?,
    //             length_votes: row.get(11)?,
    //             rating: row.get(12)?,
    //             votecount: row.get(13)?,
    //         })
    //     })?;

    //     let items = items.collect::<SqliteResult<Vec<Content>>>()?;

    //     Ok(items)
    // }

    pub fn read_content_with_stats(
        &self,
        content_type: ContentType,
    ) -> Result<Vec<ContentWithStats>> {
        let conn = self
            .conn
            .read()
            .map_err(|e| anyhow::Error::msg(e.to_string()))?;

        let mut stmt = conn.prepare(
            "SELECT
                c.*,
                MAX(ua.date) as last_active,
                SUM(COALESCE(ua.minutes_watched, 0) + COALESCE(ua.minutes_read, 0)) as total_minutes
            FROM content c
            LEFT JOIN user_activity ua ON c.id = ua.content_id
            WHERE c.content_type = ?1
            GROUP BY c.id",
        )?;

        let items = stmt.query_map([format!("{:?}", content_type)], |row| {
            Ok(ContentWithStats {
                content: Content {
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
                },
                last_active: row.get(14)?,
                total_minutes: row.get(15)?,
            })
        })?;

        Ok(items.collect::<SqliteResult<Vec<_>>>()?)
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

    pub fn write_episodes(&self, episodes: &[Episode]) -> Result<()> {
        let conn = self
            .conn
            .write()
            .map_err(|e| anyhow::Error::msg(e.to_string()))?;

        for episode in episodes {
            conn.execute("INSERT INTO episodes (content_id, episode_number, path, watched) VALUES (?1, ?2, ?3, ?4)",
            (episode.content_id.clone(), episode.episode_number, episode.path.clone(), episode.watched),
            )?;
        }

        Ok(())
    }

    pub fn read_episodes(&self, external_id: &str) -> Result<Vec<Episode>> {
        let conn = self
            .conn
            .read()
            .map_err(|e| anyhow::Error::msg(e.to_string()))?;

        let mut stmt = conn.prepare(
            "SELECT id, content_id, episode_number, path, watched 
            FROM episodes 
            WHERE content_id = ?1",
        )?;

        let episodes = stmt.query_map([external_id], |row| {
            Ok(Episode {
                id: row.get(0)?,
                content_id: row.get(1)?,
                episode_number: row.get(2)?,
                path: row.get(3)?,
                watched: row.get(4)?,
            })
        })?;

        let episodes = episodes.collect::<SqliteResult<Vec<Episode>>>()?;

        Ok(episodes)
    }

    pub fn get_activity_stats(&self, start_date: &str, end_date: &str) -> Result<ActivityStats> {
        let conn = self
            .conn
            .read()
            .map_err(|e| anyhow::Error::msg(e.to_string()))?;

        let mut stmt = conn.prepare(
            "SELECT
                c.content_type,
                SUM(COALESCE(ua.minutes_watched, 0) + COALESCE(ua.minutes_read, 0)) as total_minutes,
                COUNT(DISTINCT DATE(ua.date)) as active_days
            FROM content c
            JOIN user_activity ua ON c.id = ua.content_id
            WHERE ua.date BETWEEN ?1 and ?2
            GROUP BY c.content_type"
        )?;

        let activity_stats = stmt.query_map([start_date, end_date], |row| {
            let content_type: String = row.get(0)?;
            let minutes: u64 = row.get(1)?;
            let days: u64 = row.get(2)?;
            Ok((content_type, minutes, days))
        })?;

        let mut anime_minutes = 0;
        let mut vn_minutes = 0;
        let mut active_days = 0;

        for result in activity_stats {
            let (content_type, minutes, days) = result?;
            match content_type.as_str() {
                "Anime" => anime_minutes = minutes,
                "Vn" => vn_minutes = minutes,
                _ => continue,
            }

            active_days = active_days.max(days);
        }

        let mut stmt = conn.prepare(
            "SELECT
                DATE(date) as day,
                SUM(COALESCE(minutes_watched, 0) + COALESCE(minutes_read, 0)) as minutes
            FROM user_activity
            WHERE date BETWEEN ?1 and ?2
            GROUP BY DATE(date)
            ORDER BY day",
        )?;

        let daily_stats = stmt.query_map([start_date, end_date], |row| {
            Ok(DailyActivity {
                date: row.get(0)?,
                minutes: row.get(1)?,
            })
        })?;

        let daily_activity = daily_stats.collect::<SqliteResult<Vec<_>>>()?;

        Ok(ActivityStats {
            anime_minutes,
            vn_minutes,
            total_minutes: anime_minutes + vn_minutes,
            active_days: active_days,
            daily_activity,
        })
    }

    pub fn delete_item(&self, content_id: u64) -> Result<()> {
        let conn = self
            .conn
            .write()
            .map_err(|e| anyhow::Error::msg(e.to_string()))?;

        conn.execute("DELETE FROM content WHERE external_id = ?1", (content_id,))?;

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
