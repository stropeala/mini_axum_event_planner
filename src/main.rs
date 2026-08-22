use dotenvy::dotenv;

use mini_axum_event_planner::{app_listener, app_router, app_sql_pool};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let pool = app_sql_pool().await?;
    axum::serve(app_listener().await, app_router(pool).await).await?;
    Ok(())
}
