#[macro_use] extern crate rocket;
use api::post_handler;

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/api", routes![
            post_handler::list_posts_handler,
            post_handler::list_post_handler,
            post_handler::get_filesystem_handler,
            post_handler::get_filesystem_handler_two,
            post_handler::create_post_handler,
        ])
}