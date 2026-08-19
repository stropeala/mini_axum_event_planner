use dotenvy::dotenv;

use mini_axum_event_planner::{event_planner_listener, event_planner_pool, event_planner_router};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let pool = event_planner_pool().await?;
    axum::serve(event_planner_listener().await, event_planner_router(pool)).await?;
    Ok(())
}
