#[macro_use] extern crate rocket;

use std::sync::atomic::AtomicU64;
use api::post_handler;
use api::api_catchers;
use api::state::visit_counter::VisitorCounter;
use api::fairing::req_res_fairing::Counter;


const API_URL: &str = "/api";

#[launch]
fn rocket() -> _ {

    let visitor_counter = VisitorCounter {
        visitor: AtomicU64::new(0),
    };

    rocket::build()
        .manage(visitor_counter)
        .attach(Counter::new())
        .mount(API_URL, routes![
            post_handler::list_posts_handler,
            post_handler::list_post_handler,
            post_handler::get_filesystem_handler,
            post_handler::get_filesystem_handler_two,
            post_handler::create_post_handler,
        ])
        .register(API_URL, catchers![api_catchers::not_found])
}