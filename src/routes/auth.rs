use axum::{routing::post, Router};
use utoipa::OpenApi;

use crate::handlers::auth::{login, refresh_token, signup};

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::handlers::auth::signup,
        crate::handlers::auth::login,
        crate::handlers::auth::refresh_token
    ),
    tags(
        (name = "Auth", description = "登入相关接口")
    ),
)]
pub struct AuthApi;

pub fn auth_routes() -> Router {
    Router::new()
        .route("/signup", post(signup))
        .route("/login", post(login))
        .route("/refresh_token", post(refresh_token))
}
