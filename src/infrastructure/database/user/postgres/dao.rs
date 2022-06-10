use std::sync::Arc;

use crate::interface::repository::user::UserDatabaseTrait;

use anyhow::{anyhow, Result};
use async_trait::async_trait;
use sqlx::{self, types::Uuid, Pool, Postgres};

pub struct PostgresUserDatabase {
    pool: Arc<Pool<Postgres>>,
}

#[async_trait]
impl UserDatabaseTrait for PostgresUserDatabase {
    type UserId = Uuid;
    type UserName = String;
    type UserIsPremium = bool;
    type UserData = (Self::UserId, Self::UserName, Self::UserIsPremium);

    fn from_user_id(id: &Self::UserId) -> Result<String> {
        Ok(id.to_string())
    }
    fn from_user_name(name: &Self::UserName) -> Result<String> {
        Ok(name.to_owned())
    }
    fn from_user_is_premium(is_premium: Self::UserIsPremium) -> Result<bool> {
        Ok(is_premium)
    }
    fn from_user_data(user: &Self::UserData) -> Result<(String, String, bool)> {
        Ok((user.0.to_string(), user.1.to_owned(), user.2))
    }

    fn to_user_id(value: &str) -> Result<Self::UserId> {
        Uuid::parse_str(value).map_err(|e| anyhow!(e.to_string()))
    }
    fn to_user_name(value: &str) -> Result<Self::UserName> {
        Ok(value.to_string())
    }
    fn to_user_is_premium(value: bool) -> Result<Self::UserIsPremium> {
        Ok(value)
    }
    fn to_user_data(id: &str, name: &str, is_premium: bool) -> Result<Self::UserData> {
        let id = Uuid::parse_str(id)?;
        Ok((id, name.to_string(), is_premium))
    }

    async fn save(&self, user: &Self::UserData) -> Result<()> {
        let mut conn = self.pool.acquire().await?;

        let user_name = user.1.to_string();
        let user_id = user.0;
        let is_premium = user.2;

        sqlx::query(
            "
insert into public.user (id, name, is_premium) values ($1, $2, $3)
on conflict on constraint user_id_key
do
update set name = $2, is_premium = $3; 
            ",
        )
        .bind(user_id)
        .bind(user_name)
        .bind(is_premium)
        .execute(&mut conn)
        .await?;

        Ok(())
    }

    async fn find(&self, user_name: &Self::UserName) -> Result<Self::UserData> {
        let mut conn = self.pool.acquire().await?;

        let data =
            sqlx::query_as::<_, Self::UserData>("select * from public.user where name = $1;")
                .bind(user_name)
                .fetch_one(&mut conn)
                .await?;

        Ok(data)
    }

    async fn delete(&self, user_id: &Self::UserId) -> Result<()> {
        let mut conn = self.pool.acquire().await?;

        sqlx::query("delete from public.user where id = $1")
            .bind(user_id)
            .execute(&mut conn)
            .await?;

        Ok(())
    }

    async fn find_by_id(&self, id: &Self::UserId) -> Result<Self::UserData> {
        let mut conn = self.pool.acquire().await?;

        let data =
            sqlx::query_as::<_, Self::UserData>("select * from public.user where id::text = $1;")
                .bind(id.to_string())
                .fetch_one(&mut conn)
                .await?;

        Ok(data)
    }

    async fn batch_find(&self, users: Vec<Self::UserId>) -> Result<Vec<Self::UserData>> {
        let mut conn = self.pool.acquire().await?;

        if users.is_empty() {
            return Ok(Vec::new());
        }

        let params = users
            .iter()
            .map(|u| format!("'{}'", u))
            .collect::<Vec<String>>()
            .join(", ");
        let query = format!("select * from public.user where id in ({})", params);
        let data = sqlx::query_as::<_, Self::UserData>(&query)
            .fetch_all(&mut conn)
            .await?;

        Ok(data)
    }
}

impl PostgresUserDatabase {
    pub fn new(pool: Arc<Pool<Postgres>>) -> anyhow::Result<PostgresUserDatabase> {
        Ok(PostgresUserDatabase { pool })
    }
}
