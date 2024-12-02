use axum::{
    extract::State,
    http::StatusCode,
    response::{Html, IntoResponse},
};
use common::{errors::AppError, state::AppState};

use crate::render_error;

pub async fn not_found(
    State(state): State<AppState>, /* user: Option<AuthenticatedUser> */
) -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        Html(render_error(AppError::NotFound, state).await),
    )
}
