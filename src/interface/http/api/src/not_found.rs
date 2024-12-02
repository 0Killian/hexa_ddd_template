use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use common::state::AppState;
use serde_json::json;

pub async fn not_found(
    State(_state): State<AppState>,
    /* user: Option<AuthenticatedUser> */
) -> impl IntoResponse {
    (
        StatusCode::NOT_FOUND,
        Json(json!({
            "error": "not_found"
        })),
    )
}
