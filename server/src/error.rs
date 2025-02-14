use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum BurError {
    #[error("something went wrong")]
    DefaultError,
    #[error("app state failed ")]
    AppState(ErrorResponse),
    #[error("non-existent error")]
    DBError(ErrorResponse),
    #[error("sql error: {0}")]
    SQLxError(#[from] sqlx::Error),
    #[error("bcrypt error: {0}")]
    BcryptError(#[from] bcrypt::BcryptError),
    #[error("custom-error")]
    CustomError(ErrorResponse),
    #[error("chrono-error: {0}")]
    ChronoError(#[from] chrono::ParseError),
    #[error("serde_json-error: {0}")]
    SerdeJsonError(#[from] serde_json::Error),
    #[error("jsonwebtoken error: {0}")]
    JsonWebTokenError(#[from] jsonwebtoken::errors::Error),
    #[error("no db connection recieved from crate::app::state::DB_CONNECTION")]
    NoDbConnection,
    #[error("requested entity not found")]
    EntityNotFound,
    #[error("int parse error")]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("Form body rejected: {0}")]
    FormRejection(#[from] axum::extract::rejection::FormRejection),
    #[error("Json body rejected: {0}")]
    JsonRejection(#[from] axum::extract::rejection::JsonRejection),
    #[error("To str error: {0}")]
    ToStrError(#[from] axum::http::header::ToStrError),
    #[error("Axum error: {0}")]
    AxumError(#[from] axum::Error),
    #[error("Axum Http Error: {0}")]
    AxumHttpError(#[from] axum::http::Error),
    #[error("Invalid credential")]
    InvalidCredential,
}

#[derive(Debug, Serialize, Clone)]
pub struct ErrorResponse {
    pub error_code: u32,
    pub reason: String,
}
