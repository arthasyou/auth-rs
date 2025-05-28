use std::sync::Arc;

use axum::{http::StatusCode, Extension, Json};
use bcrypt::{hash, DEFAULT_COST};
use service_utils_rs::services::{
    http::{
        response::{CommonOk, Empty, ResponseResult, Result},
        CommonError, CommonResponse, IntoCommonResponse,
    },
    jwt::Jwt,
};

use crate::{
    database::auth,
    error::error_code,
    models::auth::{LoginRequest, LoginResponse, SignupRequest, User, UserInput},
};

#[utoipa::path(
    post,
    path = "/signup",
    request_body = SignupRequest,
    responses(
        (status = 200, description = "User registered successfully", body = CommonResponse<Empty>),
        (status = 500, description = "Internal server error", body = CommonError)
    )
)]
pub async fn signup(Json(payload): Json<SignupRequest>) -> ResponseResult<Empty> {
    // 检查用户名是否已存在
    is_username_taken(&payload.username).await?;
    // 哈希密码
    let hashed_password = hash(&payload.password, DEFAULT_COST).map_err(|_e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(error_code::PASSWORD_ERROR.into()),
        )
    })?;

    let user = UserInput::new(&payload.username, &hashed_password);

    auth::create_user(user).await.map_err(|e| {
        eprintln!("Database query error: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(error_code::SERVER_ERROR.into()),
        )
    })?;

    let res = CommonOk::default().to_json();
    Ok(res)
}

#[utoipa::path(
    post,
    path = "/login",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "User registered successfully", body = CommonResponse<LoginResponse>),
        (status = 500, description = "Internal server error", body = CommonError)
    )
)]
pub async fn login(
    Extension(jwt): Extension<Arc<Jwt>>,
    Json(payload): Json<LoginRequest>,
) -> ResponseResult<LoginResponse> {
    let db_user = get_current_user(&payload.username).await?;
    let _ = verify_password(&payload.password, db_user.password.as_ref())?;

    let user_id = db_user.uuid.clone().to_string();
    let (accece, refleash) = jwt.generate_token_pair(user_id.clone()).map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(error_code::SERVER_ERROR.into()),
        )
    })?;
    let data = LoginResponse {
        access_token: accece,
        refresh_token: refleash,
    };

    let res = data.into_common_response().to_json();
    Ok(res)
}

async fn get_current_user(username: &str) -> Result<User> {
    let existing_user = auth::get_user(username).await.map_err(|_| {
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

fn verify_password(password: &str, hash: &str) -> Result<()> {
    match bcrypt::verify(password, hash) {
        Ok(true) => Ok(()),
        Ok(false) | Err(_) => Err((
            StatusCode::UNAUTHORIZED,
            Json(error_code::PASSWORD_ERROR.into()),
        )),
    }
}

async fn is_username_taken(username: &str) -> Result<()> {
    let existing_user = auth::get_user(username).await.map_err(|e| {
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
