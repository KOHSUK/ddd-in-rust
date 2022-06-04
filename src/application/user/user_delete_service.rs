use std::sync::{Arc, Mutex};

use crate::domain::model::user::{
    entity::UserId,
    repository::UserRepositoryTrait,
};

use anyhow::{Result};

pub struct UserDeleteService {
    user_repository: Arc<Mutex<dyn UserRepositoryTrait + Send + Sync>>,
}

pub struct UserDeleteCommand {
    id: String,
}

impl UserDeleteCommand {
    pub fn new(id: &str) -> Self {
        Self { id: id.to_string() }
    }
}

impl UserDeleteService {
    pub fn new(user_repository: Arc<Mutex<dyn UserRepositoryTrait + Send + Sync>>) -> Self {
        Self { user_repository }
    }

    pub async fn handle(&self, command: UserDeleteCommand) -> Result<()> {
        let id = UserId::new(&command.id)?;
        let repo = self.user_repository.lock().unwrap();
        match repo.find_by_id(&id).await {
            Some(_) => repo.delete(&id).await,
            None => Ok(())
        }
    }
}
