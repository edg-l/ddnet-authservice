use axum::{
    body::{Body, Bytes, Full},
    http::{Response, StatusCode},
    response::IntoResponse,
    response::Json,
};
use serde::Serialize;
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

#[derive(Serialize, Debug)]
struct JsonError {
    error: String,
}

impl IntoResponse for Errors {
    type Body = Full<Bytes>;
    type BodyError = <Self::Body as axum::body::HttpBody>::Error;

    fn into_response(self) -> Response<Self::Body> {
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
