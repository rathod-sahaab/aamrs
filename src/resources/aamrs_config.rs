use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum Theme {
    Dark,
    Light,
    System,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct AamrsConfig {
    pub theme_mode: Theme,
}
