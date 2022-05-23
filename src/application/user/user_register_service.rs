use crate::domain::entity::user::{User, UserName};
use crate::domain::entity::user_service::{UserService};
use crate::domain::repository::user_repository::UserRepositoryInterface;

use anyhow::{Result, anyhow};
use std::sync::{Arc, Mutex};

pub struct UserRegisterService {
    user_repository: Arc<Mutex<dyn UserRepositoryInterface + Send + Sync>>,
}

impl UserRegisterService {
    pub fn new(user_repository: Arc<Mutex<dyn UserRepositoryInterface + Send + Sync>>) -> Self {
        Self { user_repository }
    }

    pub async fn handle(&self, name: &str) -> Result<()> {
        let name = UserName::new(name)?;
        let user = User::new(name)?;

        let repo = self.user_repository.lock().unwrap();
        let user_service = UserService::new(&*repo);
        if user_service.exists(&user).await {
            return Err(anyhow!("User already exists"));
        }

        repo.save(&user).await
    }
}
