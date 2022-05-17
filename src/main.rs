#![allow(dead_code)]

use domain::entity::user::{User, UserName};
use domain::entity::user_service::UserService;
use domain::repository::user_repository::{UserRepositoryInterface, UserRepository};
use anyhow::{ Result, anyhow };


mod domain;
mod infrastructure;

struct Program {
    user_repository: Box<dyn UserRepositoryInterface>,
}

impl Program {
    fn new(repository: Box<dyn UserRepositoryInterface>) -> Program {
        Program {
            user_repository: repository,
        }
    }

    async fn create_user(&self, user_name: &str) -> Result<()> {
        let user_name = UserName::new(user_name)?;
        let user = User::new(user_name)?;
        let user_service = UserService::new(&*self.user_repository);
 
        if user_service.exists(&user).await {
            return Err(anyhow!("User already exists."));
        }

        println!("start saving...");

        match self.user_repository.save(&user).await {
            Ok(_) => println!("saved!"),
            Err(err) => println!("Unable to save: {:?}", err),
        }

        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("Start");
    let user_repository = UserRepository::new().await?;
    let program = Program::new(Box::new(user_repository));
    program.create_user("TaroMan").await?;
    println!("End");
    Ok(())
}