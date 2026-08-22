use anyhow::Context;
use serde::Deserialize;
use sqlx::SqlitePool;

use crate::authentication::hashing::generate_hashed_password;

#[derive(sqlx::FromRow)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub hashed_password: String,
}

#[derive(Deserialize)]
pub struct UserNew {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct UserLogin {
    pub username: String,
    pub password: String,
}

impl User {
    pub async fn user_register(pool: &SqlitePool, user: &UserNew) -> anyhow::Result<i64> {
        let hashed_password =
            generate_hashed_password(&user.password).context("Failed hashing user password!")?;

        let query = "INSERT INTO User (username, email, hashed_password) VALUES (?, ?, ?)";
        let user_to_register = sqlx::query(query)
            .bind(&user.username)
            .bind(&user.email)
            .bind(&hashed_password)
            .execute(pool)
            .await
            .context("Failed adding new user to table!")?
            .last_insert_rowid();

        // returns last registered user id
        Ok(user_to_register)
    }

    pub async fn user_by_id(pool: &SqlitePool, user_id: &i64) -> anyhow::Result<Option<User>> {
        let query = "SELECT id, username, email, hashed_password FROM User WHERE id = ?";
        let user = sqlx::query_as::<_, User>(query)
            .bind(user_id)
            .fetch_optional(pool)
            .await
            .context("Failed finding user by id!")?;

        // returns the specific queried user by id
        Ok(user)
    }

    pub async fn user_by_username(
        pool: &SqlitePool,
        username: &str,
    ) -> anyhow::Result<Option<User>> {
        let query = "SELECT id, username, email, hashed_password FROM User WHERE username = ?";
        let user = sqlx::query_as::<_, User>(query)
            .bind(username)
            .fetch_optional(pool)
            .await
            .context("Failed finding user by id!")?;

        // returns the specific queried user by username
        Ok(user)
    }

    pub async fn user_delete(pool: &SqlitePool, user_id: i64) -> anyhow::Result<u64> {
        let query = "DELETE FROM User WHERE id = ?";
        let rows = sqlx::query(query)
            .bind(user_id)
            .execute(pool)
            .await
            .context("Failed deleting user from table!")?
            .rows_affected();

        // returns the number of deleted rows from the table
        Ok(rows)
    }
}
