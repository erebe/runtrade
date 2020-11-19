use diesel::PgConnection;
use diesel::r2d2::ConnectionManager;
use r2d2::Pool;

pub mod model;
pub mod insertables;
pub mod operations;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub fn get_postgres_connection_pool(database_url: &str) -> Result<DbPool, r2d2::Error> {
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::builder().build(manager)
}