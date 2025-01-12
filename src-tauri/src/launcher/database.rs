use anyhow::Result;
use rusqlite::Result as SqliteResult;

use crate::{
    common::database::Database,
    library::types::{Episode, UserActivity},
};

impl Database {
    pub fn write_user_activity(&self, activity: &UserActivity) -> Result<()> {
        let conn = self.write()?;

        conn.execute(
            "INSERT INTO user_activity (content_id, date, minutes_watched, minutes_read, characters_read) VALUES (?1, ?2, ?3, ?4, ?5)",
            (activity.content_id, &activity.date, activity.minutes_watched, activity.minutes_read, activity.characters_read),
        )?;

        Ok(())
    }

    pub fn write_episodes(&self, episodes: &[Episode]) -> Result<()> {
        let conn = self.write()?;

        for episode in episodes {
            conn.execute("INSERT INTO episodes (content_id, episode_number, path, watched) VALUES (?1, ?2, ?3, ?4)",
            (episode.content_id.clone(), episode.episode_number, episode.path.clone(), episode.watched),
            )?;
        }

        Ok(())
    }

    pub fn read_episodes(&self, external_id: &str) -> Result<Vec<Episode>> {
        let conn = self.read()?;

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
}
