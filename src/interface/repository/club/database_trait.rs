use anyhow::Result;
use async_trait::async_trait;

pub type PrimitiveId = String;
pub type PrimitiveName = String;
pub type PrimitiveMembers = Vec<String>;
pub type PrimitiveOwner = String;

#[async_trait]
pub trait ClubDatabaseTrait {
    type ClubId: Send + Sync;
    type ClubName: Send + Sync;
    type ClubMembers: Send + Sync;
    type ClubOwner: Send + Sync;
    type ClubData: Send + Sync;

    fn from_club_id(id: &Self::ClubId) -> Result<PrimitiveId>;
    fn from_club_name(name: &Self::ClubName) -> Result<PrimitiveName>;
    fn from_club_owner(owner: &Self::ClubOwner) -> Result<PrimitiveOwner>;
    fn from_club_members(members: &Self::ClubMembers) -> Result<PrimitiveMembers>;
    fn from_club_data(
        club: &Self::ClubData,
    ) -> Result<(PrimitiveId, PrimitiveName, PrimitiveOwner, PrimitiveMembers)>;

    fn to_club_id(value: &PrimitiveId) -> Result<Self::ClubId>;
    fn to_club_name(value: &PrimitiveName) -> Result<Self::ClubName>;
    fn to_club_owner(value: &PrimitiveOwner) -> Result<Self::ClubOwner>;
    fn to_club_members(members: &PrimitiveMembers) -> Result<Self::ClubMembers>;
    fn to_club_data(
        id: &PrimitiveId,
        name: &PrimitiveName,
        owner_id: &PrimitiveOwner,
        members: &PrimitiveMembers,
    ) -> Result<Self::ClubData>;

    async fn save(&self, club: &Self::ClubData) -> Result<()>;
    async fn find_by_name(&self, club_name: &Self::ClubName) -> Result<Self::ClubData>;
    async fn find_by_id(&self, id: &Self::ClubId) -> Result<Self::ClubData>;
    async fn find_all(&self) -> Result<Vec<Self::ClubData>>;
}
