pub mod models;
pub mod schema;


#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::{NewPost, Post};
    use diesel::prelude::*;
    use diesel::r2d2::{ConnectionManager, PooledConnection};
    use diesel::r2d2::Pool;
    use infrastructure::get_connection_pool;
    use rand::{thread_rng, Rng};
    use rand::distributions::Alphanumeric;


    #[test]
    fn test_create_and_list() {
        let connection_pool: Pool<ConnectionManager<PgConnection>> = get_connection_pool();
        let pooled_connection: &mut PooledConnection<ConnectionManager<PgConnection>> = &mut connection_pool.get().unwrap();

        let rand_string: String = thread_rng()
            .sample_iter(&Alphanumeric)
            .take(30)
            .map(char::from)
            .collect();

        let post: NewPost = NewPost{title: "unit test", body: &rand_string, genre: "T"};
        let r = pooled_connection.build_transaction().run::<Post, diesel::result::Error, _>(|con| {
            let result = diesel::insert_into(schema::posts::table).values(&post).get_result::<Post>(con);
            result
        });

        assert_eq!(post.title, r.unwrap().title);

        let posts = pooled_connection.build_transaction().read_only().run::<Vec<Post>, diesel::result::Error, _>(|con| {
            let posts = diesel::sql_query("SELECT id, title, body, genre, published FROM posts")
                .load::<Post>(con);
            posts
        });

        for post in posts.unwrap() {
            println!("post: {:?}", post);
        }

    }
}