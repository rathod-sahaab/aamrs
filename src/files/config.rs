use std::{fs, path::PathBuf};

use crate::resources::aamrs_config::AamrsConfig;

pub fn get_config_path(config_dir: &str) -> Option<PathBuf> {
    let home_dir = std::env::var_os("HOME")?;

    let CONFIG_PATH: [&str; 3] = [".config", config_dir, "config.json"];
    let config_path = PathBuf::from(home_dir).join(CONFIG_PATH.iter().collect::<PathBuf>());

    Some(config_path)
}

pub fn load_config() -> AamrsConfig {
    load_config_by_dir("aamrs")
}

pub fn load_config_by_dir(config_dir: &str) -> AamrsConfig {
    if let Some(config_path) = get_config_path(config_dir) {
        dbg!(&config_path);
        return load_config_from_path(config_path);
    }
    AamrsConfig::default()
}

pub fn load_config_from_path(config_path: PathBuf) -> AamrsConfig {
    if let Ok(content) = fs::read(config_path) {
        let file_string = String::from_utf8_lossy(&content);
        let config: AamrsConfig =
            serde_json::from_str(&file_string).unwrap_or(AamrsConfig::default());

        return config;
    }
    AamrsConfig::default()
}

pub fn save_config(config: &AamrsConfig) -> Option<PathBuf> {
    save_config_to_dir(config, "aamrs")
}

pub fn save_config_to_dir(config: &AamrsConfig, config_dir: &str) -> Option<PathBuf> {
    if let Some(config_path) = get_config_path(config_dir) {
        dbg!(&config_path);

        let config_dir = config_path.parent()?;

        fs::create_dir_all(config_dir).ok()?;

        let json_config = serde_json::to_string_pretty(&config).ok()?;

        fs::write(&config_path, json_config).ok()?;

        Some(config_path)
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        files::config::{load_config_by_dir, load_config_from_path, save_config_to_dir},
        resources::aamrs_config::{AamrsConfig, AamrsProject},
    };

    #[test]
    fn test_load_config_should_return_default() {
        let config = load_config_by_dir("aamrs-test");

        assert_eq!(config, AamrsConfig::default())
    }

    #[test]
    fn test_config_retrieval() {
        let original_config = AamrsConfig {
            projects: vec![AamrsProject {
                name: String::from("aamrs"),
                location: String::from("/home/abhay/personal/rust/aamrs"),
            }],
        };
        let config_dir_name = "aamrs-test-create";

        if let Some(config_path) = save_config_to_dir(&original_config, config_dir_name) {
            let config_from_path = load_config_from_path(config_path);
            let config_by_dir = load_config_by_dir(config_dir_name);

            assert_eq!(original_config, config_from_path);
            assert_eq!(original_config, config_by_dir);
        } else {
            panic!("Error writing config")
        }
    }
}
