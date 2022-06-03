mod user_factory;

pub use user_factory::UserFactory;

use super::entity::{ User, UserName };
use anyhow::Result;

pub trait UserFactoryInterface {
    fn create(&self, name: UserName) -> Result<User>;
}