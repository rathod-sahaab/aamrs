use std::{fs, path::PathBuf};
use serde::{Deserialize, Serialize};
pub fn parse_resource_from_path<T: Default>(resource_path: PathBuf) -> T
where
    for<'de> T: Deserialize<'de>,
{
    if let Ok(content) = fs::read(resource_path) {
        let file_string = String::from_utf8_lossy(&content);
        let resource: T = serde_json::from_str(&file_string).unwrap_or(T::default());
        return resource;
    }
    T::default()
}
pub fn save_resource_to_path<T>(resource: &T, resource_path: PathBuf) -> Option<()>
where
    T: Serialize,
{
    let resource_dir = resource_path.parent()?;
    fs::create_dir_all(resource_dir).ok()?;
    let json_resource = serde_json::to_string_pretty(&resource).ok()?;
    fs::write(&resource_path, json_resource).ok()?;
    Some(())
}
#[cfg(test)]
mod tests {
    use crate::{
        files::resource_files::{parse_resource_from_path, save_resource_to_path},
        resources::aamrs_state::{AamrsProject, AamrsState},
        test_utils::temp_files::test_file_path,
    };
    #[test]
    fn test_load_state_should_return_default_when_not_found() {
        let state_path = test_file_path("empty-state.json");
        let state: AamrsState = parse_resource_from_path(state_path);
        assert_eq!(state, AamrsState::default())
    }
    #[test]
    fn test_state_retrieval() {
        let original_state = AamrsState {
            projects: vec![
                AamrsProject {
                    name: String::from("aamrs"),
                    location: String::from("/home/abhay/personal/rust/aamrs"),
                },
            ],
        };
        let state_path = test_file_path("state.json");
        save_resource_to_path(&original_state, state_path.clone());
        let retrieved_state: AamrsState = parse_resource_from_path(state_path.clone());
        assert_eq!(retrieved_state, original_state)
    }
}
