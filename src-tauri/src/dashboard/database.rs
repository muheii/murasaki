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
                SUM(COALESCE(ua.minutes_watched, 0) + COALESCE(ua.minutes_read, 0)) as total_minutes,
                COUNT(DISTINCT DATE(ua.date)) as active_days
            FROM content c
            JOIN user_activity ua ON c.id = ua.content_id
            WHERE ua.date BETWEEN ?1 and ?2
            GROUP BY c.content_type",
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
}
