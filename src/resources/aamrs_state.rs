
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug)]
pub struct AamrsProject {
    pub name: String,
    pub location: String,
}

#[derive(Serialize, Deserialize, Default, PartialEq, Eq, Debug)]
pub struct AamrsState {
    pub projects: Vec<AamrsProject>,
}
