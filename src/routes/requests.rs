use serde::Deserialize;

#[derive(Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct RegistrationRequest {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct ChangeRequest {
    pub field: String,
    pub new_value: String,
}
