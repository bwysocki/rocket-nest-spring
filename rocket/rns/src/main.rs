#[macro_use] extern crate rocket;

use rocket::response::content;

//#[route(GET, uri = "/user/<uuid>", rank = 1, format = "text/plain")]
#[get("/rust-filesystem-read?<extra>", rank = 1, format = "text/plain")]
fn index(extra: &str) -> content::RawJson<&'static str> {
    let r = format!("Hello, world! {}", extra).as_str();
    content::RawJson("{ 'hi': 'world' }")
}

#[rocket::main]
async fn main() {
    rocket::build().mount("/", routes![index]).launch().await;
}
