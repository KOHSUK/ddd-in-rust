use crate::domain::entity::user::{User, UserId, UserName};
use crate::infrastructure::database::CONFIG;
use anyhow::Result;

use async_trait::async_trait;
use sqlx::types::Uuid;
use sqlx::{self, postgres, Pool, Postgres};

#[async_trait]
pub trait UserRepositoryInterface {
    async fn save(&self, user: &User) -> Result<()>;
    async fn find(&self, user_name: &UserName) -> Option<User>;
    async fn find_by_id(&self, id: &UserId) -> Option<User>;
    async fn delete(&self, user: &User) -> Result<()>;
}

pub struct UserRepository {
    pool: Pool<Postgres>,
}

type Row = (Uuid, String);

#[async_trait]
impl UserRepositoryInterface for UserRepository {
    async fn save(&self, user: &User) -> Result<()> {
        sqlx::query("insert into public.user (name) values ($1);")
            .bind(user.get_name().to_str())
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn find(&self, user_name: &UserName) -> Option<User> {
        let row =
            sqlx::query_as::<_, Row>("select * from public.user where name = $1;")
                .bind(user_name.to_str())
                .fetch_one(&self.pool)
                .await;

        if row.is_err() {
            return None;
        }

        let row = row.unwrap();

        let user_id = UserId::new(&row.0.to_string());
        let user_name = UserName::new(&row.1);

        if user_id.is_err() || user_name.is_err() {
            return None;
        }

        let user_id = user_id.unwrap();
        let user_name = user_name.unwrap();

        match User::new_with_id(user_id, user_name) {
            Ok(user) => Some(user),
            Err(_) => None,
        }
    }

    async fn delete(&self, user: &User) -> Result<()> {
        unimplemented!();
    }

    async fn find_by_id(&self, id: &UserId) -> Option<User> {
        unimplemented!();
    }
}

impl UserRepository {
    pub async fn new() -> anyhow::Result<UserRepository> {
        let pool = postgres::PgPoolOptions::new()
            .max_connections(20)
            .connect(&CONFIG.database_url())
            .await?;

        let repo = UserRepository { pool };

        Ok(repo)
    }
}
