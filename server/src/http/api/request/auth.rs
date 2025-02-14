use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TokenValidationRequest {
    pub token: String,
}
