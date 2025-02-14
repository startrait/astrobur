use crate::database::models::user::User;
use crate::error::BurError;
use chrono::Local;
use jsonwebtoken::errors::Error;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::ops::Add;
use std::time::Duration;
use tracing::event;
use tracing::Level;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    aud: Option<String>,
    exp: usize,
    sub: String,
}

pub fn validate_jwt(token: &str) -> Result<(), Error> {
    let validations = get_validations();
    let _jwt = decode::<Claims>(
        token,
        &DecodingKey::from_secret("hehe".as_ref()),
        &validations,
    )?;

    Ok(())
}

fn get_validations() -> Validation {
    let validation = Validation::default();
    validation
}

pub fn generate_jwt(user: &User, exp: Option<usize>) -> Result<String, BurError> {
    let now = Local::now().add(Duration::from_secs(86400));

    let claims = Claims {
        aud: None,
        exp: exp.unwrap_or(now.timestamp() as usize),
        sub: format!("{:?}", user.id),
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret("hehe".as_ref()),
    )?;
    event!(
        Level::INFO,
        user = user.email,
        exp = &claims.exp,
        token = &token
    );
    Ok(token)
}
