use crate::domain::model::club::entity::{Club, ClubId, ClubName};
use anyhow::Result;

use async_trait::async_trait;

#[async_trait]
pub trait ClubRepositoryTrait {
    async fn save(&self, club: &Club) -> Result<()>;
    async fn find_by_name(&self, club_name: &ClubName) -> Result<Option<Club>>;
    async fn find_by_id(&self, id: &ClubId) -> Result<Option<Club>>;
    async fn find_all(&self) -> Result<Vec<Club>>;
}
