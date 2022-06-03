use async_trait::async_trait;
use anyhow::Result;
use sqlx::types::Uuid;

pub type UserId = Uuid;
pub type UserName = String;
pub type UserData = (UserId, UserName);

#[async_trait]
pub trait UserDatabaseTrait {
    async fn save(&self, user: &UserData) -> Result<()>;
    async fn find(&self, user_name: &UserName) -> Result<UserData>;
    async fn find_by_id(&self, id: &UserId) -> Result<UserData>;
    async fn delete(&self, id: &UserId) -> Result<()>;
    async fn update(&self, user: &UserData) -> Result<()>;
}