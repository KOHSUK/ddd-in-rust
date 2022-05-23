use async_trait::async_trait;
use anyhow::Result;
use sqlx::types::Uuid;

pub type UserId = Uuid;
pub type UserName = String;
pub type UserData = (UserId, UserName);

#[async_trait]
pub trait UserDatabase {
    async fn save(&self, user_name: &UserName) -> Result<()>;
    async fn find(&self, user_name: &UserName) -> Result<UserData>;
    async fn find_by_id(&self, id: &UserId) -> Result<UserData>;
    async fn delete(&self, id: &UserId) -> Result<()>;
    async fn update(&self, user: &UserData) -> Result<()>;
}