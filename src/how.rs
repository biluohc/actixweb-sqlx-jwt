use mobc_redis::redis::RedisError;
use sqlx::Error as SqlxError;
use tokio::time::Elapsed;

pub use anyhow::Error as AnyError;
pub use anyhow::Result as AnyResult;

pub type Result<T> = std::result::Result<T, Error>;

// https://docs.rs/anyhow
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Mobc error: {0}")]
    Mobc(#[from] mobc::Error<RedisError>),
    #[error("Redis error: {0}")]
    Redis(#[from] RedisError),
    #[error("Sqlx error: {0}")]
    Sqlx(#[from] SqlxError),
    #[error("Timout error: {0}")]
    Timout(#[from] Elapsed),
}
