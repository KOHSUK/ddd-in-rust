use crate::{
    infrastructure::database::shared::DATABASE_CONFIG,
    interface::repository::user::UserDatabaseTrait,
};

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use sqlx::{self, postgres, types::Uuid};

pub struct PostgresUserDatabase {}

#[async_trait]
impl UserDatabaseTrait for PostgresUserDatabase {
    type UserId = Uuid;
    type UserName = String;
    type UserData = (Self::UserId, Self::UserName);

    fn from_user_id(id: &Self::UserId) -> Result<String> {
        Ok(id.to_string())
    }
    fn from_user_name(name: &Self::UserName) -> Result<String> {
        Ok(name.to_owned())
    }
    fn from_user_data(user: &Self::UserData) -> Result<(String, String)> {
        Ok((user.0.to_string(), user.1.to_owned()))
    }

    fn to_user_id(value: &str) -> Result<Self::UserId> {
        Uuid::parse_str(value).map_err(|e| anyhow!(e.to_string()))
    }
    fn to_user_name(value: &str) -> Result<Self::UserName> {
        Ok(value.to_string())
    }
    fn to_user_data(id: &str, name: &str) -> Result<Self::UserData> {
        let id = Uuid::parse_str(id)?;
        Ok((id, name.to_string()))
    }

    async fn save(&self, user: &Self::UserData) -> Result<()> {
        let pool = postgres::PgPoolOptions::new()
            .max_connections(20)
            .connect(&DATABASE_CONFIG.database_url())
            .await?;

        let user_name = user.1.to_string();
        let user_id = user.0;

        sqlx::query(
            "
insert into public.user (id, name) values ($1, $2)
on conflict on constraint user_id_key
do
update set name = $2; 
            ",
        )
        .bind(user_id)
        .bind(user_name)
        .execute(&pool)
        .await?;

        Ok(())
    }

    async fn find(&self, user_name: &Self::UserName) -> Result<Self::UserData> {
        let pool = postgres::PgPoolOptions::new()
            .max_connections(20)
            .connect(&DATABASE_CONFIG.database_url())
            .await?;

        let data =
            sqlx::query_as::<_, Self::UserData>("select * from public.user where name = $1;")
                .bind(user_name)
                .fetch_one(&pool)
                .await?;

        Ok(data)
    }

    async fn delete(&self, user_id: &Self::UserId) -> Result<()> {
        let pool = postgres::PgPoolOptions::new()
            .max_connections(20)
            .connect(&DATABASE_CONFIG.database_url())
            .await?;

        sqlx::query("delete from public.user where id = $1")
            .bind(user_id)
            .execute(&pool)
            .await?;

        Ok(())
    }

    async fn find_by_id(&self, id: &Self::UserId) -> Result<Self::UserData> {
        let pool = postgres::PgPoolOptions::new()
            .max_connections(20)
            .connect(&DATABASE_CONFIG.database_url())
            .await?;

        let data =
            sqlx::query_as::<_, Self::UserData>("select * from public.user where id::text = $1;")
                .bind(id.to_string())
                .fetch_one(&pool)
                .await?;

        Ok(data)
    }
}

impl PostgresUserDatabase {
    pub fn new() -> anyhow::Result<PostgresUserDatabase> {
        Ok(PostgresUserDatabase {})
    }
}
