use anyhow::{anyhow, Result};
use validator::Validate;

use crate::domain::model::user::entity::{User, UserId};

use super::{ClubId, ClubName};

#[derive(Debug, Clone, Validate)]
pub struct Club {
    #[validate]
    id: ClubId,
    #[validate]
    name: ClubName,
    members: Vec<UserId>,
    #[validate]
    owner: UserId,
}

impl Club {
    pub fn new(id: ClubId, name: ClubName, members: Vec<UserId>, owner: UserId) -> Result<Self> {
        let data = Self {
            id,
            name,
            members,
            owner,
        };
        data.validate()?;
        Ok(data)
    }

    pub fn get_name(&self) -> &ClubName {
        &self.name
    }

    pub fn get_id(&self) -> &ClubId {
        &self.id
    }

    pub fn get_owner_id(&self) -> &UserId {
        &self.owner
    }

    pub fn get_members(&self) -> &Vec<UserId> {
        &self.members
    }

    pub fn change_name(&mut self, name: ClubName) -> Result<()> {
        self.name = name;
        self.validate()?;

        Ok(())
    }

    pub fn count_members(&self) -> usize {
        let owner_num = 1;
        self.members.len() + owner_num
    }

    pub fn is_full(&self) -> bool {
        self.count_members() >= 30
    }

    pub fn join(&mut self, user: User) -> Result<()> {
        if self.is_full() {
            return Err(anyhow!(
                "Club members have already reached the upper limmit."
            ));
        }

        self.members.push(user.get_id().to_owned());

        Ok(())
    }
}
