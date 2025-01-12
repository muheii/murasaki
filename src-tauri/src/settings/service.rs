use std::{fs::create_dir_all, path::PathBuf};

use super::types::Config;
use anyhow::Result;
use directories::ProjectDirs;

pub fn load_config() -> Result<Config> {
    let config_path = get_config_path()?;

    if !config_path.exists() {
        let default_config = Config::default();
        save_config(&default_config)?;
        return Ok(default_config);
    }

    let content = std::fs::read_to_string(config_path)?;

    serde_json::from_str(&content).map_err(|e| anyhow::anyhow!(e))
}

pub fn save_config(config: &Config) -> Result<()> {
    let config_path = get_config_path()?;

    if let Some(parent) = config_path.parent() {
        create_dir_all(parent)?;
    }

    let content = serde_json::to_string_pretty(config)?;
    std::fs::write(config_path, content)?;

    Ok(())
}

fn get_config_path() -> Result<PathBuf> {
    let proj_dirs = ProjectDirs::from("com", "muhei", "murasaki")
        .ok_or_else(|| anyhow::anyhow!("Failed to get project directories"))?;

    Ok(proj_dirs.config_dir().join("config.json"))
}
