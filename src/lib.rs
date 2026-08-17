mod authentication;
mod db;
mod handlers;
mod template_structs;

use axum::{Router, routing::get};
use tokio::net::TcpListener;

use handlers::home::{homepage, not_found};

pub fn event_planner_router() -> Router {
    Router::new().route("/", get(homepage)).fallback(not_found)
}

pub async fn event_planner_listener() -> TcpListener {
    println!("Listening on http://127.0.0.1:3000");

    TcpListener::bind("127.0.0.1:3000")
        .await
        .expect("port 3000 is already in use")
}
