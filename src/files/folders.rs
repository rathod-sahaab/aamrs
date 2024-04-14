use std::path::PathBuf;

pub fn checkNewProjectFolder(folder: PathBuf) -> Result<(), &'static str> {
    if !folder.is_dir() {
        return Err("Not a directory.");
    }

    if let Ok(dir_contents) = folder.read_dir() {
        if dir_contents.peekable().peek().is_some() {
            return Err("Non empty directory.");
        }
    } else {
        return Err("Error reading directory.");
    }

    Ok(())
}
