use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AamrsState {
    pub active_environment: String,
}
