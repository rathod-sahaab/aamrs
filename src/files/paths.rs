use std::path::PathBuf;
pub fn get_config_path() -> Option<PathBuf> {
    let home_dir = std::env::var_os("HOME")?;
    let aamrs_config_dir_name = if cfg!(test) { "aamrs-unit-test" } else { "aamrs" };
    let config_path = PathBuf::from(home_dir)
        .join([".config", aamrs_config_dir_name].iter().collect::<PathBuf>());
    Some(config_path)
}
