use super::user::*;
use crate::state::AppStateRaw;

#[async_trait]
pub trait IUser: std::ops::Deref<Target = AppStateRaw> {
    async fn user_add(&self, form: &Register) -> sqlx::Result<u64>;
    async fn user_query(&self, who: &str) -> sqlx::Result<User> {
        let (column, placeholder) = column_placeholder(who);

        let sql = format!(
            "SELECT id, name, email, pass, status, create_dt, update_dt
            FROM users
            where {} = {};",
            column, placeholder
        );

        sqlx::query_as(&sql).bind(who).fetch_one(&self.sql).await
    }
    async fn user_delete(&self, who: &str) -> sqlx::Result<User> {
        let (column, placeholder) = column_placeholder(who);

        let sql = format!(
            "update users set status='deleted' where {}={} RETURNING *;",
            column, placeholder
        );

        sqlx::query_as(&sql).bind(who).fetch_one(&self.sql).await
    }
}

#[cfg(any(feature = "mysql", feature = "sqlite"))]
#[async_trait]
impl IUser for &AppStateRaw {
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
    #[cfg(any(feature = "mysql"))]
    async fn user_delete(&self, who: &str) -> sqlx::Result<User> {
        let (column, placeholder) = column_placeholder(who);

        // mysql doesn't have RETURNING
        let sql = format!(
            "update users set status='deleted' where {}={};",
            column, placeholder
        );

        sqlx::query(&sql).bind(who).execute(&self.sql).await?;
        self.user_query(who).await
    }
}

#[cfg(any(feature = "postgres"))]
#[async_trait]
impl IUser for &AppStateRaw {
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
}

fn column_placeholder(id_or_name_or_email: &str) -> (&'static str, &'static str) {
    let mut column = "name";

    if id_or_name_or_email.contains("@") {
        column = "email";
    } else if first_char_is_number(id_or_name_or_email) {
        column = "id";
    }

    // postgres: $1, $2 ..
    // mysql/sqlite: ?, ? ..
    let placeholder = if cfg!(feature = "postgres") {
        "$1"
    } else {
        "?"
    };

    (column, placeholder)
}
