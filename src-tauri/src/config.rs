use anyhow::Result;
use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::{fs::create_dir_all, path::PathBuf};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub player: PlayerConfig,
    pub interface: InterfaceConfig,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerConfig {
    pub executable: String,
    pub args: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InterfaceConfig {
    pub theme: String,
    pub language: String,
    pub grid_size: u32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            player: PlayerConfig {
                executable: "mpv".to_string(),
                args: vec!["--fullscreen".to_string()],
            },
            interface: InterfaceConfig {
                theme: "dark".to_string(),
                language: "en".to_string(),
                grid_size: 3,
            },
        }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        let config_path = get_config_path()?;
        if !config_path.exists() {
            let default_config = Config::default();
            default_config.save()?;
            return Ok(default_config);
        }

        let content = std::fs::read_to_string(config_path)?;

        serde_json::from_str(&content).map_err(|e| anyhow::anyhow!(e))
    }

    pub fn save(&self) -> Result<()> {
        let config_path = get_config_path()?;

        if let Some(parent) = config_path.parent() {
            create_dir_all(parent)?;
        }

        let content = serde_json::to_string_pretty(self)?;

        std::fs::write(config_path, content)?;

        Ok(())
    }
}

fn get_config_path() -> Result<PathBuf> {
    let proj_dirs = ProjectDirs::from("com", "muhei", "murasaki")
        .ok_or_else(|| anyhow::anyhow!("Failed to get project directories"))?;

    Ok(proj_dirs.config_dir().join("config.json"))
}
