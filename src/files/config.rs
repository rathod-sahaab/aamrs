use std::path::PathBuf;

pub fn get_config_path(config_path: Vec<&str>) -> Option<PathBuf> {
    let home_dir = std::env::var_os("HOME")?;

    let config_path = PathBuf::from(home_dir).join(config_path.iter().collect::<PathBuf>());

    Some(config_path)
}
