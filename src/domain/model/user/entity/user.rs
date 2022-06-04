use anyhow::{Result};
use validator::{Validate};

use super::{UserId, UserName};

#[derive(Debug, Clone, Validate)]
pub struct User {
    #[validate]
    id: UserId,
    #[validate]
    name: UserName,
}

impl User {
    pub fn new(id: UserId, name: UserName) -> Result<Self> {
        let data = Self {
            id,
            name,
        };
        data.validate()?;
        Ok(data)
    }

    pub fn get_name(&self) -> &UserName {
        &self.name
    }

    pub fn get_id(&self) -> &UserId {
        &self.id
    }

    pub fn change_name(&mut self, name: UserName) -> Result<()> {
        self.name = name;
        self.validate()?;

        Ok(())
    }
}