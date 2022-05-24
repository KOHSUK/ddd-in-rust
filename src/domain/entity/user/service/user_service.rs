use crate::domain::repository::user_repository::UserRepositoryInterface;
use crate::domain::entity::user::model::User;

pub struct UserService<'a> {
    repository: &'a dyn UserRepositoryInterface,
}

impl UserService<'_> {
    pub fn new(repository: &dyn UserRepositoryInterface) -> UserService {
        UserService {
            repository,
        }
    }

    pub async fn exists(&self, user: &User) -> bool {
        let name = user.get_name();
        self.repository.find_by_name(&name).await.is_some()
    }

    pub async fn exists_by_id(&self, user: &User) -> bool {
        let id = user.get_id();
        self.repository.find_by_id(&id).await.is_some()
    }
}