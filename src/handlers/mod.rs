use axum::{http::StatusCode, response::IntoResponse};

use crate::template_structs::{HomepageTemplate, NotFoundTemplate};

pub async fn homepage() -> HomepageTemplate {
    HomepageTemplate {}
}

pub async fn not_found() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, NotFoundTemplate {})
}
