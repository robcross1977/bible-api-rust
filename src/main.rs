#[macro_use]
extern crate rocket;

use crate::db::Bible;
use rocket::serde::json::{json, Json, Value};
use rocket_db_pools::{Connection, Database};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

mod book;
mod chapter;
mod db;
mod params;
mod search;
mod verse;

#[get("/")]
fn index() -> &'static str {
    "Welcome to the rust bible api!"
}

#[derive(Serialize, Deserialize)]
struct Query<'a> {
    query: Cow<'a, str>,
}

#[post("/search", format = "json", data = "<query>")]
async fn bible_search(db: Connection<Bible>, query: Json<Query<'_>>) -> Value {
    match search::search(&query.query.to_string()) {
        Ok(bible_search) => {
            let result = db::search(db, bible_search).await;

            match result {
                Ok(result) => json!({"result": "ok", "data": result}),
                Err(err) => json!({"result": "error", "error": err.to_string()}),
            }
        }
        Err(err) => json!({"result": "error", "error": err.to_string()}),
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(Bible::init())
        .mount("/", routes![index, bible_search])
}
