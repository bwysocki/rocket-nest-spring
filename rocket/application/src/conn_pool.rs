use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use lazy_static::lazy_static;

use infrastructure::get_connection_pool;

lazy_static! {
    static ref CONNECTION_POOL: Pool<ConnectionManager<PgConnection>> = get_connection_pool();
}

pub fn get_connection() -> PooledConnection<ConnectionManager<PgConnection>> {
    CONNECTION_POOL.get().unwrap()
}