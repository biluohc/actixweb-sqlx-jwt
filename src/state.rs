pub use mobc_redis::{redis, RedisConnectionManager};
pub type Connection = mobc::Connection<RedisConnectionManager>;
pub type KvPool = mobc::Pool<RedisConnectionManager>;

#[cfg(any(feature = "mysql"))]
pub type DbPool = sqlx::MySqlPool;
#[cfg(any(feature = "mysql"))]
pub type PoolOptions = sqlx::mysql::MySqlPoolOptions;

#[cfg(any(feature = "sqlite"))]
pub type DbPool = sqlx::SqlitePool;
#[cfg(any(feature = "sqlite"))]
pub type PoolOptions = sqlx::sqlite::SqlitePoolOptions;

#[cfg(any(feature = "postgres"))]
pub type DbPool = sqlx::PgPool;
#[cfg(any(feature = "postgres"))]
pub type PoolOptions = sqlx::postgres::PgPoolOptions;

use crate::config::Config;

#[derive(Clone)]
pub struct State {
    pub config: Config,
    pub db: DbPool,
    pub kv: KvPool,
}

pub type AppState = std::sync::Arc<State>;

// global state for slavo
static mut STATE: Option<AppState> = None;

pub fn global_state_init(s: AppState) {
    unsafe {
        assert!(STATE.is_none(), "global state initialized twice");
        STATE = Some(s);
    }
    global_state();
}

pub fn global_state() -> &'static AppState {
    unsafe { STATE.as_ref().expect("global state uninitialized") }
}
