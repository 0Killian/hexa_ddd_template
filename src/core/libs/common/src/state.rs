use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub db: sqlx::PgPool,
}
