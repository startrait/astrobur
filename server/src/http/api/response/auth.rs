use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AuthenticatedUserResponse {
    pub access_token: String,
    pub refresh_token: Option<String>,
}
