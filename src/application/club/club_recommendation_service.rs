use crate::domain::model::club::{
    entity::Club, repository::ClubRepositoryTrait, specifications::ClubRecommendationSpec,
};

use anyhow::Result;
use std::sync::{Arc, Mutex};

pub struct ClubRecommendationService {
    club_repository: Arc<Mutex<dyn ClubRepositoryTrait>>,
}

pub struct ClubRecommendation {
    pub club_id: String,
    pub club_name: String,
    pub owner: String,
}

impl ClubRecommendation {
    fn new(club: &Club) -> Self {
        Self {
            club_id: club.get_id().to_string(),
            club_name: club.get_name().to_string(),
            owner: club.get_owner_id().to_string(),
        }
    }
}

impl ClubRecommendationService {
    pub fn new(club_repository: Arc<Mutex<dyn ClubRepositoryTrait>>) -> Self {
        Self { club_repository }
    }

    pub async fn handle(&self) -> Result<Vec<ClubRecommendation>> {
        let spec = ClubRecommendationSpec::new();

        let repo = Arc::clone(&self.club_repository);
        let repo = repo.lock().unwrap();
        let clubs = repo.find_all().await?;
        Ok(clubs
            .into_iter()
            .filter(|c| spec.is_satisfied_by(c))
            .map(|c| ClubRecommendation::new(&c))
            .take(10)
            .collect())
    }
}
