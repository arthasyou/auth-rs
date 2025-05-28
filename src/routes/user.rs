use axum::{
    middleware::from_fn,
    routing::{get, post},
    Router,
};
use service_utils_rs::services::http::middleware::auth_mw::auth;
use utoipa::OpenApi;

use crate::handlers::user::{logout, me};

#[derive(OpenApi)]
#[openapi(
    paths(crate::handlers::user::logout, crate::handlers::user::me),
    tags(
        (name = "User", description = "User Management APIs")
    ),
    // 不知道为什么这里的 security 会影响到全局
    // security(
    //     ("Bearer" = [])
    // ),
)]
pub struct UserApi;

pub fn user_routes() -> Router {
    Router::new()
        .route("/logout", post(logout))
        .route("/me", get(me))
        .route_layer(from_fn(auth))
}
