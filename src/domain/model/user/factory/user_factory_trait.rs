use super::super::entity::{ User, UserName };
use anyhow::Result;

pub trait UserFactoryTrait {
    fn create(&self, name: UserName) -> Result<User>;
}