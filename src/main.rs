#[macro_use]
extern crate rocket;

use rocket::serde::json::{json, Json, Value};
use rocket_db_pools::{Connection, Database};

use serde::{Deserialize, Serialize};
use std::borrow::Cow;

mod book;
mod chapter;
mod params;
mod search;
mod verse;

#[derive(Database)]
#[database("bible")]
struct Bible(sqlx::PgPool);

#[derive(Serialize)]
struct SearchResult {
    title: String,
    chapter: i32,
    verse: i32,
    text: String,
}

#[get("/")]
fn index() -> &'static str {
    "Welcome to the rust bible api!"
}

#[derive(Serialize, Deserialize)]
struct Query<'a> {
    query: Cow<'a, str>,
}

#[post("/search", format = "json", data = "<query>")]
async fn bible_search(mut db: Connection<Bible>, query: Json<Query<'_>>) -> Value {
    match search::search(&query.query.to_string()) {
        Ok(result) => {
            let result = sqlx::query_as!(
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
                        ANd v.num = ANY($3)
                    ORDER BY c.num, v.num
                ",
                result.title,
                &result
                    .chapter
                    .iter()
                    .map(|c| i32::from(c.chapter))
                    .collect::<Vec<i32>>()[..],
                &result
                    .chapter
                    .iter()
                    .map(|c| c.verse.iter().map(|v| i32::from(*v)).collect::<Vec<i32>>())
                    .flatten()
                    .collect::<Vec<i32>>()[..],
            )
            .fetch_all(&mut *db)
            .await;

            match result {
                Ok(result) => json!({"result": "ok", "data": result}),
                Err(err) => panic!("Problem parsing arguments: {}", err),
            }
            // json!({"result": "o", "data": result})
        }
        Err(err) => panic!("Problem parsing arguments: {}", err),
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Bible::init())
        .mount("/", routes![index, bible_search])
}
