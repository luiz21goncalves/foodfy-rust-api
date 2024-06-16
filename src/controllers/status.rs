use axum::{extract::State, Json};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Database {
    version: String,
    max_connections: i32,
    opened_connections: i32,
    pool_size: u32,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
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

    let pool_size = pool.size();

    let status = Status {
        version: env!("CARGO_PKG_VERSION").to_string(),
        updated_at: chrono::Utc::now(),
        database: Database {
            version: postgres_version,
            max_connections,
            opened_connections,
            pool_size,
        },
    };

    Json(status)
}

#[cfg(test)]
mod tests {
    use crate::{
        app,
        controllers::status::{Database, Status},
    };
    use axum::http::StatusCode;
    use axum_test_helpers::TestClient;

    #[tokio::test]
    async fn get_api_status() {
        let app = app().await;

        let client = TestClient::new(app);
        let response = client.get("/v1/status").await;

        assert_eq!(response.status(), StatusCode::OK);

        let body = response.json::<Status>().await;

        assert_eq!(
            body.database,
            Database {
                max_connections: 100,
                opened_connections: 2,
                pool_size: 2,
                version: String::from("16.3")
            }
        );
        assert_eq!(body.version, String::from("0.0.0"));
        assert_eq!(
            body.updated_at.format("%Y-%m-%d %H:%M").to_string(),
            chrono::Utc::now().format("%Y-%m-%d %H:%M").to_string()
        );
    }
}
