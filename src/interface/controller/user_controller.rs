use std::sync::{Arc, Mutex};

use anyhow::{Result};

use crate::application::user::user_delete_service::UserDeleteCommand;
use crate::application::user::user_update_info_service::UserUpdateCommand;
use crate::application::user::{
    user_delete_service::UserDeleteService, user_get_info_service::UserGetInfoService,
    user_register_service::UserRegisterService, user_update_info_service::UserUpdateInfoService,
};
// use crate::infrastructure::database::in_memory_user_repository::InMemoryUserRepository;
use crate::infrastructure::database::postgres_user_repository::PostgresUserRepository;
use crate::interface::repository::user_repository::UserRepository;

pub struct UserController {
    user_delete_service: UserDeleteService,
    user_get_info_service: UserGetInfoService,
    user_register_service: UserRegisterService,
    user_update_info_service: UserUpdateInfoService,
}

pub struct PostArgs {
    pub name: String,
}

pub struct DeleteArgs {
    pub id: String,
}

pub struct GetArgs {
    pub id: String,
}

#[derive(Debug)]
pub struct GetResult {
    pub id: String,
    pub name: String,
}

pub struct PutArgs {
    pub id: String,
    pub name: String,
}

impl UserController {
    pub async fn new() -> Result<Self> {
        // let user_database = InMemoryUserRepository::new();
        let user_database = PostgresUserRepository::new()?;
        let user_repository = UserRepository::new(Box::new(user_database)).await?;
        let user_repository = Arc::new(Mutex::new(user_repository));

        let deletion_repository = Arc::clone(&user_repository);
        let user_delete_service = UserDeleteService::new(deletion_repository);

        let read_repository = Arc::clone(&user_repository);
        let user_get_info_service = UserGetInfoService::new(read_repository);

        let registry_repository = Arc::clone(&user_repository);
        let user_register_service = UserRegisterService::new(registry_repository);

        let update_repository = Arc::clone(&user_repository);
        let user_update_info_service = UserUpdateInfoService::new(update_repository);

        Ok(Self {
            user_delete_service,
            user_get_info_service,
            user_register_service,
            user_update_info_service,
        })
    }

    pub async fn post(&self, args: PostArgs) -> Result<()> {
        self.user_register_service.handle(&args.name).await
    }

    pub async fn delete(&self, args: DeleteArgs) -> Result<()> {
        let command = UserDeleteCommand::new(&args.id);
        self.user_delete_service.handle(command).await
    }

    pub async fn get(&self, args: GetArgs) -> Option<GetResult> {
        self.user_get_info_service
            .handle(&args.id)
            .await.map(|u| GetResult {
                    id: u.get_id(),
                    name: u.get_name(),
                })
    }

    pub async fn put(&self, args: PutArgs) -> Result<()> {
        let command = UserUpdateCommand::new(&args.id, Some(&args.name));
        self.user_update_info_service.handle(command).await
    }
}
