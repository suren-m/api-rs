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

#[derive(Serialize)]
pub struct UsersResponse {
    pub users: Vec<UserResponse>,
}

#[derive(Serialize)]
pub struct UserResponse {
    pub firstname: String,
    pub lastname: String,
    pub email: String,
}

#[derive(Deserialize)]
pub struct UpdateRequest {
    pub firstname: String,
    pub lastname: String,
}
