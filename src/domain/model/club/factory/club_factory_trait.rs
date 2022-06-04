use super::super::entity::{ ClubName, Club }; use anyhow::Result;

pub trait ClubFactoryTrait {
    fn create(&self, name: ClubName) -> Result<Club>;
}