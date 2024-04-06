use crate::{files::resource_files::parse_resource_from_path, traits::file_path::FilePath};

use crate::resources::aamrs_config::AamrsConfig;

use super::resource_files::save_resource_to_path;

pub fn load_config() -> AamrsConfig {
    parse_resource_from_path(AamrsConfig::filepath())
}

pub fn save_config(config: &AamrsConfig) -> Option<()> {
    save_resource_to_path(config, AamrsConfig::filepath())
}

#[cfg(test)]
mod tests {
    use crate::{
        files::config::{load_config, save_config},
        resources::aamrs_config::AamrsConfig,
    };

    #[test]
    fn test_config_retrieval() {
        let original_config = AamrsConfig {
            theme_mode: crate::resources::aamrs_config::Theme::Light,
        };

        if let Some(()) = save_config(&original_config) {
            let retrieved_config = load_config();
            assert_eq!(retrieved_config, original_config);
        } else {
            panic!("Error loading config")
        }
    }
}
