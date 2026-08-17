use std::env::var;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use anyhow::Context;
use sqlx::AssertSqlSafe;
use sqlx::sqlite::{SqliteConnectOptions, SqlitePool, SqlitePoolOptions};

const DB_NAME: &str = "event_planner.db";

fn get_schema(schema_name: &str) -> anyhow::Result<String> {
    let schema_path =
        PathBuf::from(var(schema_name).context("Failed getting schema path from env!")?);
    let mut schema = File::open(schema_path).context("Failed opening schema!")?;

    let mut content = String::new();
    schema
        .read_to_string(&mut content)
        .context("Failed reading schema!")?;
    Ok(content)
}

async fn sqlite_pool() -> anyhow::Result<SqlitePool> {
    let db_path =
        PathBuf::from(var("DATA_DIR_PATH").context("Failed getting data dir path from env!")?)
            .join(DB_NAME);

    let conn = SqliteConnectOptions::new()
        .filename(&db_path)
        .create_if_missing(true);
    let pool = SqlitePoolOptions::new()
        .connect_with(conn)
        .await
        .context("Failed connecting to db!")?;
    Ok(pool)
}

async fn create_table(pool: &SqlitePool, schema_name: &str) -> anyhow::Result<()> {
    let schema = get_schema(schema_name)?;
    let query = AssertSqlSafe(schema);

    sqlx::raw_sql(query)
        .execute(pool)
        .await
        .context("Failed creating db!")?;
    Ok(())
}

async fn is_table(pool: &SqlitePool, table_name: &str) -> anyhow::Result<bool> {
    let query = "SELECT name FROM sqlite_master WHERE type = 'table' AND name = ?";
    let table: Option<String> = sqlx::query_scalar(query)
        .bind(table_name)
        .fetch_optional(pool)
        .await
        .context("Failed checking for Events table!")?;
    Ok(table.is_some())
}
