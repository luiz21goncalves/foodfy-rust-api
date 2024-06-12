use axum::Json;
use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Serialize)]
pub struct Status {
    version: String,
    date: DateTime<Utc>,
}

pub async fn api_status() -> Json<Status> {
    let status = Status {
        version: env!("CARGO_PKG_VERSION").to_string(),
        date: chrono::Utc::now(),
    };

    Json(status)
}
