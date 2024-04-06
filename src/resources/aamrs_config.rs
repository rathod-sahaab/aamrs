use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::{files::paths::get_config_path, traits::file_path::FilePath};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum Theme {
    Dark,
    Light,
    System,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct AamrsConfig {
    pub theme_mode: Theme,
}

impl Default for AamrsConfig {
    fn default() -> Self {
        Self {
            theme_mode: Theme::System,
        }
    }
}

impl FilePath for AamrsConfig {
    fn filepath() -> std::path::PathBuf {
        if let Some(config_dir) = get_config_path() {
            return config_dir.join("config.json");
        }
        PathBuf::from("./config.json")
    }
}
