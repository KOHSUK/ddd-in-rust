use clap::{ Parser, ArgEnum };
use anyhow::{ Result, anyhow };

use crate::application::user::user_delete_service::{UserDeleteService, UserDeleteCommand};
use crate::application::user::user_get_info_service::UserGetInfoService;
use crate::application::user::user_register_service::UserRegisterService;
use crate::application::user::user_update_info_service::{UserUpdateInfoService, UserUpdateCommand};
use crate::interface::repository::{user_repository::UserRepository};
use crate::infrastructure::database::postgres_user_repository::PostgresUserRepository;

pub struct CommandLine {
    args: Args,
}

impl CommandLine {
    pub fn new() -> Self {
        let args = Args::parse();

        Self { args }
    }    

    pub async fn start(&self) -> Result<()> {
        match self.args.operation {
            Operation::Create => {
                self.create().await
            },
            Operation::Read => {
                self.read().await
            },
            Operation::Update => {
                self.update().await
            },
            Operation::Delete => {
                self.delete().await
            },
        }
    }

    async fn create(&self) -> Result<()> {
        let user_database = PostgresUserRepository::new()?;
        let user_repository = UserRepository::new(Box::new(user_database)).await?;
        let service = UserRegisterService::new(Box::new(user_repository));
        if let Some(name) = self.args.name.as_ref() {
            service.handle(name).await
        } else {
            Err(anyhow!("`name` option is required."))
        }
    }

    async fn read(&self) -> Result<()> {
        let user_database = PostgresUserRepository::new()?;
        let user_repository = UserRepository::new(Box::new(user_database)).await?;
        let service = UserGetInfoService::new(Box::new(user_repository));
        if let Some(id) = self.args.id.as_ref() {
            println!("id: {}", id);
            if let Some(user) = service.handle(id).await {
                println!("{:?}", user);
            } else {
                println!("Could not find user.");
            }
            Ok(())
        } else {
            Err(anyhow!("`id` option is required."))
        }
    }

    async fn update(&self) -> Result<()> {
        let user_database = PostgresUserRepository::new()?;
        let user_repository = UserRepository::new(Box::new(user_database)).await?;
        let service = UserUpdateInfoService::new(Box::new(user_repository));
        if let Some(id) = self.args.id.as_ref() {
            if let Some(name) = self.args.name.as_ref() {
                let command = UserUpdateCommand::new(id, Some(name));
                service.handle(command).await
            } else {
                Err(anyhow!("`id` option id required."))
            }
        } else {
            Err(anyhow!("`name` option id required."))
        }
    }

    async fn delete(&self) -> Result<()> {
        let user_database = PostgresUserRepository::new()?;
        let user_repository = UserRepository::new(Box::new(user_database)).await?;
        let service = UserDeleteService::new(Box::new(user_repository));
        if let Some(id) = self.args.id.as_ref() {
            let command = UserDeleteCommand::new(id);
            service.handle(command).await
        } else {
            Err(anyhow!("`id` option is required."))
        }
    }
}

#[derive(Parser, Debug)]
#[clap(
    name = "DDD in Rust",
    author = "Kohsuk",
    version = "v0.0.1",
    about = "Example Command Line Application using DDD in Rust",
    long_about = None
)]
pub struct Args {
    #[clap(arg_enum, short, long)]
    operation: Operation,
    #[clap(short, long)]
    name: Option<String>,
    #[clap(short, long)]
    id: Option<String>,
}

#[derive(ArgEnum, Clone, Debug)]
enum Operation {
    Create,
    Read,
    Update,
    Delete,
}
