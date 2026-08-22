use askama::Template;
use askama_web::WebTemplate;
use axum::{http::StatusCode, response::IntoResponse};

// Home page

#[derive(Template, WebTemplate)]
#[template(path = "home.html")]
pub struct HomePage {}

pub async fn home_get() -> HomePage {
    HomePage {}
}

// Not found error page

#[derive(Template, WebTemplate)]
#[template(path = "not_found.html")]
pub struct NotFoundPage {}

pub async fn not_found_get() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, NotFoundPage {})
}
