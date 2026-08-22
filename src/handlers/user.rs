use anyhow::Context;
use askama::Template;
use askama_web::WebTemplate;
use axum::extract::{Form, State};
use axum::response::Redirect;
use sqlx::SqlitePool;
use tower_sessions::Session;

use crate::authentication::hashing::check_hashed_password;
use crate::authentication::session::CurrentUser;
use crate::error::AppError;
use crate::models::user::{User, UserLogin, UserNew};

// Register new user

#[derive(Template, WebTemplate)]
#[template(path = "user_register.html")]
pub struct UserRegisterPage {}

pub async fn user_register_get() -> UserRegisterPage {
    UserRegisterPage {}
}

pub async fn user_register_post(
    State(pool): State<SqlitePool>,
    Form(user_new): Form<UserNew>,
) -> Result<Redirect, AppError> {
    User::user_register(&pool, &user_new).await?;

    Ok(Redirect::to("/login"))
}

// Login

#[derive(Template, WebTemplate)]
#[template(path = "user_login.html")]
pub struct UserLoginPage {}

pub async fn user_login_get() -> UserLoginPage {
    UserLoginPage {}
}

pub async fn user_login_post(
    State(pool): State<SqlitePool>,
    session: Session,
    Form(user_login): Form<UserLogin>,
) -> Result<Redirect, AppError> {
    let user = User::user_by_username(&pool, &user_login.username).await?;

    match user {
        Some(user) => {
            let password_correct =
                check_hashed_password(&user_login.password, &user.hashed_password)
                    .context("Failed checking password!")?;

            if password_correct {
                session.insert("user_id", user.id).await?;
                Ok(Redirect::to("/events"))
            } else {
                Ok(Redirect::to("/login"))
            }
        }
        None => Ok(Redirect::to("/login")),
    }
}

pub async fn user_logout_post(session: Session, CurrentUser(_user_id): CurrentUser) -> Redirect {
    session.clear().await;
    Redirect::to("/")
}

// User profile

#[derive(Template, WebTemplate)]
#[template(path = "user_profile.html")]
pub struct UserProfilePage {
    pub user: User,
}

pub async fn user_profile_get(
    State(pool): State<SqlitePool>,
    CurrentUser(user_id): CurrentUser,
) -> Result<UserProfilePage, AppError> {
    let user = User::user_by_id(&pool, &user_id)
        .await?
        .ok_or_else(|| anyhow::anyhow!("User not found!"))?;

    Ok(UserProfilePage { user })
}

pub async fn user_delete_post(
    State(pool): State<SqlitePool>,
    session: Session,
    CurrentUser(user_id): CurrentUser,
) -> Result<Redirect, AppError> {
    User::user_delete(&pool, user_id).await?;
    session.clear().await;
    Ok(Redirect::to("/"))
}
