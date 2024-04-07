use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::{files::paths::get_config_path, traits::file_path::FilePath};

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
}

impl FilePath for AamrsState {
    fn filepath() -> std::path::PathBuf {
        if let Some(config_dir) = get_config_path() {
            return config_dir.join("state.json");
        }
        PathBuf::from("./state.json")
    }
}
