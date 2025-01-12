use serde::{Deserialize, Serialize};

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
            },
        }
    }
}
