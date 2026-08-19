use askama::Template;
use askama_web::WebTemplate;
use axum::{http::StatusCode, response::IntoResponse};

#[derive(Template, WebTemplate)]
#[template(path = "homepage.html")]
pub struct Homepage {}

pub async fn homepage() -> Homepage {
    Homepage {}
}

#[derive(Template, WebTemplate)]
#[template(path = "not_found.html")]
pub struct NotFound {}

pub async fn not_found() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, NotFound {})
}
