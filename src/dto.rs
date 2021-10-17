use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize)]
pub struct SignupRequest {
    pub firstname: String,
    pub lastname: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
}
