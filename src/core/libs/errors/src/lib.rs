use axum::{response::IntoResponse, Extension};
use serde::Serialize;
use thiserror::Error;

#[derive(Serialize, Error, Debug, Clone, PartialEq)]
pub enum AppError {
    #[error("An error occurred while setting up: {0}")]
    Setup(String),

    #[error("error: {0}")]
    Generic(String),

    #[error("An element was not found")]
    NotFound,

    #[error("Validation error: {0:?}")]
    ValidationError(Vec<ValidationError>),

    #[error("Incorrect header")]
    IncorrectHeader,
}

pub type Result<T> = std::result::Result<T, AppError>;

impl AppError {
    pub fn status(&self) -> u16 {
        match self {
            AppError::Generic(_) => 500,
            AppError::NotFound => 404,
            AppError::ValidationError(_) => 400,
            AppError::IncorrectHeader => 406,
            AppError::Setup(_) => unreachable!(),
        }
    }

    pub fn redirect_to(&self) -> Option<String> {
        match self {
            AppError::Generic(_) => None,
            AppError::NotFound => None,
            AppError::ValidationError(_) => None,
            AppError::IncorrectHeader => None,
            AppError::Setup(_) => unreachable!(),
        }
    }

    pub fn title(&self) -> String {
        match self {
            AppError::Generic(_) => "Une erreur interne est survenue".to_string(),
            AppError::NotFound => "Élément introuvable".to_string(),
            AppError::ValidationError(_) => "Erreurs de validation".to_string(),
            AppError::IncorrectHeader => "Header incorrect".to_string(),
            AppError::Setup(_) => unreachable!(),
        }
    }

    pub fn message(&self) -> String {
        match self {
            AppError::Generic(message) => message.clone(),
            AppError::NotFound => "L'élément demandé n'a pas été trouvé.".to_string(),
            AppError::ValidationError(errors) => {
                let mut message = String::new();
                for error in errors {
                    message.push_str(&format!("{}: {}\n", error.field, error.reason));
                }
                message
            }
            AppError::IncorrectHeader => "".to_string(),
            AppError::Setup(_) => unreachable!(),
        }
    }
}

impl From<sqlx::Error> for AppError {
    fn from(e: sqlx::Error) -> Self {
        log_error!(match e {
            sqlx::Error::RowNotFound => AppError::NotFound,
            _ => AppError::Generic(e.to_string()),
        })
    }
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub struct ValidationError {
    pub field: String,
    pub reason: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        Extension(self).into_response()
    }
}

#[macro_export]
macro_rules! create_log_error {
    ($variant:ident, $( $param:expr ),*) => {{
        use tracing::error;
        use errors::AppError;

        let err = AppError::$variant($($param),*);
        error!("{:?}", err);
        err
    }};

    ($message: expr, $variant:ident) => {{
        use tracing::error;
        use errors::AppError;

        let err = AppError::$variant($message);
        error!("{} ({:?})", $message, err);
        err
    }};
}

#[macro_export]
macro_rules! log_error {
    ($error: expr) => {{
        use tracing::error;

        error!("{:?}", $error);
        $error
    }};

    ($message: expr, $error: expr) => {{
        use tracing::error;

        error!("{} ({:?})", $message, $error);
        $error
    }};
}
