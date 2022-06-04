use crate::domain::model::club::{
    entity::Club,
    repository::ClubRepositoryTrait,
};

pub struct ClubService<'a> {
    repository: &'a dyn ClubRepositoryTrait,
}

impl ClubService<'_> {
    pub fn new(repository: &dyn ClubRepositoryTrait) -> ClubService {
        ClubService {
            repository,
        }
    }

    pub async fn exists(&self, user: &Club) -> bool {
        let name = user.get_name();
        self.repository.find_by_name(name).await.is_some()
    }

    pub async fn exists_by_id(&self, user: &Club) -> bool {
        let id = user.get_id();
        self.repository.find_by_id(id).await.is_some()
    }
}
