use crate::domain::model::club::{entity::Club, repository::ClubRepositoryTrait};

pub struct ClubService<'a> {
    repository: &'a dyn ClubRepositoryTrait,
}

impl ClubService<'_> {
    pub fn new(repository: &dyn ClubRepositoryTrait) -> ClubService {
        ClubService { repository }
    }

    pub async fn exists(&self, club: &Club) -> bool {
        let name = club.get_name();
        match self.repository.find_by_name(name).await {
            Ok(maybe_user) => maybe_user.is_some(),
            Err(_) => false,
        }
    }
}
