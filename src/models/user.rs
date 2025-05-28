use serde::Serialize;
use utoipa::ToSchema;

use super::User;

#[derive(Debug, Serialize, ToSchema)]
pub struct MeResponse {
    pub username: String,
    pub uuid: String,
    pub email: Option<String>,
    pub role: String,
}

impl MeResponse {
    pub fn from(user: User) -> Self {
        Self {
            username: user.username,
            uuid: user.uuid.to_string(),
            email: user.email,
            role: user.role,
        }
    }
}
