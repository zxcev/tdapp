use axum::http::StatusCode;
use bcrypt::{bcrypt, hash, verify};

use super::app_error::AppError;
const COST: u32 = 12;

pub fn hash_password(password: &str) -> Result<String, AppError> {
    hash(password, COST).map_err(|err| {
        eprintln!("Error hashing password: {err:?}");
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error securing password")
    })
}

pub fn verify_password(password: &str, hash: &str) -> Result<bool, AppError> {
    verify(password, hash).map_err(|err| {
        eprintln!("Error verifying pasword: {err:?}");
        AppError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "The was a problem verifying your password",
        )
    })
}
