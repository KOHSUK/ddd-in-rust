use anyhow::{anyhow, Result};
use std::sync::{Arc, Mutex};

use crate::domain::model::{
    club::{
        entity::ClubId, factory::ClubFactoryTrait, repository::ClubRepositoryTrait,
        service::ClubService,
    },
    user::{entity::UserId, repository::UserRepositoryTrait},
};

pub struct ClubJoinCommand {
    user_id: String,
    club_id: String,
}

impl ClubJoinCommand {
    pub fn new(user_id: &str, club_id: &str) -> Self {
        Self {
            user_id: user_id.to_string(),
            club_id: club_id.to_string(),
        }
    }
}

pub struct ClubJoinService {
    club_repository: Arc<Mutex<dyn ClubRepositoryTrait>>,
    club_factory: Arc<dyn ClubFactoryTrait>,
    club_service: Arc<ClubService>,
    user_repository: Arc<dyn UserRepositoryTrait>,
}

impl<'a> ClubJoinService {
    pub fn new(
        club_repository: Arc<Mutex<dyn ClubRepositoryTrait>>,
        club_factory: Arc<dyn ClubFactoryTrait>,
        club_service: Arc<ClubService>,
        user_repository: Arc<dyn UserRepositoryTrait>,
    ) -> Self {
        Self {
            club_repository,
            club_factory,
            club_service,
            user_repository,
        }
    }

    pub async fn handle(&self, command: ClubJoinCommand) -> Result<()> {
        let member_id = UserId::new(&command.user_id)?;
        let user_repo = Arc::clone(&self.user_repository);
        let user = user_repo
            .find_by_id(&member_id)
            .await?
            .ok_or_else(|| anyhow!("Could not find the user"))?;

        let club_id = ClubId::new(&command.club_id)?;
        let club_repo = Arc::clone(&self.club_repository);
        let club_repo = club_repo.lock().unwrap();
        let mut club = club_repo
            .find_by_id(&club_id)
            .await?
            .ok_or_else(|| anyhow!("Could not find the club"))?;

        club.join(user)?;

        club_repo.save(&club).await
    }
}
