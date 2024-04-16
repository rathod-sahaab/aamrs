use std::path::Path;
pub fn checkNewProjectDirectory(directory: &Path) -> Result<(), String> {
    if !directory.is_dir() {
        return Err("Not a directory.".to_string());
    }
    if let Ok(dir_contents) = directory.read_dir() {
        if dir_contents.peekable().peek().is_some() {
            return Err("Non empty directory.".to_string());
        }
    } else {
        return Err("Error reading directory.".to_string());
    }
    Ok(())
}
