use crate::domain::model::user::{
    entity::{UserId, UserName},
    service::UserService,
    repository::UserRepositoryTrait,
};

use anyhow::{anyhow, Result};
use std::sync::{Arc, Mutex};

pub struct UserUpdateInfoService {
    user_repository: Arc<Mutex<dyn UserRepositoryTrait + Send + Sync>>,
}

pub struct UserUpdateCommand {
    id: String,
    name: Option<String>,
}

impl UserUpdateCommand {
    pub fn new(id: &str, name: Option<&str>) -> Self {
        Self {
            id: id.to_string(),
            name: name.map(|x| x.to_string()),
        }
    }
}

impl UserUpdateInfoService {
    pub fn new(user_repository: Arc<Mutex<dyn UserRepositoryTrait + Send + Sync>>) -> Self {
        Self { user_repository }
    }

    pub async fn handle(&self, command: UserUpdateCommand) -> Result<()> {
        let target_id = UserId::new(&command.id)?;
        let repo = self.user_repository.lock().unwrap();

        let mut user = repo
            .find_by_id(&target_id)
            .await
            .ok_or_else(|| anyhow!("Could not find the target user."))?;

        if let Some(name) = command.name {
            let new_user_name = UserName::new(&name)?;
            user.change_name(new_user_name)?;

            let user_service = UserService::new(&*repo);
            if user_service.exists(&user).await {
                return Err(anyhow!("User already exists"));
            }
        }

        repo.save(&user).await
    }
}
