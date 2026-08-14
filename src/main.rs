use mini_axum_event_planner::{event_planner_listener, event_planner_router};

#[tokio::main]
async fn main() {
    axum::serve(event_planner_listener().await, event_planner_router())
        .await
        .unwrap();
}
