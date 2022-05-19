use crate::domain::entity::user::UserId;
use crate::domain::repository::user_repository::UserRepositoryInterface;

use anyhow::{Result};

pub struct UserDeleteService {
    user_repository: Box<dyn UserRepositoryInterface>,
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
    pub fn new(user_repository: Box<dyn UserRepositoryInterface>) -> Self {
        Self { user_repository }
    }

    pub async fn handle(&self, command: UserDeleteCommand) -> Result<()> {
        let user_id = UserId::new(&command.id)?;

        match self.user_repository.find_by_id(&user_id).await {
            Some(u) => self.user_repository.delete(&u).await,
            None => Ok(())
        }
    }
}
