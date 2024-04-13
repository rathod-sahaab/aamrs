use std::path::PathBuf;

pub fn checkNewProjectFolder(folder: PathBuf) -> Result<(), &'static str> {
    if !folder.is_dir() {
        return Err("Folder is not a directory");
    }

    if let Ok(dir_contents) = folder.read_dir() {
        if dir_contents.peekable().peek().is_some() {
            return Err("Directory not empty can't create a project");
        }
    } else {
        return Err("Error reading directory");
    }

    Ok(())
}
