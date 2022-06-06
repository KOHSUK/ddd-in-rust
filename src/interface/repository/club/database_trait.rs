use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait ClubDatabaseTrait {
    type ClubId: Send + Sync;
    type ClubName: Send + Sync;
    type ClubData: Send + Sync;

    fn from_club_id(id: &Self::ClubId) -> Result<String>;
    fn from_club_name(name: &Self::ClubName) -> Result<String>;
    fn from_club_data(club: &Self::ClubData) -> Result<(String, String, String)>;

    fn to_club_id(value: &str) -> Result<Self::ClubId>;
    fn to_club_name(value: &str) -> Result<Self::ClubName>;
    fn to_club_data(id: &str, name: &str, owner_id: &str) -> Result<Self::ClubData>;

    async fn save(&self, club: &Self::ClubData) -> Result<()>;
    async fn find_by_name(&self, club_name: &Self::ClubName) -> Result<Self::ClubData>;
    async fn find_by_id(&self, id: &Self::ClubId) -> Result<Self::ClubData>;
}
