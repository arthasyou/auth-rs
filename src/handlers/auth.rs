use std::sync::Arc;

use axum::{http::StatusCode, Extension, Json};
use bcrypt::{hash, DEFAULT_COST};
use service_utils_rs::services::{
    http::{
        response::{CommonOk, Empty, ResponseResult},
        CommonError, CommonResponse, IntoCommonResponse,
    },
    jwt::Jwt,
};
use validator::Validate;

use super::{get_current_user, is_username_taken, verify_password};
use crate::{
    database::user,
    error::error_code,
    models::{
        auth::{
            LoginRequest, LoginResponse, RefreshTokenRequest, RefreshTokenResponse, SignupRequest,
        },
        UserInput,
    },
};

#[utoipa::path(
    post,
    path = "/signup",
    request_body = SignupRequest,
    responses(
        (status = 200, description = "User registered successfully", body = CommonResponse<Empty>),
        (status = 500, description = "Internal server error", body = CommonError)
    ),
    description = "用户注册接口，创建新用户",
    tag = "Auth",
)]
pub async fn signup(Json(payload): Json<SignupRequest>) -> ResponseResult<Empty> {
    if let Err(validation_errors) = payload.validate() {
        return Err((
            StatusCode::BAD_REQUEST,
            CommonError {
                code: error_code::INVALID_PARAMS.0,
                message: format!("{}", validation_errors),
            }
            .to_json(),
        ));
    }

    is_username_taken(&payload.username).await?;

    let hashed_password = hash(&payload.password, DEFAULT_COST).map_err(|_e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(error_code::PASSWORD_ERROR.into()),
        )
    })?;

    let user = UserInput::new(&payload.username, &hashed_password);

    user::create_user(user).await.map_err(|e| {
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
    ),
    description = "用户登录接口，验证用户凭据并返回 JWT 令牌",
    tag = "Auth",
)]
pub async fn login(
    Extension(jwt): Extension<Arc<Jwt>>,
    Json(payload): Json<LoginRequest>,
) -> ResponseResult<LoginResponse> {
    if let Err(validation_errors) = payload.validate() {
        return Err((
            StatusCode::BAD_REQUEST,
            CommonError {
                code: error_code::INVALID_PARAMS.0,
                message: validation_errors.to_string(),
            }
            .to_json(),
        ));
    }

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

#[utoipa::path(
    post,
    path = "/refresh_token",
    request_body = RefreshTokenRequest,
    responses(
        (status = 200, description = "Token refreshed successfully", body = CommonResponse<RefreshTokenResponse>),
        (status = 500, description = "Internal server error", body = CommonError)
    ),
    description = "使用 refresh_token 获取新的 access_token",
    tag = "Auth",
)]
pub async fn refresh_token(
    Extension(jwt): Extension<Arc<Jwt>>,
    Json(payload): Json<RefreshTokenRequest>,
) -> ResponseResult<RefreshTokenResponse> {
    let access = jwt
        .refresh_access_token(&payload.refresh_token)
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(error_code::INVALID_TOKEN.into()),
            )
        })?;
    let data = RefreshTokenResponse {
        access_token: access,
    };

    let res = data.into_common_response().to_json();
    Ok(res)
}
