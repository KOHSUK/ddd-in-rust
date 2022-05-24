use crate::domain::entity::user::model::{User, UserId, UserName};
use crate::domain::repository::user_repository::UserRepositoryInterface;
use crate::interface::repository::user_database::{UserDatabase};

use anyhow::Result;
use async_trait::async_trait;
use sqlx::types::Uuid;

pub struct UserRepository {
    database: Box<dyn UserDatabase + Sync + Send>,
}

#[async_trait]
impl UserRepositoryInterface for UserRepository {
    async fn save(&self, user: &User) -> Result<()> {
        let user_name = user.get_name().to_str().to_string();
        self.database.save(&user_name).await?;

        Ok(())
    }

    async fn update(&self, user: &User) -> Result<()> {
        let id = user.get_id();
        let id = Uuid::parse_str(id.to_str())?;
        let name = user.get_name().to_str().to_string();
        let user = (id, name);
        self.database.update(&user).await?;

        Ok(())
    }

    async fn find_by_name(&self, user_name: &UserName) -> Option<User> {
        let user_name = user_name.to_str().to_string();
        let row = self.database.find(&user_name).await;

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

        match User::new(user_id, user_name) {
            Ok(user) => Some(user),
            Err(_) => None,
        }
    }

    async fn delete(&self, id: &UserId) -> Result<()> {
        let id = id.to_str();
        let id = Uuid::parse_str(id)?;

        self.database.delete(&id).await
    }

    async fn find_by_id(&self, id: &UserId) -> Option<User> {
        let id = id.to_str();
        let id = Uuid::parse_str(id);

        if id.is_err() {
            return None;
        }

        let row = self.database.find_by_id(&id.unwrap()).await;

        if let Err(e) = row {
            eprintln!("Error: {}", e);
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

        match User::new(user_id, user_name) {
            Ok(user) => Some(user),
            Err(_) => None,
        }
    }
}

impl UserRepository {
    pub async fn new(database: Box<dyn UserDatabase + Sync + Send>) -> anyhow::Result<UserRepository> {
        let repo = UserRepository { database };

        Ok(repo)
    }
}
