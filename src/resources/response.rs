use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Response {
    pub status: u32,
    pub headers: Vec<(String, String)>,
    pub body: String,
}
