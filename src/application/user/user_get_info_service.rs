use std::sync::{Arc, Mutex};

use crate::domain::entity::user::{User, UserId};
use crate::domain::repository::user_repository::UserRepositoryInterface;

pub struct UserGetInfoService {
    user_repository: Arc<Mutex<dyn UserRepositoryInterface + Send + Sync>>,
}

#[derive(Debug)]
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

impl UserGetInfoService {
    pub fn new(user_repository: Arc<Mutex<dyn UserRepositoryInterface + Send + Sync>>) -> Self {
        Self { user_repository }
    }

    pub async fn handle(&self, user_id: &str) -> Option<UserData> {
        let target_id = UserId::new(user_id);
        if target_id.is_err() {
            return None;
        }

        let repo = self.user_repository.lock().unwrap();
        let user = repo.find_by_id(&target_id.unwrap()).await?;

        Some(UserData::new(&user))
    }
}
