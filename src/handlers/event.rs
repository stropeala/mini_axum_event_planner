use askama::Template;
use askama_web::WebTemplate;
use axum::extract::{Form, Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Redirect, Response};
use sqlx::SqlitePool;

use crate::authentication::session::CurrentUser;
use crate::error::AppError;
use crate::models::event::{Event, EventDelete, EventNew};

// Display all events

#[derive(Template, WebTemplate)]
#[template(path = "event_planner/event_list.html")]
pub struct EventListPage {
    pub events: Vec<Event>,
}

pub async fn event_list_get(
    State(pool): State<SqlitePool>,
    CurrentUser(user_id): CurrentUser,
) -> Result<EventListPage, AppError> {
    Ok(EventListPage {
        events: Event::event_list(&pool, user_id).await?,
    })
}

// Add new event

#[derive(Template, WebTemplate)]
#[template(path = "event_planner/event_add.html")]
pub struct EventAddPage {}

pub async fn event_add_get(CurrentUser(_user_id): CurrentUser) -> EventAddPage {
    EventAddPage {}
}

pub async fn event_add_post(
    State(pool): State<SqlitePool>,
    CurrentUser(user_id): CurrentUser,
    Form(event_new): Form<EventNew>,
) -> Result<Redirect, AppError> {
    Event::event_add(&pool, &event_new, user_id).await?;

    Ok(Redirect::to("/events"))
}

// Show event description

#[derive(Template, WebTemplate)]
#[template(path = "event_planner/event_description.html")]
pub struct EventDescriptionPage {
    pub event: Event,
}

pub async fn event_by_id_get(
    State(pool): State<SqlitePool>,
    CurrentUser(user_id): CurrentUser,
    Path(event_id): Path<i64>,
) -> Result<Response, AppError> {
    match Event::event_by_id(&pool, event_id, user_id).await? {
        Some(event) => Ok(EventDescriptionPage { event }.into_response()),
        None => Ok((StatusCode::NOT_FOUND, "That event doesn't exist.").into_response()),
    }
}

// Delete event

#[derive(Template, WebTemplate)]
#[template(path = "event_planner/event_delete.html")]
pub struct EventDeletePage {
    pub events: Vec<Event>,
}

pub async fn event_delete_get(
    State(pool): State<SqlitePool>,
    CurrentUser(user_id): CurrentUser,
) -> Result<EventDeletePage, AppError> {
    Ok(EventDeletePage {
        events: Event::event_list(&pool, user_id).await?,
    })
}

pub async fn event_delete_post(
    State(pool): State<SqlitePool>,
    CurrentUser(user_id): CurrentUser,
    Form(event_to_delete): Form<EventDelete>,
) -> Result<Redirect, AppError> {
    Event::event_delete(&pool, event_to_delete.id, user_id).await?;

    Ok(Redirect::to("/events/delete"))
}
