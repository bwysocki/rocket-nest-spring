use domain::models::Post;
use shared::response_dto::{Response, ResponseBody};
use infrastructure::get_connection_pool;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use diesel::r2d2::Pool;
use rocket::response::status::NotFound;



pub fn list_post(post_id: i32) -> Result<Post, NotFound<String>> {
    use domain::schema::posts;

    let connection_pool: Pool<ConnectionManager<PgConnection>> = get_connection_pool();
    let pooled_connection: &mut PooledConnection<ConnectionManager<PgConnection>> = &mut connection_pool.get().unwrap();

    match posts::table.find(post_id).first::<Post>(pooled_connection) {
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

    let connection_pool: Pool<ConnectionManager<PgConnection>> = get_connection_pool();
    let pooled_connection: &mut PooledConnection<ConnectionManager<PgConnection>> = &mut connection_pool.get().unwrap();

    match posts::table.select(posts::all_columns).load::<Post>(pooled_connection) {
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