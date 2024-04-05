use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AamrsProject {
    name: String,
    location: String,
}

#[derive(Serialize, Deserialize)]
pub struct AamrsConfig {
    pub projects: Vec<AamrsProject>,
}
