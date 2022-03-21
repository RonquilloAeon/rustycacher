use std::io;

#[macro_use] extern crate rocket;

use rocket::data::{Data, ToByteUnit};

use cacher::Cacher;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[post("/", data = "<data>")]
async fn cache(data: Data<'_>) -> &'static str {
    let input = data.open(512.kibibytes()).into_bytes().await.unwrap();

    let db_url = std::env::var("DB_URL").ok().unwrap();

    let mut cacher = Cacher::new(db_url);
    let exists = cacher.exists_or_save(&input);

    match exists {
        true => "true",
        _ => "false"
    }
}

#[rocket::main]
async fn main() {
    rocket::build()
        .mount("/hello", routes![index])
        .mount("/cache", routes![cache])
        .launch()
        .await;
}
