use crate::repository::user_repository::UserRepositoryInterface;

use super::user::{User};

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
        self.repository.find(&name).await.is_some()
    }
}
