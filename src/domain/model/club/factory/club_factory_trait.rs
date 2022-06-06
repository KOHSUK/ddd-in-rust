use super::super::entity::{Club, ClubName};
use crate::domain::model::user::entity::User;

use anyhow::Result;

pub trait ClubFactoryTrait {
    fn create(&self, name: ClubName, owner: User) -> Result<Club>;
}
