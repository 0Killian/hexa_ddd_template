use axum::{
    body::Body,
    extract::State,
    http::{Request, Response, StatusCode},
    middleware::Next,
    response::{Html, IntoResponse},
    Json,
};
use common::{errors::AppError, state::AppState};
use serde::Serialize;

#[derive(PartialEq, Debug)]
enum ResponseType {
    Html,
    Json,
}

#[derive(Serialize)]
struct ErrorResponse {
    data: AppError,
    html: String,
}

/// Extracts errors passed in the responses of the routes as extensions, and renders them as HTML or JSON
/// depending on the Accept header
pub async fn extract_error(
    //user: Option<AuthenticatedUser>,
    State(state): State<AppState>,
    request: Request<Body>,
    next: Next,
) -> impl IntoResponse {
    let headers = request.headers();

    let is_hx_request = headers
        .get("HX-Request")
        .and_then(|hv| hv.to_str().ok())
        .is_some();

    let response_type = if let Some(accept) = headers.get("Accept").and_then(|hv| hv.to_str().ok())
    {
        if accept.contains("text/html") {
            ResponseType::Html
        } else {
            ResponseType::Json
        }
    } else {
        ResponseType::Json
    };

    let response = next.run(request).await;
    let error = if let Some(error) = response.extensions().get::<AppError>() {
        error.clone()
    } else {
        return response;
    };

    render_error(
        //user,
        state,
        error,
        is_hx_request,
        response_type,
    )
    .await
}

#[tracing::instrument(skip(state))]
async fn render_error(
    //user: Option<AuthenticatedUser>,
    state: AppState,
    error: AppError,
    is_hx_request: bool,
    response_type: ResponseType,
) -> Response<Body> {
    let status = error.status();

    let redirect_to_value = error.redirect_to().unwrap_or_default();
    let _redirect_to = format!("/sign-in?redirect_to={redirect_to_value}",);
    let response_status = StatusCode::from_u16(status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);

    match error {
        _ => {
            let template = if is_hx_request || response_type == ResponseType::Json {
                front::render_alert(error.clone()).await
            } else {
                front::render_error(error.clone(), state).await
            };

            match response_type {
                ResponseType::Html => (response_status, Html(template)).into_response(),
                ResponseType::Json => (
                    response_status,
                    Json(ErrorResponse {
                        data: error,
                        html: template,
                    }),
                )
                    .into_response(),
            }
        }
    }
}
