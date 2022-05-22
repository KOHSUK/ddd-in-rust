use crate::domain::entity::user::{User, UserId, UserName};
use anyhow::Result;

use async_trait::async_trait;

#[async_trait]
pub trait UserRepositoryInterface {
    async fn save(&self, user: &User) -> Result<()>;
    async fn update(&self, user: &User) -> Result<()>;
    async fn find_by_name(&self, user_name: &UserName) -> Option<User>;
    async fn find_by_id(&self, id: &UserId) -> Option<User>;
    async fn delete(&self, id: &UserId) -> Result<()>;
}
