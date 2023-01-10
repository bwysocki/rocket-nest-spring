use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use rocket::response::content;
use rocket::response::status::Created;
use rocket::serde::json::Json;

use domain::models::{NewPost, Post};
use shared::response_dto::{Response, ResponseBody};

use crate::conn_pool::get_connection;

pub fn create_post(post: Json<NewPost>) -> content::RawJson<Created<String>> {
    use domain::schema::posts;

    let post = post.into_inner();

    let conn: &mut PooledConnection<ConnectionManager<PgConnection>> = &mut get_connection();

    match diesel::insert_into(posts::table).values(&post).get_result::<Post>(conn) {
        Ok(post) => {
            let response = Response { body: ResponseBody::Post(post) };
            content::RawJson(Created::new("").tagged_body(serde_json::to_string(&response).unwrap()))
        },
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        }
    }
}