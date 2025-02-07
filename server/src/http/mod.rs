pub mod analytics;
pub mod authenticator;
pub mod error;
pub mod generator;
pub mod http_server;
pub mod middleware;
pub mod request_payload;
pub mod response;

use serde::Serialize;

pub struct ResponseMessage<T: Serialize> {
    message: String,
    data: Option<T>,
}
