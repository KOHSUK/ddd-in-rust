use crate::domain::model::user::{
    entity::UserName,
    service::UserService,
    factory::UserFactoryTrait,
    repository::UserRepositoryTrait,
};

use anyhow::{Result, anyhow};
use std::sync::{Arc, Mutex};

pub struct UserRegisterService {
    user_repository: Arc<Mutex<dyn UserRepositoryTrait + Send + Sync>>,
    user_factory: Arc<Mutex<dyn UserFactoryTrait>>,
}

impl UserRegisterService {
    pub fn new(
        user_repository: Arc<Mutex<dyn UserRepositoryTrait + Send + Sync>>,
        user_factory: Arc<Mutex<dyn UserFactoryTrait>>,
    ) -> Self {
        Self { user_repository, user_factory }
    }

    pub async fn handle(&self, name: &str) -> Result<()> {
        let name = UserName::new(name)?;
        let factory = Arc::clone(&self.user_factory);
        let user = factory.lock().unwrap().create(name)?;

        let repo = self.user_repository.lock().unwrap();
        let user_service = UserService::new(&*repo);
        if user_service.exists(&user).await {
            return Err(anyhow!("User already exists"));
        }

        repo.save(&user).await
    }
}

#[cfg(test)]
mod test {
    use std::sync::{Arc, Mutex};

    use crate::domain::model::user::{
        entity::UserName,
        factory::UserFactory,
        repository::UserRepositoryTrait,
    };
    use crate::infrastructure::database::user::InMemoryUserDatabase;
    use crate::interface::repository::user::UserRepository;

    use super::UserRegisterService;

    #[tokio::test]
    async fn can_register_min_user_name() {
        let user_database = InMemoryUserDatabase::new();
        let user_repository =
            UserRepository::new(Box::new(user_database))
            .await
            .map(|repo| Arc::new(Mutex::new(repo)))
            .unwrap();
        let registry_repository = Arc::clone(&user_repository);
        let user_factory = Arc::new(Mutex::new(UserFactory::new()));
        let user_register_service = UserRegisterService::new(registry_repository, user_factory);

        let min_name = "abc";
        user_register_service.handle(min_name).await.unwrap();

        let read_repository = Arc::clone(&user_repository);
        let target_name = UserName::new(min_name).unwrap();
        let target = read_repository.lock().unwrap().find_by_name(&target_name).await;

        assert!(target.is_some());
    }

    #[tokio::test]
    async fn cannot_register_name_shorter_than_min_length() {
        let user_database = InMemoryUserDatabase::new();
        let user_repository =
            UserRepository::new(Box::new(user_database))
            .await
            .map(|repo| Arc::new(Mutex::new(repo)))
            .unwrap();
        let registry_repository = Arc::clone(&user_repository);
        let user_factory = Arc::new(Mutex::new(UserFactory::new()));
        let user_register_service = UserRegisterService::new(registry_repository, user_factory);

        let short_name = "ab";
        let error_msg = match user_register_service.handle(short_name).await {
            Ok(_) => panic!(),
            Err(e) => e.to_string(),
        };

        assert_eq!(&error_msg, "The length of a user name must be greater than 3");
    }

    #[tokio::test]
    async fn cannot_register_dupulicate_name() {
        let user_database = InMemoryUserDatabase::new();
        let user_repository =
            UserRepository::new(Box::new(user_database))
            .await
            .map(|repo| Arc::new(Mutex::new(repo)))
            .unwrap();
        let registry_repository = Arc::clone(&user_repository);
        let user_factory = Arc::new(Mutex::new(UserFactory::new()));
        let user_register_service = UserRegisterService::new(registry_repository, user_factory);

        let min_name = "duplicate";
        user_register_service.handle(min_name).await.unwrap();

        let duplicate_name = min_name;
        let res = user_register_service.handle(duplicate_name).await;

        let error_msg = match res {
            Ok(_) => panic!(),
            Err(e) => e.to_string(),
        };

        assert_eq!(&error_msg, "User already exists");
    }
}