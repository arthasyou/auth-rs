use axum::{http::StatusCode, Extension, Json};
use service_utils_rs::services::http::{
    middleware::auth_mw::UserId,
    response::{CommonOk, Empty, ResponseResult},
    CommonError, CommonResponse, IntoCommonResponse,
};

use crate::{database::user, error::error_code, models::user::MeResponse};

#[utoipa::path(
    post,
    path = "/logout",
    responses(
        (status = 200, description = "Logout successful", body = CommonResponse<Empty>),
        (status = 500, description = "Internal server error", body = CommonError)
    ),
    description = "用户登出接口",
    tag = "User",
    security(("Bearer" = [])),
)]
pub async fn logout(Extension(UserId(_user_id)): Extension<UserId>) -> ResponseResult<Empty> {
    let res = CommonOk::default().to_json();
    Ok(res)
}

#[utoipa::path(
    get,
    path = "/me",
    responses(
        (status = 200, description = "Current user information", body = CommonResponse<MeResponse>),
        (status = 500, description = "Internal server error", body = CommonError)
    ),
    description = "获取当前用户信息接口",
    tag = "User",
    security(("Bearer" = [])),
)]
pub async fn me(Extension(UserId(user_id)): Extension<UserId>) -> ResponseResult<MeResponse> {
    let user = user::get_user_by_id(&user_id).await.map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(error_code::SERVER_ERROR.into()),
        )
    })?;

    match user {
        Some(user) => {
            let me = MeResponse::from(user);
            let res = me.into_common_response().to_json();
            Ok(res)
        }
        None => Err((
            StatusCode::BAD_REQUEST,
            Json(error_code::USER_NOT_EXIST.into()),
        )),
    }
}
