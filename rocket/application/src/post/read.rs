use domain::models::Post;
use shared::response_dto::{Response, ResponseBody};
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use rocket::response::status::NotFound;

use crate::conn_pool::get_connection;

pub fn list_post(post_id: i32) -> Result<Post, NotFound<String>> {
    use domain::schema::posts;

    let conn: &mut PooledConnection<ConnectionManager<PgConnection>> = &mut get_connection();

    match posts::table.find(post_id).first::<Post>(conn) {
        Ok(post) => Ok(post),
        Err(err) => match err {
            diesel::result::Error::NotFound => {
                let response = Response { body: ResponseBody::Message(format!("Error selecting post with id {} - {}", post_id, err))};
                return Err(NotFound(serde_json::to_string(&response).unwrap()));
            },
            _ => {
                panic!("Database error - {}", err);
            }
        }
    }
}

pub fn list_posts() -> Vec<Post> {
    use domain::schema::posts;

    let conn: &mut PooledConnection<ConnectionManager<PgConnection>> = &mut get_connection();

    match posts::table.select(posts::all_columns).load::<Post>(conn) {
        Ok(mut posts) => {
            posts.sort();
            posts
        },
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        }
    }
}