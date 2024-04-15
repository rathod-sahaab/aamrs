use std::collections::BTreeMap;
use serde::{Deserialize, Serialize};
use crate::traits::file_name::SingletonFileName;
use super::environment::Environment;
#[derive(Serialize, Deserialize, Default)]
pub struct ProjectState {
    pub active_environment: String,
    pub environments: BTreeMap<String, Environment>,
    pub default_env: Environment,
}
impl ProjectState {
    pub fn set_active_environment(&mut self, env_name: String) -> Result<(), String> {
        if !self.environments.contains_key(&env_name) {
            return Err(format!("{} environment not found.", &env_name));
        }
        self.active_environment = env_name;
        Ok(())
    }
    pub fn create_environment(&mut self, env_name: String) {
        if self.environments.contains_key(&env_name) {
            return;
        }
        self.environments.insert(env_name, self.default_env.clone());
    }
    pub fn get_env_values(&self, env_name: String) -> &BTreeMap<String, String> {
        &self.environments.get(&env_name).unwrap_or(&self.default_env).values
    }
}
impl SingletonFileName for ProjectState {
    fn filename() -> String {
        "project.state.rs".to_string()
    }
}
