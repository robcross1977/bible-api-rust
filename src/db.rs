use crate::search::{BibleSearch, Chapter};
use rocket_db_pools::{Connection, Database};
use serde::Serialize;

#[derive(Database)]
#[database("bible")]
pub struct Bible(sqlx::PgPool);

#[derive(Serialize)]
pub struct SearchResult {
    title: String,
    chapter: i32,
    verse: i32,
    text: String,
}

pub async fn search(
    mut db: Connection<Bible>,
    bible_search: BibleSearch,
) -> Result<Vec<SearchResult>, sqlx::Error> {
    let title = bible_search.title;
    let chapters = get_chapters(&bible_search.chapter);
    let verses = get_verses(&bible_search.chapter);

    sqlx::query_as!(
        SearchResult,
        "
      SELECT b.title as title, c.num as chapter, v.num as verse, v.contents as text
      FROM books b
        INNER JOIN chapters c
          ON c.title = b.title
        INNER JOIN verses v
          ON v.title = c.title
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
    .fetch_all(&mut *db)
    .await
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
