use std::{net::SocketAddr, sync::Arc};

use axum::{extract::Request, middleware::from_fn_with_state, ServiceExt};
use common::{
    config::CONFIG,
    errors::{self, AppError},
    state::AppState,
};
use middleware::{error, request_id};
use sqlx::PgPool;
use tokio::net::TcpListener;
use tower::Layer;
use tower_http::{
    normalize_path::NormalizePathLayer, request_id::RequestId, services::ServeDir,
    trace::TraceLayer,
};

mod middleware;

pub async fn serve() -> errors::Result<()> {
    let router = router().await;
    let router = NormalizePathLayer::trim_trailing_slash().layer(router);

    let addr = SocketAddr::from(([0, 0, 0, 0], CONFIG.http.listen_port));
    let listener = TcpListener::bind(addr)
        .await
        .map_err(|e| AppError::Setup(format!("Failed to bind to {}: {}", addr, e)))?;

    axum::serve(listener, ServiceExt::<Request>::into_make_service(router))
        .await
        .map_err(|e| AppError::Setup(format!("Failed to start HTTP server: {}", e)))
}

async fn router() -> axum::Router {
    let current_dir = std::env::current_dir().unwrap();

    let state = AppState {
        db: PgPool::connect(&CONFIG.db_url).await.unwrap(),
    };

    axum::Router::new()
        // Routers
        .nest("/app", front::router())
        .nest("/api/v1", api::router())
        // Middlewares
        .layer(
            TraceLayer::new_for_http().make_span_with(|request: &Request| {
                let request_id = request
                    .extensions()
                    .get::<RequestId>()
                    .and_then(|id| id.header_value().to_str().ok())
                    .unwrap_or_default();

                tracing::debug_span!(
                    "request",
                    request_id = request_id,
                    method = %request.method(),
                    uri = %request.uri(),
                    status_code = tracing::field::Empty,
                )
            }),
        )
        .layer(request_id::set_request_id_layer())
        .layer(request_id::propagate_request_id_layer())
        .nest_service("/assets", ServeDir::new(current_dir.join("assets")))
        // .merge(landing::router())
        .layer(from_fn_with_state(state.clone(), error::extract_error))
        .with_state(state)
}
