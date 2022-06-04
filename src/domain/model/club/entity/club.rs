use anyhow::{Result};
use validator::{Validate};

use super::{ClubId, ClubName};

#[derive(Debug, Clone, Validate)]
pub struct Club {
    #[validate]
    id: ClubId,
    name: ClubName,
}

impl Club {
    pub fn new(id: ClubId, name: ClubName) -> Result<Self> {
        let data = Self {
            id,
            name,
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

    pub fn change_name(&mut self, name: ClubName) -> Result<()> {
        self.name = name;
        self.validate()?;

        Ok(())
    }
}