use serde::Deserialize;

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct GetStreamsQuery {
    pub limit: Option<i32>,
    pub cursor: Option<String>,
}
