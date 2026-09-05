use diesel::r2d2::{self, ConnectionManager};
use diesel::PgConnection;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub async fn setup_database_with_migration() -> DbPool {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set!");

    let manager = ConnectionManager::<PgConnection>::new(&database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create database pool")
}
