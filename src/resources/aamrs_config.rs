use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use crate::files::resource_files::parse_resource_from_path;
use crate::files::resource_files::save_resource_to_path;
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
impl AamrsConfig {
    pub fn load_config() -> AamrsConfig {
        println!("Loading config...");
        parse_resource_from_path(AamrsConfig::filepath())
    }
    pub fn save_config(&self) -> Option<()> {
        println!("Saving config...");
        save_resource_to_path(self, AamrsConfig::filepath())
    }
}
impl Default for AamrsConfig {
    fn default() -> Self {
        Self { theme_mode: Theme::System }
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
#[cfg(test)]
mod tests {
    use super::AamrsConfig;
    #[test]
    fn test_config_retrieval() {
        let original_config = AamrsConfig {
            theme_mode: crate::resources::aamrs_config::Theme::Light,
        };
        if let Some(()) = original_config.save_config() {
            let retrieved_config = AamrsConfig::load_config();
            assert_eq!(retrieved_config, original_config);
        } else {
            panic!("Error loading config")
        }
    }
}
