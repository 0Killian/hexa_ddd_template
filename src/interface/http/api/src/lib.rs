use common::state::AppState;

mod not_found;

pub fn router() -> axum::Router<AppState> {
    axum::Router::new().fallback(not_found::not_found)
}
