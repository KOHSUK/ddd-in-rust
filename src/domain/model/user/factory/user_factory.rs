use super::UserFactoryTrait;
use crate::domain::model::user::entity::{User, UserId, UserIsPremium, UserName};

use anyhow::Result;

pub struct UserFactory {}

impl UserFactory {
    pub fn new() -> Self {
        Self {}
    }
}

impl UserFactoryTrait for UserFactory {
    fn create(&self, name: UserName) -> Result<User> {
        let id = uuid::Uuid::new_v4().to_string();
        let id = UserId::new(&id)?;
        let is_premium = UserIsPremium::new(false);
        User::new(id, name, is_premium)
    }
}
