mod authentication;
mod db;
mod error;
mod handlers;

use axum::{Router, routing::get};
use sqlx::SqlitePool;
use tokio::net::TcpListener;

use db::{create_table, sqlite_pool};
use handlers::{
    events::{
        get_event_delete, get_event_planner_add, get_event_planner_description,
        get_event_planner_index, post_event_delete, post_event_planner_add,
    },
    home::{homepage, not_found},
};

const EVENTS: &str = "EVENT_PLANNER_SCHEMA";
const USER: &str = "USER_SCHEMA";

pub async fn event_planner_pool() -> anyhow::Result<SqlitePool> {
    let pool = sqlite_pool().await?;
    create_table(&pool, EVENTS).await?;
    create_table(&pool, USER).await?;
    Ok(pool)
}

pub fn events_router() -> Router<SqlitePool> {
    Router::new()
        .route("/", get(get_event_planner_index))
        .route(
            "/add",
            get(get_event_planner_add).post(post_event_planner_add),
        )
        .route("/delete", get(get_event_delete).post(post_event_delete))
        .route("/{id}", get(get_event_planner_description))
}

pub fn event_planner_router(pool: SqlitePool) -> Router {
    Router::new()
        .route("/", get(homepage))
        .nest("/events", events_router())
        .fallback(not_found)
        .with_state(pool)
}

pub async fn event_planner_listener() -> TcpListener {
    println!("Listening on http://127.0.0.1:3000");

    TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("port 3000 is already in use")
}
