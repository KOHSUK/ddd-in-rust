use crate::domain::model::user::{entity::UserId, repository::UserRepositoryTrait};

use anyhow::{anyhow, Result};
use std::sync::{Arc, Mutex};

pub struct UserUpgradeService {
    user_repository: Arc<Mutex<dyn UserRepositoryTrait + Send + Sync>>,
}

pub struct UserUpgradeCommand {
    id: String,
}

impl UserUpgradeCommand {
    pub fn new(id: &str) -> Self {
        Self { id: id.to_string() }
    }
}

impl UserUpgradeService {
    pub fn new(user_repository: Arc<Mutex<dyn UserRepositoryTrait + Send + Sync>>) -> Self {
        Self { user_repository }
    }

    pub async fn handle(&self, command: UserUpgradeCommand) -> Result<()> {
        let target_id = UserId::new(&command.id)?;
        let repo = self.user_repository.lock().unwrap();

        let mut user = repo
            .find_by_id(&target_id)
            .await?
            .ok_or_else(|| anyhow!("Could not find the target user."))?;

        user.upgrade()?;

        repo.save(&user).await
    }
}
