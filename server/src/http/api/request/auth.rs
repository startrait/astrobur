use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TokenValidationRequest {
    pub token: String,
}

#[derive(Serialize, Deserialize)]
pub struct UserLoginRequest {
    pub email: String,
    pub password: String,
}
