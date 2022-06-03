use crate::domain::model::user::entity::{ UserName, UserId };
use crate::domain::model::user::service::UserService;
use crate::domain::repository::user_repository_trait::UserRepositoryTrait;

use anyhow::{Result, anyhow};
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
        let user = repo.find_by_id(&target_id).await;

        if user.is_none() {
            return Err(anyhow!("Could not find the target user."));
        }

        let mut user = user.unwrap();

        if let Some(name) = command.name {
            let new_user_name = UserName::new(&name)?;
            user.change_name(new_user_name)?;

            let user_service = UserService::new(&*repo);
            if user_service.exists(&user).await {
                return Err(anyhow!("User already exists"));
            }
        }

        repo.update(&user).await
    }
}
