mod authentication;
mod db;
mod error;
mod handlers;
mod models;

use axum::routing::post;
use axum::{Router, routing::get};
use sqlx::SqlitePool;
use time::Duration;
use tokio::net::TcpListener;
use tower_sessions::{Expiry, SessionManagerLayer};
use tower_sessions_sqlx_store::SqliteStore;

use db::{create_table, sql_pool};
use handlers::event::{
    event_add_get, event_add_post, event_by_id_get, event_delete_get, event_delete_post,
    event_list_get,
};
use handlers::home::{home_get, not_found_get};
use handlers::user::{
    user_delete_post, user_login_get, user_login_post, user_profile_get, user_register_get,
    user_register_post,
};

use crate::handlers::user::user_logout_post;

const EVENT_SCHEMA: &str = "EVENT_PLANNER_SCHEMA";
const USER_SCHEMA: &str = "USER_SCHEMA";

pub async fn app_sql_pool() -> anyhow::Result<SqlitePool> {
    let pool = sql_pool().await?;
    create_table(&pool, USER_SCHEMA).await?;
    create_table(&pool, EVENT_SCHEMA).await?;

    Ok(pool)
}

pub fn event_router() -> Router<SqlitePool> {
    Router::new()
        .route("/", get(event_list_get))
        .route("/add", get(event_add_get).post(event_add_post))
        .route("/delete", get(event_delete_get).post(event_delete_post))
        .route("/{id}", get(event_by_id_get))
}

pub fn user_router() -> Router<SqlitePool> {
    Router::new()
        .route("/register", get(user_register_get).post(user_register_post))
        .route("/login", get(user_login_get).post(user_login_post))
        .route("/profile", get(user_profile_get))
        .route("/delete", post(user_delete_post))
        .route("/logout", post(user_logout_post))
}

pub async fn app_router(pool: SqlitePool) -> Router {
    let session_store = SqliteStore::new(pool.clone());
    session_store
        .migrate()
        .await
        .expect("session migration failed");

    let session_layer = SessionManagerLayer::new(session_store)
        .with_expiry(Expiry::OnInactivity(Duration::days(7)));

    Router::new()
        .route("/", get(home_get))
        .merge(user_router())
        .nest("/events", event_router())
        .fallback(not_found_get)
        .layer(session_layer)
        .with_state(pool)
}

pub async fn app_listener() -> TcpListener {
    println!("Listening on http://127.0.0.1:3000");

    TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("port 3000 is already in use")
}
