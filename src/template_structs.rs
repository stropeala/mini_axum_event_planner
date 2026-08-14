use askama::Template;
use askama_web::WebTemplate;

#[derive(Template, WebTemplate)]
#[template(path = "homepage.html")]
pub struct HomepageTemplate {}

#[derive(Template, WebTemplate)]
#[template(path = "not_found.html")]
pub struct NotFoundTemplate {}
