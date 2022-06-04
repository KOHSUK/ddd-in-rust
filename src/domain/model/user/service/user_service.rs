use crate::domain::model::user::{
    entity::User,
    repository::UserRepositoryTrait,
};

pub struct UserService<'a> {
    repository: &'a dyn UserRepositoryTrait,
}

impl UserService<'_> {
    pub fn new(repository: &dyn UserRepositoryTrait) -> UserService {
        UserService {
            repository,
        }
    }

    pub async fn exists(&self, user: &User) -> bool {
        let name = user.get_name();
        self.repository.find_by_name(name).await.is_some()
    }

    pub async fn exists_by_id(&self, user: &User) -> bool {
        let id = user.get_id();
        self.repository.find_by_id(id).await.is_some()
    }
}
