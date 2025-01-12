use crate::common::{
    database::Database,
    types::{Content, ContentType},
};
use anyhow::Result;
use rusqlite::Result as SqliteResult;

use super::types::ContentWithStats;

impl Database {
    pub fn write_item(&self, item: &Content) -> Result<()> {
        let conn = self.write()?;

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

    pub fn read_content_with_stats(
        &self,
        content_type: ContentType,
    ) -> Result<Vec<ContentWithStats>> {
        let conn = self.read()?;

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

    pub fn delete_item(&self, content_id: u64) -> Result<()> {
        let conn = self.write()?;

        conn.execute("DELETE FROM content WHERE external_id = ?1", (content_id,))?;

        Ok(())
    }
}
