extern crate dotenv;
mod book;
mod chapter;
mod db;
mod params;
mod search;
mod verse;

use axum::{extract::Query, extract::State, http::StatusCode, routing::get, Json, Router};
use search::Chapter;
use serde::{de, Deserialize, Deserializer, Serialize};
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::{fmt, str::FromStr, time::Duration};
use tokio::net::TcpListener;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Serialize)]
pub struct SearchResult {
    pub title: String,
    pub chapter: i32,
    pub verse: i32,
    pub text: String,
}

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
        .with_state(pool);

    // run it with hyper
    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn hello(State(pool): State<PgPool>) -> Result<String, (StatusCode, String)> {
    sqlx::query_scalar("select 'hello world from pg'")
        .fetch_one(&pool)
        .await
        .map_err(internal_error)
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
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
    let query = match params.query {
        Some(q) => q,
        None => {
            return Err((
                StatusCode::BAD_REQUEST,
                "missing query parameter".to_string(),
            ))
        }
    };

    match search::search(&query) {
        Ok(bible_search) => {
            let title = bible_search.title;
            let chapters = get_chapters(&bible_search.chapter);
            let verses = get_verses(&bible_search.chapter);

            sqlx::query_as!(
                SearchResult,
                "
                SELECT
                    b.title as title,
                    c.num as chapter,
                    v.num as verse,
                    v.contents as text
                FROM books b
                    INNER JOIN chapters c ON c.title = b.title
                    INNER JOIN verses v ON v.title = c.title
                        AND v.chapter_num = c.num
                WHERE b.title = $1
                    AND c.num = ANY($2)
                    AND v.num = ANY($3)
              ORDER BY c.num, v.num
      ",
                title,
                &chapters[..],
                &verses[..],
            )
            .fetch_all(&pool)
            .await
            .map(|results| Json(results))
            .map_err(internal_error)
        }
        Err(err) => Err((StatusCode::NOT_FOUND, err)),
    }
}

fn get_chapters(chapters: &Vec<Chapter>) -> Vec<i32> {
    chapters
        .iter()
        .map(|c| i32::from(c.chapter))
        .collect::<Vec<i32>>()
}

fn get_verses(chapters: &Vec<Chapter>) -> Vec<i32> {
    chapters
        .iter()
        .map(|c| c.verse.iter().map(|v| i32::from(*v)).collect::<Vec<i32>>())
        .flatten()
        .collect::<Vec<i32>>()
}

/// Utility function for mapping any error into a `500 Internal Server Error` response.
fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
