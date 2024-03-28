use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::time::Duration;

pub type DatabasePool = Pool<Postgres>;

const DB_MAX_CONN: u32 = 10;
const DB_CONN_TIMEOUT_SEC: u64 = 3;

#[derive(Clone)]
pub struct AppState {
    pub single_user_id: uuid::Uuid,
    pub database_pool: DatabasePool,
    pub single_user_mode: bool
}


pub async fn get_db_pool(conn_string: String) -> Result<DatabasePool, sqlx::Error> {
    let dur: Duration = Duration::new(DB_CONN_TIMEOUT_SEC, 0);
    PgPoolOptions::new()
        .max_connections(DB_MAX_CONN)
        .acquire_timeout(dur)
        .connect(&conn_string)
        .await
}
