use anyhow::Result;
use async_trait::async_trait;

use crate::domain::model::{
    club::{
        entity::{Club, ClubId, ClubName},
        repository::ClubRepositoryTrait,
    },
    user::entity::UserId,
};

use super::database_trait::ClubDatabaseTrait;

#[async_trait]
pub trait ClubDatabaseTraitWrapper {
    async fn save(&self, club: &Club) -> Result<()>;
    async fn find_by_name(&self, club_name: &ClubName) -> Result<Option<Club>>;
    async fn find_by_id(&self, id: &ClubId) -> Result<Option<Club>>;
}

#[async_trait]
impl<D: ClubDatabaseTrait + Send + Sync> ClubDatabaseTraitWrapper for D {
    async fn save(&self, club: &Club) -> Result<()> {
        let club = D::to_club_data(
            &club.get_id().to_string(),
            &club.get_name().to_string(),
            &club.get_owner_id().to_string(),
            &club
                .get_members()
                .iter()
                .map(|m| m.to_string())
                .collect::<Vec<String>>(),
        )?;
        self.save(&club).await
    }

    async fn find_by_name(&self, club_name: &ClubName) -> Result<Option<Club>> {
        let club_name = D::to_club_name(&club_name.to_string())?;
        let club = self.find_by_name(&club_name).await?;
        let club = D::from_club_data(&club)?;

        let club_id = ClubId::new(&club.0)?;
        let club_name = ClubName::new(&club.1)?;
        let user_id = UserId::new(&club.2)?;
        let club = Club::new(club_id, club_name, Vec::new(), user_id)?;

        Ok(Some(club))
    }

    async fn find_by_id(&self, club_id: &ClubId) -> Result<Option<Club>> {
        let club_id = D::to_club_id(&club_id.to_string())?;
        let club = self.find_by_id(&club_id).await?;
        let club = D::from_club_data(&club)?;

        let club_id = ClubId::new(&club.0)?;
        let club_name = ClubName::new(&club.1)?;
        let user_id = UserId::new(&club.2)?;
        let member = club
            .3
            .iter()
            .map(|u| UserId::new(u).unwrap())
            .collect::<Vec<UserId>>();
        let club = Club::new(club_id, club_name, member, user_id)?;

        Ok(Some(club))
    }
}

pub struct ClubRepository {
    database: Box<dyn ClubDatabaseTraitWrapper + Send + Sync>,
}

#[async_trait]
impl ClubRepositoryTrait for ClubRepository {
    async fn save(&self, club: &Club) -> Result<()> {
        self.database.save(club).await
    }
    async fn find_by_name(&self, club_name: &ClubName) -> Result<Option<Club>> {
        self.database.find_by_name(club_name).await
    }
    async fn find_by_id(&self, id: &ClubId) -> Result<Option<Club>> {
        self.database.find_by_id(id).await
    }
}

impl ClubRepository {
    pub async fn new(database: Box<dyn ClubDatabaseTraitWrapper + Send + Sync>) -> Result<Self> {
        Ok(Self { database })
    }
}
