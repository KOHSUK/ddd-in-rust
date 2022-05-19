use crate::domain::entity::user::{User, UserId, UserName};
use crate::domain::entity::user_service::{UserService};
use crate::domain::repository::user_repository::UserRepositoryInterface;

use anyhow::{Result, anyhow};

pub struct UserApplicationService {
    user_repository: Box<dyn UserRepositoryInterface>,
}

pub struct UserData {
    id: String,
    name: String,
}

impl UserData {
    pub fn new(user: &User) -> Self {
        let id = user.get_id();
        let name = user.get_name();

        Self {
            id: id.to_str().to_string(),
            name: name.to_str().to_string(),
        }
    }
    pub fn get_id(&self) -> String {
        self.id.to_string()
    }
    pub fn get_name(&self) -> String {
        self.name.to_string()
    }
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

pub struct UserDeleteCommand {
    id: String,
}

impl UserDeleteCommand {
    pub fn new(id: &str) -> Self {
        Self { id: id.to_string() }
    }
}

impl UserApplicationService {
    pub fn new(user_repository: Box<dyn UserRepositoryInterface>) -> Self {
        Self { user_repository }
    }

    pub async fn get(&self, user_id: &str) -> Option<UserData> {
        let target_id = UserId::new(user_id);
        if target_id.is_err() {
            return None;
        }

        let user = self.user_repository.find_by_id(&target_id.unwrap()).await?;

        Some(UserData::new(&user))
    }

    pub async fn register(&self, name: &str) -> Result<()> {
        let name = UserName::new(name)?;
        let user = User::new(name)?;

        let user_service = UserService::new(&*self.user_repository);
        if user_service.exists_by_name(&user).await {
            return Err(anyhow!("User already exists"));
        }

        self.user_repository.save(&user).await
    }

    pub async fn update(&self, command: UserUpdateCommand) -> Result<()> {
        if let (Some(name), id) = (command.name, command.id) {
            let user_service = UserService::new(&*self.user_repository);
            let name = UserName::new(&name)?;
            let id = UserId::new(&id)?;

            let user = User::new_with_id(id.clone(), name.clone())?;
            if !user_service.exists_by_id(&user).await {
                return Err(anyhow!("Could not find the target user."));
            }

            self.user_repository.save(&user).await
        } else {
            Err(anyhow!("Could not update."))
        }
    }

    pub async fn delete(&self, command: UserDeleteCommand) -> Result<()> {
        let id = UserId::new(&command.id)?;
        // TODO: works but tricky...
        let name = UserName::new("")?;
        let user = User::new_with_id(id, name)?;

        let user_service = UserService::new(&*self.user_repository);
        if !user_service.exists_by_id(&user).await {
            return Ok(())
        }

        self.user_repository.delete(&user).await
    }
}
