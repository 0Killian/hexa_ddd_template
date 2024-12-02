use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub fn init() {
    let registry = tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or("trace".into()));

    let log_fmt = std::env::var("LOG_FMT").unwrap_or_else(|_| "pretty".to_string());
    match log_fmt.as_str() {
        "json" => {
            registry
                .with(tracing_subscriber::fmt::layer().json())
                .init();
        }
        _ => {
            registry
                .with(tracing_subscriber::fmt::layer().pretty())
                .init();
        }
    }
}
