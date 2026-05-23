use serde::Serialize;

#[derive(Serialize)]
pub struct LoginResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub expires_in: u32,
}

#[derive(Serialize)]
pub struct RegisterResponse {
    pub id: String,
    pub message: String,
}

#[derive(Serialize)]
pub struct StreamCatalogItem {
    pub stream_id: String,
    pub title: String,
    pub category: String,
    pub viewers_count: i32,
}

#[derive(Serialize)]
pub struct StreamCatalogResponse {
    pub data: Vec<StreamCatalogItem>,
    pub next_cursor: String,
}
