use anyhow::Result;
use chrono::Local;
use std::time::Instant;
use std::{os::windows::process::CommandExt, process::Command};

use crate::config::Config;
use crate::types::common::{Content, ContentType};
use crate::types::database::{Episode, UserActivity};

impl ContentType {
    pub async fn launch(
        &self,
        content: &Content,
        episode: Option<Episode>,
    ) -> Result<UserActivity> {
        let start_instant = Instant::now();

        let mut user_activity = UserActivity {
            id: 0,
            date: Local::now().to_rfc3339(),
            minutes_watched: 0,
            minutes_read: 0,
            characters_read: 0,
            content_id: content.id,
        };

        match self {
            ContentType::Vn => {
                launch_vn(content).await?;
                user_activity.minutes_read = start_instant.elapsed().as_secs() / 60;
            }
            ContentType::Anime => {
                if let Some(ep) = episode {
                    launch_anime(&ep).await?;
                }
                user_activity.minutes_watched = start_instant.elapsed().as_secs() / 60;
            }
        }

        Ok(user_activity)
    }
}

async fn launch_vn(vn: &Content) -> Result<()> {
    let file_path = vn
        .file_path
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("File path is None"))?;
    let exe_path = std::path::Path::new(file_path);
    let working_dir = exe_path
        .parent()
        .ok_or_else(|| anyhow::anyhow!("Invalid executable path"))?;

    let mut command = Command::new(exe_path);

    // Change the working directory to avoid errors from the VN
    command.current_dir(working_dir);

    #[cfg(target_os = "windows")]
    command.creation_flags(winapi::um::winbase::DETACHED_PROCESS);

    let mut child = command.spawn()?;
    child.wait()?;

    Ok(())
}

async fn launch_anime(episode: &Episode) -> Result<()> {
    let config = Config::load()?;

    let mut command = Command::new(&config.player.executable);

    command.args(&config.player.args);

    command.arg(&episode.path);

    #[cfg(target_os = "windows")]
    command.creation_flags(winapi::um::winbase::DETACHED_PROCESS);

    let mut child = command.spawn()?;
    child.wait()?;

    Ok(())
}
