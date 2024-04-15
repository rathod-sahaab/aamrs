use std::{fs, path::PathBuf};
use crate::{
    resources::project_state::ProjectState, traits::file_name::SingletonFileName,
};
use super::folders::checkNewProjectDirectory;
pub fn createProjectInDirectory(directory: PathBuf) -> Result<(), String> {
    checkNewProjectDirectory(&directory)?;
    if let Ok(project_json) = serde_json::to_string_pretty(&ProjectState::default()) {
        let path = directory.join(ProjectState::filename());
        if fs::write(&path, project_json).is_err() {
            return Err(
                format!("Error writing to file: {}", path.to_str().unwrap_or(""))
                    .to_string(),
            );
        }
    } else {
        return Err("Error encoding json".to_string());
    }
    Ok(())
}
