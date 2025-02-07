use crate::database::models::User;
use crate::http::error::BurError;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use jsonwebtoken::errors::Error;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::ops::Add;
use std::time::{Duration};
use chrono::{DateTime, Local};
use tracing::{event};
use tracing::Level;

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    aud: Option<String>,
    exp: usize,
    sub: String,
}

pub fn validate_jwt(token: &str) -> Result<(), Error> {
    let validations = get_validations();
    let jwt = decode::<Claims>(
        token,
        &DecodingKey::from_secret("hehe".as_ref()),
        &validations,
    )?;

    Ok(())
}

fn get_validations() -> Validation {
    let mut validation = Validation::default();
    validation
}

pub fn generate_jwt(user: &User, exp: Option<usize>) -> Result<String, BurError> {
    let now = Local::now().add(Duration::from_secs(86400));

    let claims = Claims {
        aud: None,
        exp: now.timestamp() as usize,
        sub: format!("{}", user.id),
    };


    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret("hehe".as_ref()),
    )?;
    event!(Level::INFO, user = user.email, exp = &claims.exp, token = &token);
    Ok(token)
}
