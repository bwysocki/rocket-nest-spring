use domain::models::{Post, NewPost};
use shared::response_dto::{Response, ResponseBody};
use infrastructure::get_connection_pool;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use rocket::response::status::Created;
use rocket::serde::json::Json;

pub fn create_post(post: Json<NewPost>) -> Created<String> {
    use domain::schema::posts;

    let post = post.into_inner();

    let connection_pool: Pool<ConnectionManager<PgConnection>> = get_connection_pool();
    let pooled_connection: &mut PooledConnection<ConnectionManager<PgConnection>> = &mut connection_pool.get().unwrap();

    match diesel::insert_into(posts::table).values(&post).get_result::<Post>(pooled_connection) {
        Ok(post) => {
            let response = Response { body: ResponseBody::Post(post) };
            Created::new("").tagged_body(serde_json::to_string(&response).unwrap())
        },
        Err(err) => match err {
            _ => {
                panic!("Database error - {}", err);
            }
        }
    }
}