use anyhow::Result;
use std::sync::{Arc, Mutex};

use crate::domain::model::user::{
    entity::{User, UserId},
    repository::UserRepositoryTrait,
};

pub struct UserGetInfoService {
    user_repository: Arc<Mutex<dyn UserRepositoryTrait + Send + Sync>>,
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
            id: id.to_string(),
            name: name.to_string(),
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
    pub fn new(user_repository: Arc<Mutex<dyn UserRepositoryTrait + Send + Sync>>) -> Self {
        Self { user_repository }
    }

    pub async fn handle(&self, user_id: &str) -> Result<Option<UserData>> {
        let target_id = UserId::new(user_id)?;

        let repo = self.user_repository.lock().unwrap();
        repo.find_by_id(&target_id)
            .await
            .map(|x| x.map(|user| UserData::new(&user)))
    }
}
