use serde::{Deserialize, Serialize};

use super::{request::Request, response::Response};

#[derive(Serialize, Deserialize)]
pub struct RequestExample {
    request: Request,
    response: Response,
}
