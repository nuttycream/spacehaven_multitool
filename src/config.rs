use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct SharedConfig {
    game_path: String,
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct ModdingViewConfig {
    // modding-specific fields here
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct SaveEditingViewConfig {
    // save-editing-specific fields here
}

#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Config {
    pub common: SharedConfig,
    pub modding: ModdingViewConfig,
    pub save_editing: SaveEditingViewConfig,
}

impl Config {
    pub fn load_from_file(path: &str) -> Self {
        Config::default()
    }

    pub fn save_to_file(&self, path: &str) {
        // implementation here
    }
}
