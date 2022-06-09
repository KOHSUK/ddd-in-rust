use crate::domain::model::user::{entity::UserId, repository::UserRepositoryTrait};

use anyhow::{anyhow, Result};
use std::sync::{Arc, Mutex};

pub struct UserDowngradeService {
    user_repository: Arc<Mutex<dyn UserRepositoryTrait + Send + Sync>>,
}

pub struct UserDowngradeCommand {
    id: String,
}

impl UserDowngradeCommand {
    pub fn new(id: &str) -> Self {
        Self { id: id.to_string() }
    }
}

impl UserDowngradeService {
    pub fn new(user_repository: Arc<Mutex<dyn UserRepositoryTrait + Send + Sync>>) -> Self {
        Self { user_repository }
    }

    pub async fn handle(&self, command: UserDowngradeCommand) -> Result<()> {
        let target_id = UserId::new(&command.id)?;
        let repo = self.user_repository.lock().unwrap();

        let mut user = repo
            .find_by_id(&target_id)
            .await?
            .ok_or_else(|| anyhow!("Could not find the target user."))?;

        user.downgrade()?;

        repo.save(&user).await
    }
}
