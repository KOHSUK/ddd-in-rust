use super::UserFactoryInterface;
use crate::domain::model::user::entity::{ UserId, UserName, User };

use anyhow::Result;

pub struct UserFactory {}

impl UserFactory {
    pub fn new() -> Self {
        Self {}
    }
}

impl UserFactoryInterface for UserFactory {
    fn create(&self, name: UserName) -> Result<User> {
        let id = uuid::Uuid::new_v4().to_string();
        let id = UserId::new(&id).unwrap();
        User::new(id, name)
    }
}