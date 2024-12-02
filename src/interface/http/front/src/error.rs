use common::{errors::AppError, state::AppState};
use yew::{prelude::*, ServerRenderer};

#[derive(Properties, PartialEq)]
pub struct AppErrorProps {
    error: AppError,
}

#[function_component]
pub fn AppErrorTemplate(props: &AppErrorProps) -> yew::Html {
    html! {
        <>
            <h1>{props.error.title()}</h1>
            <p>{props.error.message()}</p>
        </>
    }
}

pub async fn render_alert(error: AppError) -> String {
    format!("<p>{}</p>", error.message())
}

pub async fn render_error(
    error: AppError,
    _state: AppState, /* user: Option<AuthenticatedUser> */
) -> String {
    ServerRenderer::<AppErrorTemplate>::with_props(move || AppErrorProps { error })
        .render()
        .await
}
