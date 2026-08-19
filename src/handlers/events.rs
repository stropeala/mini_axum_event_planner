use anyhow::Context;
use askama::Template;
use askama_web::WebTemplate;
use axum::extract::{Form, Path, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Redirect, Response};
use serde::Deserialize;
use sqlx::SqlitePool;

use crate::error::AppError;

#[derive(sqlx::FromRow)]
pub struct Events {
    pub id: i64,
    pub name: String,
    pub date: String,
    pub time: String,
    pub description: String,
}

#[derive(Deserialize)]
pub struct NewEvent {
    pub name: String,
    pub date: String,
    pub time: String,
    pub description: String,
}

#[derive(Deserialize)]
pub struct DeleteEvent {
    pub id: i64,
}

impl Events {
    pub async fn display_events(pool: &SqlitePool) -> anyhow::Result<Vec<Events>> {
        sqlx::query_as::<_, Events>(
            "SELECT id, name, date, time, description FROM EventPlanner ORDER BY date, time",
        )
        .fetch_all(pool)
        .await
        .context("Failed getting events from table!")
    }

    pub async fn event_desc(pool: &SqlitePool, event_id: i64) -> anyhow::Result<Option<Events>> {
        sqlx::query_as::<_, Events>(
            "SELECT id, name, date, time, description FROM EventPlanner WHERE id = ?",
        )
        .bind(event_id)
        .fetch_optional(pool)
        .await
        .context("Failed getting event from table!")
    }

    pub async fn add_event(pool: &SqlitePool, event: &NewEvent) -> anyhow::Result<i64> {
        let id = sqlx::query(
            "INSERT INTO EventPlanner (name, date, time, description) VALUES (?, ?, ?, ?)",
        )
        .bind(&event.name)
        .bind(&event.date)
        .bind(&event.time)
        .bind(&event.description)
        .execute(pool)
        .await
        .context("Failed adding event to table!")?
        .last_insert_rowid();
        Ok(id)
    }

    pub async fn delete_event(pool: &SqlitePool, event_id: i64) -> anyhow::Result<u64> {
        let rows = sqlx::query("DELETE FROM EventPlanner WHERE id = ?")
            .bind(event_id)
            .execute(pool)
            .await
            .context("Failed deleting event from table!")?
            .rows_affected();
        Ok(rows)
    }
}

#[derive(Template, WebTemplate)]
#[template(path = "event_index.html")]
pub struct EventPlannerIndex {
    pub events: Vec<Events>,
}

pub async fn get_event_planner_index(
    State(pool): State<SqlitePool>,
) -> Result<EventPlannerIndex, AppError> {
    Ok(EventPlannerIndex {
        events: Events::display_events(&pool).await?,
    })
}

#[derive(Template, WebTemplate)]
#[template(path = "event_add.html")]
pub struct EventPlannerAdd {}

pub async fn get_event_planner_add() -> EventPlannerAdd {
    EventPlannerAdd {}
}

pub async fn post_event_planner_add(
    State(pool): State<SqlitePool>,
    Form(new_event): Form<NewEvent>,
) -> Result<Redirect, AppError> {
    Events::add_event(&pool, &new_event).await?;
    Ok(Redirect::to("/events"))
}

#[derive(Template, WebTemplate)]
#[template(path = "event_description.html")]
pub struct EventPlannerDescription {
    pub event: Events,
}

pub async fn get_event_planner_description(
    State(pool): State<SqlitePool>,
    Path(event_id): Path<i64>,
) -> Result<Response, AppError> {
    match Events::event_desc(&pool, event_id).await? {
        Some(event) => Ok(EventPlannerDescription { event }.into_response()),
        None => Ok((StatusCode::NOT_FOUND, "That event doesn't exist.").into_response()),
    }
}

#[derive(Template, WebTemplate)]
#[template(path = "event_delete.html")]
pub struct EventPlannerDelete {
    pub events: Vec<Events>,
}

pub async fn get_event_delete(
    State(pool): State<SqlitePool>,
) -> Result<EventPlannerDelete, AppError> {
    Ok(EventPlannerDelete {
        events: Events::display_events(&pool).await?,
    })
}

pub async fn post_event_delete(
    State(pool): State<SqlitePool>,
    Form(to_delete): Form<DeleteEvent>,
) -> Result<Redirect, AppError> {
    Events::delete_event(&pool, to_delete.id).await?;
    Ok(Redirect::to("/events/delete"))
}
