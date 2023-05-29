use axum::{http::StatusCode, Json};
use serde::Serialize;
use sqlx::{Pool, Postgres};

use crate::{
    internal_error,
    search::{BibleSearch, Chapter},
};
#[derive(Serialize)]
pub struct SearchResult {
    pub title: String,
    pub chapter: i32,
    pub verse: i32,
    pub text: String,
}

pub async fn search(
    pool: Pool<Postgres>,
    bible_search: BibleSearch,
) -> Result<Json<Vec<SearchResult>>, (StatusCode, String)> {
    let title = bible_search.title;
    let chapter = bible_search.chapter.chapter as i32;
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
                    AND c.num = $2
                    AND v.num = ANY($3)
              ORDER BY v.num
      ",
        title,
        chapter,
        &verses[..],
    )
    .fetch_all(&pool)
    .await
    .map(|results| Json(results))
    .map_err(internal_error)
}

fn get_verses(chapter: &Chapter) -> Vec<i32> {
    chapter
        .verses
        .iter()
        .map(|v| i32::from(*v))
        .collect::<Vec<i32>>()
}
