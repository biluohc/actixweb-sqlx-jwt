pub use mobc_redis::{redis, RedisConnectionManager};
pub type Connection = mobc::Connection<RedisConnectionManager>;
pub type KvPool = mobc::Pool<RedisConnectionManager>;

#[cfg(any(feature = "mysql"))]
pub type SqlPool = sqlx::MySqlPool;
#[cfg(any(feature = "mysql"))]
pub type PoolOptions = sqlx::mysql::MySqlPoolOptions;

#[cfg(any(feature = "sqlite"))]
pub type SqlPool = sqlx::SqlitePool;
#[cfg(any(feature = "sqlite"))]
pub type PoolOptions = sqlx::sqlite::SqlitePoolOptions;

#[cfg(any(feature = "postgres"))]
pub type SqlPool = sqlx::PgPool;
#[cfg(any(feature = "postgres"))]
pub type PoolOptions = sqlx::postgres::PgPoolOptions;

use crate::config::Config;

#[derive(Clone)]
pub struct State {
    pub config: Config,
    pub sql: SqlPool,
    pub kv: KvPool,
}

pub type AppStateRaw = std::sync::Arc<State>;
pub type AppState = actix_web::web::Data<AppStateRaw>;
