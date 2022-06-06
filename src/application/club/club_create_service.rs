use anyhow::{anyhow, Result};
use std::sync::{Arc, Mutex};

use crate::domain::model::{
    club::{
        entity::ClubName, factory::ClubFactoryTrait, repository::ClubRepositoryTrait,
        service::ClubService,
    },
    user::{entity::UserId, repository::UserRepositoryTrait},
};

pub struct ClubCreateService<'a> {
    club_repository: Arc<Mutex<dyn ClubRepositoryTrait>>,
    club_factory: Arc<dyn ClubFactoryTrait>,
    club_service: Arc<ClubService<'a>>,
    user_repository: Arc<dyn UserRepositoryTrait>,
}

pub struct ClubCreateCommand {
    user_id: String,
    name: String,
}

impl ClubCreateCommand {
    pub fn new(user_id: &str, name: &str) -> Self {
        Self {
            user_id: user_id.to_string(),
            name: name.to_string(),
        }
    }
}

impl<'a> ClubCreateService<'a> {
    pub fn new(
        club_repository: Arc<Mutex<dyn ClubRepositoryTrait>>,
        club_factory: Arc<dyn ClubFactoryTrait>,
        club_service: Arc<ClubService<'a>>,
        user_repository: Arc<dyn UserRepositoryTrait>,
    ) -> Self {
        Self {
            club_repository,
            club_factory,
            club_service,
            user_repository,
        }
    }

    pub async fn handle(&self, command: ClubCreateCommand) -> Result<()> {
        let user_id = UserId::new(&command.user_id)?;
        let user_repo = Arc::clone(&self.user_repository);
        let owner = user_repo
            .find_by_id(&user_id)
            .await?
            .ok_or_else(|| anyhow!("User not found"))?;

        let name = ClubName::new(&command.name)?;
        let club_factory = Arc::clone(&self.club_factory);
        let club = club_factory.create(name, owner)?;

        let club_service = Arc::clone(&self.club_service);
        if club_service.exists(&club).await {
            return Err(anyhow!("Club already exists"));
        }

        let club_repo = Arc::clone(&self.club_repository);
        let club_repo = club_repo.lock().unwrap();

        club_repo.save(&club).await
    }
}
