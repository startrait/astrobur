use crate::database::models::user::User;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct UserCreationRequest {
    pub name: String,
    pub email: String,
    pub password: String,
}

impl Into<User> for UserCreationRequest {
    fn into(self) -> User {
        User {
            id: None,
            name: self.name,
            email: self.email,
            password: self.password,
        }
    }
}
