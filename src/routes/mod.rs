mod auth;
mod user;

use std::sync::Arc;

use auth::{auth_routes, AuthApi};
use axum::{Extension, Router};
use service_utils_rs::services::{http::middleware::cors::create_cors, jwt::Jwt};
use user::{user_routes, UserApi};
use utoipa::{
    openapi::security::{ApiKey, SecurityScheme},
    OpenApi,
};
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    nest(
        (path = "/auth", api = AuthApi),
        (path = "/user", api = UserApi)
    )
)]
struct ApiDoc;

pub fn create_routes(jwt: Arc<Jwt>) -> Router {
    let cors = create_cors();

    let mut doc = ApiDoc::openapi();
    doc.components
        .get_or_insert_with(Default::default)
        .add_security_scheme(
            "Bearer",
            SecurityScheme::ApiKey(ApiKey::Header(
                utoipa::openapi::security::ApiKeyValue::with_description(
                    "Authorization",
                    "请输入格式：Bearer <token>",
                ),
            )),
        );
    Router::new()
        .nest("/user", user_routes())
        .nest("/auth", auth_routes())
        .layer(Extension(jwt))
        .layer(cors)
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", doc))
}
