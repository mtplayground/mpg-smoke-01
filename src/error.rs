use std::borrow::Cow;
use std::env::VarError;
use std::io;

use axum::Json;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::Serialize;

#[derive(Debug)]
pub enum AppError {
    MissingEnv(&'static str),
    InvalidEnv {
        key: &'static str,
        value: String,
    },
    Validation(Cow<'static, str>),
    NotFound(Cow<'static, str>),
    Database {
        context: &'static str,
        query: Option<&'static str>,
        source: sqlx::Error,
    },
    Bind(io::Error),
    Server(io::Error),
}

#[derive(Serialize)]
struct ErrorResponse<'a> {
    error: &'a str,
}

impl AppError {
    pub fn missing_env(key: &'static str) -> Self {
        Self::MissingEnv(key)
    }

    pub fn invalid_env(key: &'static str, value: String) -> Self {
        Self::InvalidEnv { key, value }
    }

    pub fn validation(message: impl Into<Cow<'static, str>>) -> Self {
        Self::Validation(message.into())
    }

    pub fn not_found(message: impl Into<Cow<'static, str>>) -> Self {
        Self::NotFound(message.into())
    }

    pub fn database_query(query: &'static str, source: sqlx::Error) -> Self {
        tracing::error!(query, error = %source, "database query failed");
        Self::Database {
            context: "database query failed",
            query: Some(query),
            source,
        }
    }

    pub fn database_connection(context: &'static str, source: sqlx::Error) -> Self {
        tracing::error!(context, error = %source, "database operation failed");
        Self::Database {
            context,
            query: None,
            source,
        }
    }

    pub fn bind(source: io::Error) -> Self {
        Self::Bind(source)
    }

    pub fn server(source: io::Error) -> Self {
        Self::Server(source)
    }
}

impl From<VarError> for AppError {
    fn from(_: VarError) -> Self {
        Self::missing_env("DATABASE_URL")
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            Self::MissingEnv(key) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("missing required environment variable: {key}"),
            ),
            Self::InvalidEnv { key, value } => (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("invalid environment variable {key}: {value}"),
            ),
            Self::Validation(message) => (StatusCode::BAD_REQUEST, message.into_owned()),
            Self::NotFound(message) => (StatusCode::NOT_FOUND, message.into_owned()),
            Self::Database { context, query, source } => {
                tracing::error!(context, query, error = %source, "returning database error response");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "internal database error".to_string(),
                )
            }
            Self::Bind(source) => {
                tracing::error!(error = %source, "failed to bind TCP listener");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "failed to bind server".to_string(),
                )
            }
            Self::Server(source) => {
                tracing::error!(error = %source, "server exited with error");
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "server exited with error".to_string(),
                )
            }
        };

        (status, Json(ErrorResponse { error: &message })).into_response()
    }
}
