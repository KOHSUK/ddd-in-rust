use crate::interface::repository::user_database::{UserId, UserName, UserData, UserDatabase};

use anyhow::Result;
use async_trait::async_trait;
use sqlx::{self, postgres};
use once_cell::sync::Lazy;

#[derive(Debug)]
pub struct Config {
    postgres_host: String,
    postgres_port: String,
    postgres_user: String,
    postgres_password: String,
    postgres_database: String,
}

impl Config {
    pub fn database_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.postgres_user,
            self.postgres_password,
            self.postgres_host,
            self.postgres_port,
            self.postgres_database,
        )
    }
}

pub static CONFIG: Lazy<Config> = Lazy::new(|| Config {
    postgres_host: std::env::var("POSTGRES_HOSTNAME").unwrap(),
    postgres_port: std::env::var("POSTGRES_PORT").unwrap(),
    postgres_user: std::env::var("POSTGRES_USER").unwrap(),
    postgres_password: std::env::var("POSTGRES_PASSWORD").unwrap(),
    postgres_database: std::env::var("POSTGRES_DB").unwrap(),
});

pub struct PostgresUserRepository {
}

#[async_trait]
impl UserDatabase for PostgresUserRepository {
    async fn save(&self, user_name: &UserName) -> Result<()> {
        let pool = postgres::PgPoolOptions::new()
            .max_connections(20)
            .connect(&CONFIG.database_url())
            .await?;

        sqlx::query("insert into public.user (name) values ($1);")
            .bind(user_name)
            .execute(&pool)
            .await?;

        Ok(())
    }

    async fn find(&self, user_name: &UserName) -> Result<UserData> {
        let pool = postgres::PgPoolOptions::new()
            .max_connections(20)
            .connect(&CONFIG.database_url())
            .await?;

        let data = sqlx::query_as::<_, UserData>("select * from public.user where name = $1;")
            .bind(user_name)
            .fetch_one(&pool)
            .await?;

        Ok(data)
    }

    async fn delete(&self, _: &UserData) -> Result<()> {
        let _ = postgres::PgPoolOptions::new()
            .max_connections(20)
            .connect(&CONFIG.database_url())
            .await?;

        unimplemented!();
    }

    async fn find_by_id(&self, id: &UserId) -> Result<UserData> {
        let pool = postgres::PgPoolOptions::new()
            .max_connections(20)
            .connect(&CONFIG.database_url())
            .await?;

        let data =
            sqlx::query_as::<_, UserData>("select * from public.user where id = $1;")
                .bind(id.to_string())
                .fetch_one(&pool)
                .await?;

        Ok(data)
    }
}

impl PostgresUserRepository {
    pub fn new() -> anyhow::Result<PostgresUserRepository> {
        Ok(PostgresUserRepository { })
    }
}
