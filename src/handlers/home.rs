use axum::{http::StatusCode, response::IntoResponse};

use crate::template_structs::{Homepage, NotFound};

pub async fn homepage() -> Homepage {
    Homepage {}
}

pub async fn not_found() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, NotFound {})
}
