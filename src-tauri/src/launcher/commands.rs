use anyhow_tauri::TAResult;
use tauri::State;

use crate::{
    common::{database::Database, types::Content},
    library::types::Episode,
};

use super::service;

#[tauri::command]
pub async fn launch_content(
    db: State<'_, Database>,
    content: Content,
    episode: Option<Episode>,
) -> TAResult<()> {
    let user_activity = service::launch_content(&content, &episode).await?;
    db.write_user_activity(&user_activity.0)?;

    if user_activity.1 == true {
        if let Some(ep) = episode {
            db.write_completed(&ep)?;
        }
    }

    Ok(())
}
