#![allow(dead_code)]

use std::sync::Arc;

use application::user_application_service::UserApplicationService;
use interface::repository::{user_repository::UserRepository, user_database};
use infrastructure::database::postgres_user_repository::PostgresUserRepository;

use anyhow::Result;
use tokio::sync::Mutex;

mod application;
mod domain;
mod infrastructure;
mod interface;

struct Program {}

impl Program {
    fn new() -> Program {
        Program {}
    }

    async fn create_user(&self, user_name: &str) -> Result<()> {
        let user_database = PostgresUserRepository::new()?;
        let user_database = Arc::new(Mutex::new(user_database));
        let thread_arc = user_database.lock().await;

        let user_repository = UserRepository::new(Box::new(thread_arc)).await?;
        let application = UserApplicationService::new(Box::new(user_repository));
        application.register(user_name).await?;


        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("Start");
    let program = Program::new();
    program.create_user("TaroMan").await?;
    println!("End");
    Ok(())
}