use crate::domain::entity::user::{User, UserName};
use crate::domain::entity::user_service::{UserService};
use crate::domain::repository::user_repository::UserRepositoryInterface;

use anyhow::{Result, anyhow};

pub struct UserRegisterService {
    user_repository: Box<dyn UserRepositoryInterface>,
}

impl UserRegisterService {
    pub fn new(user_repository: Box<dyn UserRepositoryInterface>) -> Self {
        Self { user_repository }
    }

    pub async fn handle(&self, name: &str) -> Result<()> {
        let name = UserName::new(name)?;
        let user = User::new(name)?;

        let user_service = UserService::new(&*self.user_repository);
        if user_service.exists(&user).await {
            return Err(anyhow!("User already exists"));
        }

        self.user_repository.save(&user).await
    }
}
