use super::ClubFactoryTrait;
use crate::domain::model::{
    club::entity::{Club, ClubId, ClubName},
    user::entity::User,
};

use anyhow::Result;

pub struct ClubFactory {}

impl ClubFactory {
    pub fn new() -> Self {
        Self {}
    }
}

impl ClubFactoryTrait for ClubFactory {
    fn create(&self, name: ClubName, owner: User) -> Result<Club> {
        let id = uuid::Uuid::new_v4().to_string();
        let id = ClubId::new(&id).unwrap();
        Club::new(id, name, Vec::new(), owner.get_id().clone())
    }
}
