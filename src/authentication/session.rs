use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use axum::response::{IntoResponse, Redirect, Response};
use tower_sessions::Session;

pub struct CurrentUser(pub i64);

impl<S> FromRequestParts<S> for CurrentUser
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let session = Session::from_request_parts(parts, state)
            .await
            .map_err(|_| Redirect::to("/login").into_response())?;

        match session.get::<i64>("user_id").await {
            Ok(Some(user_id)) => Ok(CurrentUser(user_id)),
            _ => Err(Redirect::to("/login").into_response()),
        }
    }
}
