pub mod auth;
pub mod user;

use axum::{http::StatusCode, Json};
use service_utils_rs::services::http::response::Result;

use crate::{database::user::get_user_by_name, error::error_code, models::User};

pub async fn get_current_user(username: &str) -> Result<User> {
    let existing_user = get_user_by_name(username).await.map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(error_code::SERVER_ERROR.into()),
        )
    })?;
    match existing_user {
        Some(user) => Ok(user),
        None => Err((
            StatusCode::BAD_REQUEST,
            Json(error_code::USER_NOT_EXIST.into()),
        )),
    }
}

pub fn verify_password(password: &str, hash: &str) -> Result<()> {
    match bcrypt::verify(password, hash) {
        Ok(true) => Ok(()),
        Ok(false) | Err(_) => Err((
            StatusCode::UNAUTHORIZED,
            Json(error_code::PASSWORD_ERROR.into()),
        )),
    }
}

pub async fn is_username_taken(username: &str) -> Result<()> {
    let existing_user = get_user_by_name(username).await.map_err(|e| {
        eprintln!("Database query error: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(error_code::SERVER_ERROR.into()),
        )
    })?;
    println!("existing: {:?}", existing_user);
    match existing_user {
        // Some(_) => Ok(()),
        Some(_) => Err((StatusCode::BAD_REQUEST, Json(error_code::USER_EXIST.into()))),
        None => Ok(()),
    }
}
