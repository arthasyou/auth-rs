use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Deserialize, Serialize, Debug)]
pub struct User {
    pub username: String,
    pub password: String,
    pub uuid: String,
    pub email: Option<String>,
    pub email_verified: bool,
    pub role: String,
    pub disabled: bool,
    pub mfa_enabled: bool,
    pub mfa_secret: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UserInput {
    pub uuid: String,
    pub username: String,
    pub password: String,
    pub role: String,
}

impl UserInput {
    pub fn new(username: &str, password: &str) -> Self {
        Self {
            uuid: uuid::Uuid::new_v4().to_string(),
            username: username.to_string(),
            password: password.to_string(),
            role: "user".to_string(),
        }
    }
}

#[derive(Deserialize, Debug, ToSchema)]
pub struct SignupRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct LoginResponse {
    #[serde(rename = "accessToken")]
    pub access_token: String,
    #[serde(rename = "refreshToken")]
    pub refresh_token: String,
}
