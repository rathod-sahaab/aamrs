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

#[cfg(test)]
mod tests {
    use std::{fs, path::{Path, PathBuf}};

    use anyhow::Result;
    use tempfile::tempdir;

    use super::checkNewProjectDirectory;

    #[test]
    fn empty_dir_is_valid()-> Result<()> {
        let empty_dir = tempdir()?;

        let check_result = checkNewProjectDirectory(empty_dir.path());
        
        assert!(check_result.is_ok());

        Ok(())
    }

    #[test]
    fn non_empty_dir_is_invalid()-> Result<()> {
        let dir = tempdir()?;

        let file_path = dir.path().join("my-temporary-note.txt");
        fs::write(file_path, "Brian was here. Briefly.")?;

        let check_result = checkNewProjectDirectory(dir.path());
        
        assert!(check_result.is_err());

        Ok(())
    }

    #[test]
    fn non_existant_dir_is_invalid() ->Result<()> {
        let dir = tempdir()?;

        let dir_path = PathBuf::from(dir.path());

        let _ = dir.close();

        let check_result = checkNewProjectDirectory(&dir_path);
        
        assert!(check_result.is_err());

        Ok(())
    }
}
