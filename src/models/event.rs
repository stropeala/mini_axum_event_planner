use anyhow::Context;
use serde::Deserialize;
use sqlx::SqlitePool;

#[derive(sqlx::FromRow)]
pub struct Event {
    pub id: i64,
    pub name: String,
    pub date: String,
    pub time: String,
    pub description: String,
}

#[derive(Deserialize)]
pub struct EventNew {
    pub name: String,
    pub date: String,
    pub time: String,
    pub description: String,
}

#[derive(Deserialize)]
pub struct EventDelete {
    pub id: i64,
}

impl Event {
    pub async fn event_add(
        pool: &SqlitePool,
        event: &EventNew,
        user_id: i64,
    ) -> anyhow::Result<i64> {
        let query = "INSERT INTO EventPlanner (name, date, time, description, user_id) VALUES (?, ?, ?, ?, ?)";
        let event_id = sqlx::query(query)
            .bind(&event.name)
            .bind(&event.date)
            .bind(&event.time)
            .bind(&event.description)
            .bind(user_id)
            .execute(pool)
            .await
            .context("Failed adding new event to table!")?
            .last_insert_rowid();

        // returns last added event id
        Ok(event_id)
    }

    pub async fn event_list(pool: &SqlitePool, user_id: i64) -> anyhow::Result<Vec<Event>> {
        let query = "SELECT id, name, date, time, description FROM EventPlanner WHERE user_id = ? ORDER BY date, time";
        let events = sqlx::query_as::<_, Event>(query)
            .bind(user_id)
            .fetch_all(pool)
            .await
            .context("Failed getting events from table!")?;

        // returns all events
        Ok(events)
    }

    pub async fn event_by_id(
        pool: &SqlitePool,
        event_id: i64,
        user_id: i64,
    ) -> anyhow::Result<Option<Event>> {
        let query = "SELECT id, name, date, time, description FROM EventPlanner WHERE id = ? AND user_id = ?";
        let event = sqlx::query_as::<_, Event>(query)
            .bind(event_id)
            .bind(user_id)
            .fetch_optional(pool)
            .await
            .context("Failed getting event from table!")?;

        // returns the specific queried event
        Ok(event)
    }

    pub async fn event_delete(
        pool: &SqlitePool,
        event_id: i64,
        user_id: i64,
    ) -> anyhow::Result<u64> {
        let query = "DELETE FROM EventPlanner WHERE id = ? AND user_id = ?";
        let rows = sqlx::query(query)
            .bind(event_id)
            .bind(user_id)
            .execute(pool)
            .await
            .context("Failed deleting event from table!")?
            .rows_affected();

        // returns the number of deleted rows from the table
        Ok(rows)
    }
}
