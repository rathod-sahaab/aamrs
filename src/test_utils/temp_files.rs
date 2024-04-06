use std::{env, path::PathBuf};

pub fn test_file_path(file_name: &str) -> PathBuf {
    let root_dir = &env::var("CARGO_MANIFEST_DIR").expect("$CARGO_MANIFEST_DIR");
    PathBuf::from(root_dir).join(["temp", "fixtures", file_name].iter().collect::<PathBuf>())
}
