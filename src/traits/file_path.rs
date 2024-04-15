use std::path::PathBuf;
pub trait FilePath {
    fn filepath() -> PathBuf;
}
