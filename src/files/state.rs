use crate::{files::resource_files::parse_resource_from_path, traits::file_path::FilePath};

use crate::resources::aamrs_state::AamrsState;

use super::resource_files::save_resource_to_path;

pub fn load_state() -> AamrsState {
    parse_resource_from_path(AamrsState::filepath())
}

pub fn save_state(state: &AamrsState) -> Option<()> {
    save_resource_to_path(state, AamrsState::filepath())
}

#[cfg(test)]
mod tests {
    use crate::{
        files::state::{load_state, save_state},
        resources::aamrs_state::{AamrsProject, AamrsState},
    };

    #[test]
    fn test_state_retrieval() {
        let original_state = AamrsState {
            projects: vec![AamrsProject {
                name: String::from("monorepo"),
                location: String::from("/home/abhay/projects/monorepo/.aamrs/"),
            }],
        };

        if let Some(()) = save_state(&original_state) {
            let retrieved_state = load_state();
            assert_eq!(retrieved_state, original_state);
        } else {
            panic!("Error loading state")
        }
    }
}
