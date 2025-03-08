use crate::common::database::Database;
use anyhow::Result;
use rusqlite::Result as SqliteResult;

use super::types::{ActivityStats, DailyActivity};

impl Database {
    pub fn get_activity_stats(&self, start_date: &str, end_date: &str) -> Result<ActivityStats> {
        let conn = self.read()?;

        let mut stmt = conn.prepare(
            "SELECT
                c.content_type,
                SUM(ua.minutes_watched + ua.minutes_read) as total_minutes
            FROM content c
            JOIN user_activity ua ON c.id = ua.content_id
            WHERE ua.date BETWEEN ?1 and ?2
            GROUP BY c.content_type",
        )?;

        let activity_stats = stmt.query_map([start_date, end_date], |row| {
            let content_type: String = row.get(0)?;
            let minutes: u64 = row.get(1)?;
            Ok((content_type, minutes))
        })?;

        let mut anime_minutes = 0;
        let mut vn_minutes = 0;

        for result in activity_stats {
            let (content_type, minutes) = result?;
            match content_type.as_str() {
                "Anime" => anime_minutes = minutes,
                "Vn" => vn_minutes = minutes,
                _ => continue,
            }
        }

        let mut stmt = conn.prepare(
            "SELECT
                DATE(date) as day,
                SUM(minutes_watched + minutes_read) as minutes,
                SUM(minutes_read) as reading_minutes,
                SUM(minutes_watched) as listening_minutes,
                COALESCE(SUM(CASE WHEN c.content_type = 'Anime' THEN minutes_watched END), 0) as anime_minutes,
                COALESCE(SUM(CASE WHEN c.content_type = 'Vn' THEN minutes_read END), 0) as vn_minutes
            FROM user_activity ua
            JOIN content c ON ua.content_id = c.id
            WHERE ua.date BETWEEN ?1 and ?2
            GROUP BY DATE(date)
            ORDER BY day",
        )?;

        let daily_stats = stmt.query_map([start_date, end_date], |row| {
            Ok(DailyActivity {
                date: row.get(0)?,
                total_minutes: row.get(1)?,
                reading_minutes: row.get(2)?,
                listening_minutes: row.get(3)?,
                anime_minutes: row.get(4)?,
                vn_minutes: row.get(5)?,
            })
        })?;

        let daily_activity = daily_stats.collect::<SqliteResult<Vec<_>>>()?;

        let mut stmt = conn.prepare(
            "SELECT DATE(date) FROM user_activity GROUP BY DATE(date) ORDER BY DATE(date)",
        )?;

        let dates = stmt.query_map([], |row| {
            let date: String = row.get(0)?;
            Ok(date)
        })?;

        let dates: Vec<String> = dates.collect::<SqliteResult<Vec<_>>>()?;

        let reading_minutes = vn_minutes;
        let listening_minutes = anime_minutes;

        Ok(ActivityStats {
            total_minutes: anime_minutes + vn_minutes,
            reading_minutes,
            listening_minutes,
            anime_minutes,
            vn_minutes,
            current_streak: 0,
            best_streak: 0,
            daily_activity,
        })
    }
}
