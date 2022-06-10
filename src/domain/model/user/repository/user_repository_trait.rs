use crate::domain::model::user::entity::{User, UserId, UserName};
use anyhow::Result;

use async_trait::async_trait;

#[async_trait]
pub trait UserRepositoryTrait {
    async fn save(&self, user: &User) -> Result<()>;
    async fn find_by_name(&self, user_name: &UserName) -> Result<Option<User>>;
    async fn find_by_id(&self, id: &UserId) -> Result<Option<User>>;
    async fn delete(&self, id: &UserId) -> Result<()>;
    async fn batch_find(&self, users: Vec<UserId>) -> Result<Vec<User>>;
}
