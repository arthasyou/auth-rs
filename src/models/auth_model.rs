use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct User {
    pub id: u32,
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Debug)]
pub struct UserInput {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Debug)]
pub struct SignupRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginRespon {
    #[serde(rename = "accessToken")]
    pub access_token: String,
    pub refresh: String,
}
