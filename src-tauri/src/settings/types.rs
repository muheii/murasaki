use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub player: PlayerConfig,
    pub interface: InterfaceConfig,
    pub vn: VnConfig,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct VnConfig {
    pub textractor_executable: String,
    pub textractor_enabled: bool,
    pub texthooker_enabled: bool,
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
            vn: VnConfig {
                textractor_executable: "".to_string(),
                textractor_enabled: false,
                texthooker_enabled: false,
            },
        }
    }
}
