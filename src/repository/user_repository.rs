use crate::domain::user::{User, UserName};
use crate::infrastructure::database::CONFIG;

use sqlx::{self, postgres};


pub trait UserRepositoryInterface {
    fn save(&self, user: &User);
    fn find(&self, user_name: &UserName) -> Option<User>;
}

pub struct UserRepository {}

impl UserRepositoryInterface for UserRepository {
    fn save(&self, user: &User) {
        println!("Saved {}", user.get_name().to_str());
    }

    fn find(&self, user_name: &UserName) -> Option<User> {
        // 本当はここでデータベースにアクセスする

        // match User::new(user_name.clone()) {
        //     Ok(user) => Some(user),
        //     Err(_) => None,
        // }

        None
    }
}

impl UserRepository {
    pub async fn new() -> anyhow::Result<UserRepository> {
        let pool = postgres::PgPoolOptions::new()
            .max_connections(20)
            .connect(&CONFIG.database_url())
            .await?;

        let repo = UserRepository {};
        Ok(repo)
    }
}