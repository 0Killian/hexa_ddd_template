mod error;
mod not_found;

use common::state::AppState;
pub use error::{render_alert, render_error};

pub fn router() -> axum::Router<AppState> {
    axum::Router::new().fallback(not_found::not_found)
}
