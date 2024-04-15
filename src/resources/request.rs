use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize)]
pub struct Request {
    pub method: String,
    pub url: String,
    pub body: String,
    pub headers: Vec<(String, String)>,
    pub query: Vec<(String, String)>,
    pub params: Vec<(String, String)>,
}
