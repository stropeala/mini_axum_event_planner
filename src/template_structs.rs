use askama::Template;
use askama_web::WebTemplate;

#[derive(Template, WebTemplate)]
#[template(path = "homepage.html")]
pub struct Homepage {}

#[derive(Template, WebTemplate)]
#[template(path = "event_index.html")]
pub struct EventIndex {}

#[derive(Template, WebTemplate)]
#[template(path = "event_add.html")]
pub struct EventAdd {}

#[derive(Template, WebTemplate)]
#[template(path = "event_description.html")]
pub struct EventDescription {}

#[derive(Template, WebTemplate)]
#[template(path = "event_delete.html")]
pub struct EventDelete {}

#[derive(Template, WebTemplate)]
#[template(path = "not_found.html")]
pub struct NotFound {}
