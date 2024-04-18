use std::{fs, path::Path};
pub fn check_new_project_directory(directory: &Path) -> Result<(), String> {
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
pub fn create_empty_directory(path: &Path, dir_name: String) -> Result<(), String> {
    if !path.is_dir() {
        return Err("Not a directory.".to_string());
    }
    if fs::create_dir(path.join(dir_name)).is_err() {
        return Err("Failed to create directory.".to_string());
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{check_new_project_directory, create_empty_directory};
    use anyhow::Result;
    use std::{fs, path::PathBuf};
    use tempfile::tempdir;

    #[test]
    fn empty_dir_is_valid() -> Result<()> {
        let empty_dir = tempdir()?;
        let check_result = check_new_project_directory(empty_dir.path());
        assert!(check_result.is_ok());
        Ok(())
    }

    #[test]
    fn non_empty_dir_is_invalid() -> Result<()> {
        let dir = tempdir()?;
        let file_path = dir.path().join("my-temporary-note.txt");
        fs::write(file_path, "Brian was here. Briefly.")?;
        let check_result = check_new_project_directory(dir.path());
        assert!(check_result.is_err());
        Ok(())
    }

    #[test]
    fn non_existant_dir_is_invalid() -> Result<()> {
        let dir = tempdir()?;
        let dir_path = PathBuf::from(dir.path());
        let _ = dir.close();
        let check_result = check_new_project_directory(&dir_path);
        assert!(check_result.is_err());
        Ok(())
    }

    #[test]
    fn create_dir_in_empty_dir() -> Result<()> {
        let dir = tempdir()?;
        let dir_path = PathBuf::from(dir.path());
        let sub_dir = create_empty_directory(&dir_path, "temporary".to_string());
        assert!(sub_dir.is_ok());
        let read_dir = fs::read_dir(dir_path.join("temporary"));
        assert!(read_dir.is_ok());
        Ok(())
    }

    #[test]
    fn create_dir_in_non_existant_dir() -> Result<()> {
        let dir = tempdir()?;
        let dir_path = PathBuf::from(dir.path());
        let _ = dir.close();
        let sub_dir = create_empty_directory(&dir_path, "temporary".to_string());
        assert!(sub_dir.is_err());
        Ok(())
    }
}
