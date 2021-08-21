use axum::{
    body::Body,
    http::{Response, StatusCode},
    response::IntoResponse,
    response::Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub(crate) enum Errors {
    #[error("database error")]
    DatabaseError(#[from] sqlx::Error),
    #[error("not found")]
    NotFound,
    #[error("invalid signature")]
    InvalidSignature,
}

impl IntoResponse for Errors {
    fn into_response(self) -> axum::http::Response<axum::body::Body> {
        match self {
            Errors::DatabaseError(e) => {
                tracing::error!("database error: {:?}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({
                        "error": "database error",
                    })),
                )
                    .into_response()
            }
            Errors::NotFound => (
                StatusCode::NOT_FOUND,
                Json(json!({
                    "error": "not found",
                })),
            )
                .into_response(),
            Errors::InvalidSignature => (
                StatusCode::BAD_REQUEST,
                Json(json!({
                    "error": "invalid signature",
                })),
            )
                .into_response(),
        }
    }
}
