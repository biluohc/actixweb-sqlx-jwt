use super::user::*;
use crate::state::AppState;

use sqlx::Done;

#[async_trait]
pub trait IUser {
    async fn user_add(&self, form: &Register) -> sqlx::Result<u64>;
    async fn user_query(&self, name: &str) -> sqlx::Result<User>;
}

#[cfg(any(feature = "mysql", feature = "sqlite"))]
#[async_trait]
impl IUser for AppState {
    async fn user_add(&self, form: &Register) -> sqlx::Result<u64> {
        let passh = form.passhash();

        sqlx::query!(
            r#"
        INSERT INTO users (name, email, pass)
        VALUES (?, ?, ?)
                "#,
            form.name,
            form.email,
            passh
        )
        .execute(&self.sql)
        .await
        .map(|d| d.rows_affected())
    }
    async fn user_query(&self, name: &str) -> sqlx::Result<User> {
        sqlx::query_as!(
            User,
            r#"
        SELECT id, name, email, pass, create_dt, update_dt
        FROM users
        where name = ?
                "#,
            name
        )
        .fetch_one(&self.sql)
        .await
    }
}

#[cfg(any(feature = "postgres"))]
#[async_trait]
impl IUser for AppState {
    async fn user_add(&self, form: &Register) -> sqlx::Result<u64> {
        let passh = form.passhash();

        sqlx::query!(
            r#"
        INSERT INTO users (name, email, pass)
        VALUES ($1 ,$2 ,$3)
                "#,
            form.name,
            form.email,
            passh
        )
        .execute(&self.sql)
        .await
        .map(|d| d.rows_affected())
    }
    async fn user_query(&self, name: &str) -> sqlx::Result<User> {
        sqlx::query_as!(
            User,
            r#"
        SELECT id, name, email, pass, create_dt, update_dt
        FROM users
        where name = $1
                "#,
            name
        )
        .fetch_one(&self.sql)
        .await
    }
}
