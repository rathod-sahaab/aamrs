use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use crate::{
    files::{
        paths::get_config_path,
        resource_files::{parse_resource_from_path, save_resource_to_path},
    },
    traits::file_path::FilePath,
};
#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct AamrsProject {
    pub name: String,
    pub location: String,
}
#[derive(Serialize, Deserialize, Default, PartialEq, Eq, Debug)]
pub struct AamrsState {
    pub projects: Vec<AamrsProject>,
}
impl AamrsState {
    pub fn add_project(&mut self, aamrs_project: AamrsProject) {
        self.projects.push(aamrs_project)
    }
    pub fn load_state() -> AamrsState {
        parse_resource_from_path(AamrsState::filepath())
    }
    pub fn save_state(&self) -> Option<()> {
        save_resource_to_path(self, AamrsState::filepath())
    }
}
impl FilePath for AamrsState {
    fn filepath() -> std::path::PathBuf {
        if let Some(config_dir) = get_config_path() {
            return config_dir.join("state.json");
        }
        PathBuf::from("./state.json")
    }
}
#[cfg(test)]
mod tests {
    use crate::resources::aamrs_state::{AamrsProject, AamrsState};
    #[test]
    fn test_state_retrieval() {
        let original_state = AamrsState {
            projects: vec![
                AamrsProject {
                    name: String::from("monorepo"),
                    location: String::from("/home/abhay/projects/monorepo/.aamrs/"),
                },
            ],
        };
        if let Some(()) = original_state.save_state() {
            let retrieved_state = AamrsState::load_state();
            assert_eq!(retrieved_state, original_state);
        } else {
            panic!("Error loading state")
        }
    }
}
