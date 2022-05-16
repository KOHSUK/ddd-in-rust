use domain::user::{User, UserName};
use domain::user_service::UserService;
use repository::user_repository::{UserRepositoryInterface, UserRepository};
use anyhow::{ Result, anyhow };


mod domain;
mod repository;
mod infrastructure;

struct Program<'a> {
    user_service: UserService<'a>,
    user_repository: &'a dyn UserRepositoryInterface,
}

impl Program<'_> {
    fn new(repository: &dyn UserRepositoryInterface) -> Program {
        Program {
            user_service: UserService::new(repository),
            user_repository: repository,
        }
    }

    fn create_user(&self, user_name: &str) -> Result<()> {
        let user_name = UserName::new(user_name)?;
        let user = User::new(user_name)?;
 
        if self.user_service.exists(&user) {
            return Err(anyhow!("User already exists."));
        }

        println!("start saving...");

        self.user_repository.save(&user);

        println!("saved!");

        Ok(())
    }
}

fn main() -> Result<()> {
    println!("Start");
    let user_repository = UserRepository::new();
    let program = Program::new(&user_repository);
    program.create_user("TaroMan")?;
    println!("End");
    Ok(())
}