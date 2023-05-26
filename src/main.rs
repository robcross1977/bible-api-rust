use rocket::serde::json::{json, Json, Value};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

mod book;
mod chapter;
mod params;
mod search;
mod verse;

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Welcome to the rust bible api!"
}

#[derive(Serialize, Deserialize)]
struct Query<'a> {
    query: Cow<'a, str>,
}

#[post("/search", format = "json", data = "<query>")]
fn bible_search(query: Json<Query<'_>>) -> Value {
    match search::search(&query.query.to_string()) {
        Ok(result) => {
            json!({
                "status": "success",
                "data": result
            })
        }
        Err(err) => panic!("Problem parsing arguments: {}", err),
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, bible_search])
}
