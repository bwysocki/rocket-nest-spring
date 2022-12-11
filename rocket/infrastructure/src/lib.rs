extern crate diesel;
use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use dotenvy::dotenv;
use std::env;

pub fn get_connection_pool() -> Pool<ConnectionManager<PgConnection>> {
    dotenv().ok(); //initialize env

    // Pull DATABASE_URL env var
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set.");

    // Create a connection pool manager for a Postgres connection at the `database_url`
    let manager = ConnectionManager::<PgConnection>::new(database_url);

    // Create the pool with the default config and the r2d2 connection manager
    Pool::builder()
        .test_on_check_out(true)
        .max_size(50)
        .build(manager)
        .expect("Could not build connection pool")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_creates_connection() {
        let connection_pool = get_connection_pool();
        let pooledConnection = connection_pool.get().unwrap();
    }
}
