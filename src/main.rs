extern crate dotenv;
mod book;
mod chapter;
mod db;
mod params;
mod search;
mod verse;

use axum::{extract::Query, extract::State, http::StatusCode, routing::get, Json, Router};
use db::SearchResult;
use serde::{de, Deserialize, Deserializer};
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::{fmt, str::FromStr, time::Duration};
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("Failed to load .env file");

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .expect("Failed to load .env file (tracing)"),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let db_connection_str = std::env::var("DATABASE_URL").expect("DATABASE_URL not set");

    // setup connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&db_connection_str)
        .await
        .expect("can't connect to database");

    // build our application with some routes
    let app = Router::new()
        .route("/", get(hello))
        .route("/search", get(search))
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(pool);

    // run it with hyper
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn hello(State(pool): State<PgPool>) -> Result<String, (StatusCode, String)> {
    sqlx::query_scalar("select 'hello world from pg'")
        .fetch_one(&pool)
        .await
        .map_err(internal_error)
}

#[derive(Debug, Deserialize)]
struct Params {
    #[serde(default, deserialize_with = "empty_string_as_none")]
    query: Option<String>,
}

/// Serde deserialization decorator to map empty Strings to None,
fn empty_string_as_none<'de, D, T>(de: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr,
    T::Err: fmt::Display,
{
    let opt = Option::<String>::deserialize(de)?;
    match opt.as_deref() {
        None | Some("") => Ok(None),
        Some(s) => FromStr::from_str(s).map_err(de::Error::custom).map(Some),
    }
}

async fn search(
    State(pool): State<PgPool>,
    Query(params): Query<Params>,
) -> Result<Json<Vec<SearchResult>>, (StatusCode, String)> {
    let query = params.query.ok_or((
        StatusCode::BAD_REQUEST,
        "missing query parameter".to_string(),
    ))?;

    match search::search(&query) {
        Ok(bible_search) => match db::search(pool, bible_search).await {
            Ok(results) => Ok(results),
            Err(err) => Err(err),
        },
        Err(err) => Err((StatusCode::NOT_FOUND, err)),
    }
}

/// Utility function for mapping any error into a `500 Internal Server Error` response.
fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
