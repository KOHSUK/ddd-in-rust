use crate::domain::model::club::{entity::Club, repository::ClubRepositoryTrait};

use std::sync::{Arc, Mutex};

pub struct ClubService {
    repository: Arc<Mutex<dyn ClubRepositoryTrait>>,
}

impl ClubService {
    pub fn new(repository: Arc<Mutex<dyn ClubRepositoryTrait>>) -> ClubService {
        ClubService { repository }
    }

    pub async fn exists(&self, club: &Club) -> bool {
        let name = club.get_name();
        let repository = self.repository.lock().unwrap();
        match repository.find_by_name(name).await {
            Ok(maybe_user) => maybe_user.is_some(),
            Err(_) => false,
        }
    }
}
