mod auth_route;

use auth_route::routes_auth;
use axum::{Extension, Router};
use service_utils_rs::services::{http::middleware::cors::create_cors, jwt::Jwt};
use std::sync::Arc;

pub fn create_routes(jwt: Arc<Jwt>) -> Router {
    let cors = create_cors();

    Router::new()
        // .merge(routes_manage())
        .nest("/auth", routes_auth())
        .layer(Extension(jwt))
        .layer(cors)
}
