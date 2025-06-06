use std::{os::windows::process::CommandExt, process::Command, time::Instant};

use anyhow::{Ok, Result};
use chrono::Local;

use crate::{
    common::types::{Content, ContentType},
    library::types::{Episode, UserActivity},
    settings::service::load_config,
};

// Handles the time-logging aspect of the launch process, starting before the first process is opened.
// Will wait until the async to finish (the process to close) to record the time spent.
pub async fn launch_content(
    content: &Content,
    episode: &Option<Episode>,
) -> Result<(UserActivity, bool)> {
    let start_instant = Instant::now();

    let mut user_activity = UserActivity {
        id: 0,
        date: Local::now().to_rfc3339(),
        minutes_watched: 0,
        minutes_read: 0,
        characters_read: 0,
        content_id: content.id,
    };

    let mut complete = false;

    match content.content_type {
        ContentType::Vn => {
            launch_vn(content).await?;
            user_activity.minutes_read = start_instant.elapsed().as_secs() / 60;
        }
        ContentType::Anime => {
            if let Some(ep) = episode {
                launch_anime(&ep).await?;
            }
            user_activity.minutes_watched = start_instant.elapsed().as_secs() / 60;

            // Naive solution for solving if episode has been watched or not
            if user_activity.minutes_watched > 18 {
                complete = true;
            }
        }
    }

    Ok((user_activity, complete))
}

// Spawns a window using the path specified inside of the vn parameter.
// Textractor will automatically close after the VN is closed.
async fn launch_vn(vn: &Content) -> Result<()> {
    let config = load_config()?;
    let file_path = vn
        .file_path
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("File path is None"))?;
    let exe_path = std::path::Path::new(file_path);
    let working_dir = exe_path
        .parent()
        .ok_or_else(|| anyhow::anyhow!("Invalid executable path"))?;

    let mut command = Command::new(exe_path);

    // Defining here allows us to kill on VN close
    let mut textractor = Command::new(&config.vn.textractor_executable);
    let mut textractor_child = None;

    // Open textractor or texthooker if requested in settings
    if config.vn.textractor_enabled {
        textractor_child = Some(textractor.spawn()?);
    }

    if config.vn.texthooker_enabled {
        open::that("https://anacreondjt.gitlab.io/texthooker.html");
    }

    // Change the working directory to avoid errors from the VN
    command.current_dir(working_dir);

    #[cfg(target_os = "windows")]
    command.creation_flags(winapi::um::winbase::DETACHED_PROCESS);

    let mut child = command.spawn()?;
    child.wait()?;

    // Kill Textractor process after VN closes
    if let Some(mut textractor_child) = textractor_child {
        textractor_child.kill()?;
    }

    Ok(())
}

// Spawns a window using the user's media player of choice and will wait until that window is closed to complete.
async fn launch_anime(episode: &Episode) -> Result<()> {
    let config = load_config()?;
    let mut command = Command::new(&config.player.executable);

    command.args(&config.player.args);
    command.arg(&episode.path);

    #[cfg(target_os = "windows")]
    command.creation_flags(winapi::um::winbase::DETACHED_PROCESS);

    let mut child = command.spawn()?;
    child.wait()?;

    Ok(())
}
