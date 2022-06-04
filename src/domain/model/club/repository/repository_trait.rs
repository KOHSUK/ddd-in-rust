use crate::domain::model::club::entity::{Club, ClubId, ClubName};
use anyhow::Result;

use async_trait::async_trait;

#[async_trait]
pub trait ClubRepositoryTrait {
    async fn save(&self, user: &Club) -> Result<()>;
    async fn find_by_name(&self, user_name: &ClubName) -> Option<Club>;
    async fn find_by_id(&self, id: &ClubId) -> Option<Club>;
    async fn delete(&self, id: &ClubId) -> Result<()>;
}
