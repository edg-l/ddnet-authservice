#![forbid(unsafe_code)]

use axum::response::Json;
use axum::AddExtensionLayer;
use axum::{http::StatusCode, prelude::*, response::IntoResponse};
use base64::STANDARD;
use base64_serde::base64_serde_type;
use ed25519_dalek::{PublicKey, Signature, Verifier};
use errors::Errors;
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::MySqlPool;
use std::convert::TryFrom;
use std::net::SocketAddr;
use std::time::Duration;
use std::{convert::TryInto, sync::Arc};
use tower::ServiceBuilder;
use tower_http::{compression::CompressionLayer, trace::TraceLayer};
use tracing::debug;
use tracing_subscriber::EnvFilter;

use crate::{email::Mailer, settings::Config};

pub(crate) mod db;
pub(crate) mod email;
pub(crate) mod errors;
pub(crate) mod settings;

base64_serde_type!(Base64Serde, STANDARD);

fn setup() {
    dotenv::dotenv().ok();

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info")
    }

    tracing_subscriber::fmt::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();
}

#[tokio::main]
async fn main() {
    setup();

    let config = Config::default();
    let pool = MySqlPool::connect(&config.database.url)
        .await
        .expect("Error connecting to db");

    sqlx::migrate!("./migrations").run(&pool).await.unwrap();

    let mailer: Arc<Mailer> = Arc::new(config.smtp.try_into().expect("error loading mailer"));

    let middleware_stack = ServiceBuilder::new()
        // timeout all requests after 5 seconds
        .timeout(Duration::from_secs(5))
        // add high level tracing of requests and responses
        .layer(TraceLayer::new_for_http())
        // compression responses
        .layer(CompressionLayer::new())
        // convert the `ServiceBuilder` into a `tower::Layer`
        .into_inner();

    let app = route("/version", get(version))
        .route("/account/mapping", post(account_id_mapping))
        .route("/account/verify", post(verify_user))
        .route("/account/register", post(register_account))
        .layer(middleware_stack)
        .layer(AddExtensionLayer::new(mailer))
        .layer(AddExtensionLayer::new(pool));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[derive(Debug, Deserialize)]
struct AccoundIdRequest {
    public_key: Vec<u8>,
}

async fn account_id_mapping(
    pool: extract::Extension<MySqlPool>,
    extract::Json(account_request): extract::Json<AccoundIdRequest>,
) -> Result<impl IntoResponse, Errors> {
    let id = db::get_account_id(&pool, &account_request.public_key).await?;

    match id {
        Some(id) => Ok(id.to_string()),
        None => Err(Errors::NotFound),
    }
}

#[derive(Debug, Deserialize)]
struct RegisterPayload {
    #[serde(with = "Base64Serde")]
    public_key: Vec<u8>,
    email: String,
    #[serde(with = "Base64Serde")]
    email_signature: Vec<u8>,
}

async fn register_account(
    pool: extract::Extension<MySqlPool>,
    extract::Json(payload): extract::Json<RegisterPayload>,
) -> Result<impl IntoResponse, Errors> {
    debug!("Incoming register request");
    
    dbg!(&payload);

    let public_key = PublicKey::from_bytes(&payload.public_key).unwrap();
    let signature = Signature::try_from(&payload.email_signature[..]).unwrap();

    dbg!(&public_key);
    dbg!(&signature);

    let verified = public_key
        .verify(payload.email.as_bytes(), &signature)
        .is_ok();

    dbg!(&verified);

    if verified {
        let id = uuid::Uuid::new_v4();
        // TODO: check if email already exists.
        db::add_account_mapping(&pool, public_key.as_bytes(), &id, &payload.email).await?;
        Ok(id.to_string())
    } else {
        Err(Errors::InvalidSignature)
    }
}

#[derive(Debug, Deserialize)]
struct VerifyUserPayload {
    #[serde(with = "Base64Serde")]
    public_key: Vec<u8>,
    message: String,
    #[serde(with = "Base64Serde")]
    message_signature: Vec<u8>,
}

async fn verify_user(
    pool: extract::Extension<MySqlPool>,
    extract::Json(payload): extract::Json<VerifyUserPayload>,
) -> Result<impl IntoResponse, Errors> {
    if let Some(account_id) = db::get_account_id(&pool, &payload.public_key).await? {
        let public_key = PublicKey::from_bytes(&payload.public_key).unwrap();
        let signature = Signature::try_from(&payload.message_signature[..]).unwrap();
        let verified = public_key
            .verify(payload.message.as_bytes(), &signature)
            .is_ok();

        if verified {
            Ok(Json(json!({
                "account_id": account_id,
            })))
        } else {
            Err(Errors::InvalidSignature)
        }
    } else {
        Err(Errors::NotFound)
    }
}
