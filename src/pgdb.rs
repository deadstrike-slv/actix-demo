use std::env;

use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use lazy_static::lazy_static;
use r2d2;

use crate::api::api_errors::ApiError;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!();

lazy_static! {
    static ref POOL: Pool = {
        let db_url = env::var("DATABASE_URL").expect("Database url not set");
        let manager = ConnectionManager::<PgConnection>::new(db_url);
        Pool::new(manager).expect("Failed to create db pool")
    };
}

pub fn init() {
    info!("Initializing DB");
    lazy_static::initialize(&POOL);
    let mut conn = connection().expect("Failed to get db connection");
    conn.run_pending_migrations(MIGRATIONS).unwrap();
}

pub fn connection() -> Result<DbConnection, ApiError> {
    POOL.get()
        .map_err(|e| ApiError::new(500, format!("Failed getting db connection: {}", e)))
}
