use axum::{extract::State, Json};
use chrono::{DateTime, Utc};
use serde::Serialize;
use sqlx::{Pool, Postgres};

#[derive(Serialize)]
pub struct Database {
    version: String,
    max_connections: i32,
    opened_connections: i32,
}

#[derive(Serialize)]
pub struct Status {
    version: String,
    updated_at: DateTime<Utc>,
    database: Database,
}
struct Count {
    count: Option<i32>,
}
struct PostgresVersion {
    server_version: Option<String>,
}

struct MaxConnections {
    max_connections: Option<String>,
}

pub async fn api_status(State(pool): State<Pool<Postgres>>) -> Json<Status> {
    let postgres_version = sqlx::query_as!(PostgresVersion, "SHOW server_version")
        .fetch_one(&pool)
        .await
        .unwrap()
        .server_version
        .unwrap();

    let database_name = std::env::var("POSTGRES_DB").expect("cannot find POSTGRES_DB env");

    let opened_connections = sqlx::query_as!(
        Count,
        "SELECT count(*)::int FROM pg_stat_activity WHERE datname = $1;",
        database_name,
    )
    .fetch_one(&pool)
    .await
    .unwrap()
    .count
    .unwrap();

    let max_connections = sqlx::query_as!(MaxConnections, "SHOW max_connections;")
        .fetch_one(&pool)
        .await
        .unwrap()
        .max_connections
        .unwrap()
        .parse::<i32>()
        .unwrap();

    let status = Status {
        version: env!("CARGO_PKG_VERSION").to_string(),
        updated_at: chrono::Utc::now(),
        database: Database {
            version: postgres_version,
            max_connections,
            opened_connections,
        },
    };

    Json(status)
}
