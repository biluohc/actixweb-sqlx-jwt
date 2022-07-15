use super::user::*;
use crate::state::AppStateRaw;

#[async_trait]
pub trait IUser: std::ops::Deref<Target = AppStateRaw> {
    async fn adress_add(&self, address: &str, experience: &str) -> sqlx::Result<u64>
    {
        sqlx::query!(
            r#"
        INSERT INTO user_address2 (address, experience)
       VALUES ($1 ,$2)
                "#,
            address,
            experience
        )
            .execute(&self.sql)
            .await
            .map(|d| d.rows_affected())
    }

   async fn adress_query(&self, address: &str) -> sqlx::Result<AddressExperience> {

        let sql = format!(
            "SELECT address, experience
            FROM user_address2
            where address = '{}';",
            address
        );
        sqlx::query_as::<_, AddressExperience>(&sql).bind(address).fetch_one(&self.sql).await
    }

    async fn adress_update(&self, address: &str, experience: &str) ->sqlx::Result<u64> {

        let sql = format!(
            "update user_address2 set experience='{}' where address='{}';",
            experience, address
        );

        sqlx::query(&sql).bind(address).execute(&self.sql).await.map(|d| d.rows_affected())
    }
}

#[cfg(any(feature = "postgres"))]
#[async_trait]
impl IUser for &AppStateRaw {

}

